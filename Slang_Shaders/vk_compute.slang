#language slang legacy

#include "gm_utils.slang"

// Used for Greedy Meshing
static const uint GRID_SIZE_X = 64;
static const uint GRID_SIZE_Y = 64;
// Merges GRID_SIZE_X and GRID_SIZE_Y
static const uint TOTAL_SIZE = GRID_SIZE_X * GRID_SIZE_Y;
// Merged Quad Data
struct Quad {
    int value; // Identifier
    int2 start; // Top-left Cell Coord
    int2 size;  // Dimensions of Quad
};

// Occupancy Grid: Asumed to be a 2D Texture
readonly Texture2D<int> cells : register(t0);
// Output Buffer for Merged Quads (SSBO)
RWStructuredBuffer<Quad> quads : register(u0);

// Source Image (2D texture)
readonly Texture2D<float4> InTex : register(t1);
// Read Write Texture for writing to the Screen
RWTexture2D<float4> OutTex : register(u1);

// Global Atomic Counter
RWByteAddressBuffer QuadCounter : register(u2);

// Adding Screen Resizing for the Fun of it
// Holds Scale Factor + If Bilinear Interpolation is going to be used
cbuffer ResizeParams : register(b0) {
    float2 params; // x: scale factor, y: useBilinear (0.0 or 1.0)
};

// Group Shared Variable sotring the Input Textures Dimensions
// This is for Accessing Values without Querying the Texture
groupshared uint2 dims[2];
groupshared uint mergedMask[(64 * 64 + 31) / 32];

[numthreads(16, 16, 1)]
void greedyMeshing(
    uint3 DTid : SV_DispatchThreadID,
    uint3 GTid : SV_GroupThreadID,
    uint3 GIndex : SV_GroupID
) {
    // Load Dims, Reset Atomics, and Clears the mergedMask 1D Array
    if (all(GTid.xy == 0 && GIndex.xy == 0)) {
        InTex.GetDimensions(dims[0].x, dims[0].y);
        QuadCounter.Store(0, 0);
        gm_utils::clearMerged(mergedMask);
    }
    // Shares memory with the other threads
    GroupMemoryBarrierWithGroupSync();
    // computes thread's "stride"
    const uint localIndex = GTid.y * 16 + GTid.x; // Range between 0 to 256
    const uint threadsPerRow = 16; 
    const uint groupSize = threadsPerRow * threadsPerRow;  // Number of Threads
    const uint DispatchDimX = (dims[0].x + 16 - 1) / 16; // Number of Groups launched on the X Axis
    const uint DispatchDimY = (dims[0].y + 16 - 1) / 16; // Number of Groups launched on the Y Axis
    const uint DispatchCount = DispatchDimX * DispatchDimY;
    const uint globalOffset = (GIndex.y * DispatchDimX + GIndex.x) * groupSize;
    uint idx = globalOffset + localIndex;
    uint stride = groupSize * DispatchCount;
    // Each Thread goes along its own interleaved chunk of cells
    for (; idx < TOTAL_SIZE; idx += stride) {
        uint x = idx & (GRID_SIZE_X - 1);
        uint y = idx >> 6;
        // Skip if already merged
        if (gm_utils::isMerged(x, y, gm_utils::ptrConverterUint(mergedMask))) continue;
        // load cell value
        int cellValue = cells.Load(int3(x, y, 0));
        // Skip if the Texture is Empty
        if (cellValue == 0) {
            gm_utils::setMerged(x, y, gm_utils::ptrConverterUint(mergedMask));
            continue;
        }
        uint cacheLineSize = 16; // 16-byte cache lines (assumed!)
        // Determine Quad Width & Height
        uint quadWidth = 1;
        uint quadHeight = 1;
        bool validRow = true;
        uint startX = x & ~(cacheLineSize - 1); // Align to Cache Line
        uint endX = min(x + threadsPerRow, (x & ~(cacheLineSize - 1)) + cacheLineSize);
        for (uint tx = startX; tx < endX; ++tx) {
            if (gm_utils::isMerged(tx, y, gm_utils::ptrConverterUint(mergedMask))) break;
            int tempVal = cells.Load(int3(tx, y, 0));
            if (tempVal != cellValue) break;
            if (GIndex.x == 0 && GIndex.y == 0 && GTid.x == 0 && GTid.y == 0) {
                quadWidth++;
            }
        }
        uint startY = y & ~(cacheLineSize - 1);
        uint endY = min(y + quadHeight, (y & ~(cacheLineSize - 1)) + cacheLineSize);
        for (uint ty = startY; ty < endY && validRow; ++ty) {
            for (uint tx = x; tx < x + quadWidth; ++tx) {
                if (gm_utils::isMerged(tx, ty, gm_utils::ptrConverterUint(mergedMask))) {
                    validRow = false;
                    break;
                }
                int tempVal = cells.Load(int3(tx, ty, 0));
                if (tempVal != cellValue) {
                    validRow = false;
                    break;
                }
            }
            // If the row is valid, only 1 thread increments the value
            if (validRow) {
                if (all(GTid.xy == 0)) {
                    quadHeight++;
                }
            }
        }
        // Mark Cells as Merged
        for (uint ty = y; ty < y + quadHeight; ++ty) {
            for (uint tx = x; tx < x + quadWidth; ++tx) {
                gm_utils::setMerged(tx, ty, gm_utils::ptrConverterUint(mergedMask));
            }
        }
        // Assmeble && Output Merged Quad
        uint outIdx;
        Quad q;
        q.value = cellValue;
        q.start = int2(x, y);
        q.size = int2(quadWidth, quadHeight);
        QuadCounter.InterlockedAdd(
            0,
            1,
            outIdx
        );
        quads[outIdx] = q;
        
    }
}

void bipolation_fn(
    float useBilinear,
    float scaleFactor,
    uint3 dtid: SV_DispatchThreadID
) {
    // Prepares for Picture Scale Changing
    // Maps Coordinates using Multiplication by the Inverse Scale
    float invScale = 1.0 / scaleFactor;
    float2 srcCoord = dtid.xy * invScale;
    srcCoord = clamp(srcCoord, 0.0, float2(dims[0]) - 1.0);
    // Nearest neighbor sampling
    if (useBilinear == 0.0) {
        OutTex[dtid.xy] = InTex[dtid.xy];
        return;
    }
    // Bilinear Interpolation; Slower but looks better
    int2 ipos = int2(floor(srcCoord));
    float2 frac = srcCoord - float2(ipos);
    int2 texDim = dims[0];
    // Compute the Neighboring Pixel Indices Branchlessly.
    int x1 = min(ipos.x + 1, texDim.x - 1);
    int y1 = min(ipos.y + 1, texDim.y - 1);
    // Sample the 4 Fixels Needed
    float4 c00 = InTex[ipos];
    float4 c10 = InTex[int2(x1, ipos.y)];
    float4 c01 = InTex[int2(ipos.x, y1)];
    float4 c11 = InTex[int2(x1, y1)];
    // Perform Bilinear Interpolation
    float4 c0 = lerp(c00, c10, frac.x);
    float4 c1 = lerp(c01, c11, frac.x);

    OutTex[dtid.xy] = lerp(c0, c1, frac.y);
}

// Compute Shader
[shader("compute")]
// Thread Group of size 256
[numthreads(16, 16, 1)]
// Inputs:
//   - dtid: The Unique Output Coordinate
//   - gtid: Local Calculations
//   - ltID: The Local Thread Position
func computeMain(
    uint3 dtid: SV_DispatchThreadID,
    uint3 gtid : SV_GroupThreadID,
    uint3 ltID: SV_GroupThreadID,
    )->void {
    // Only 1 Thread per group gets the Texture Dimensions
    if (gtid.x == 0 && gtid.y == 0) {
        uint2 inputDim, outputDim;
        InTex.GetDimensions(inputDim.x, inputDim.y);
        OutTex.GetDimensions(outputDim.x, outputDim.y);
        dims[0] = inputDim;
        dims[1] = outputDim;
    }
    // Makes sure all Threads have the groupshared variables
    GroupMemoryBarrierWithGroupSync();
    // Checks if Coordinates are in Bounds ; Early Exit if yes
    if (dtid.x >= dims[1].x || dtid.y >= dims[1].y) return;
    greedyMeshing(
        dtid,
        gtid,
        ltID
    );

    // Get Parameters
    float scaleFactor = params.x;
    float useBilinear = params.y;
    // If scaleFactor is 1, simply display the picture
    if (scaleFactor == 1.0) {
        OutTex[dtid.xy] = InTex[dtid.xy];
        return;
    }
    bipolation_fn(
        useBilinear,
        scaleFactor,
        dtid,
    );
}