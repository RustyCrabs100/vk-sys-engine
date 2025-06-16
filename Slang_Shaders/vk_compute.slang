#language slang legacy

// Source Image (2D texture)
Texture2D<float4> InTex;
// Read Write Texture for writing to the Screen
RWTexture2D<float4> OutTex;

// Adding Screen Resizing for the Fun of it
// Holds Scale Factor + If Bilinear Interpolation is going to be used
cbuffer ResizeParams : register(b0) {
    float2 params; // x: scale factor, y: useBilinear (0.0 or 1.0)
}

// Group Shared Variable sotring the Input Textures Dimensions
// This is for Accessing Values without Querying the Texture
groupshared uint2 dims[2];


func bipolation_fn(
    float useBilinear,
    float scaleFactor,
    uint3 dtid: SV_DispatchThreadID
) -> void {
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