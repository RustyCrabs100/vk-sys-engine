#language slang legacy

// Source Image (2D texture)
Texture2D<float4> InTex;
// Read Write Texture for writing to the Screen
RWTexture2D<float4> OutTex;

// Adding Screen Resizing for the Fun of it
// Holds Scale Factor + If Bilinear Interpolation is going to be used
struct ResizeParameters {
    float scaleFactor;
    // True: Bilinear Interpolation
    // False: Nearest Neighbor
    bool useBilinear;
}
ConstantBuffer<ResizeParameters> ResizeParams;

// Group Shared Variable sotring the Input Textures Dimensions
// This is for Accessing Values without Querying the Texture
groupshared uint2 inputDims;

// Compute Shader
[shader("compute")]
// Thread Group of size 256
[numthreads(16, 16, 1)]
// Inputs:
//   - dtid: The Unique Output Coordinate
//   - ltID: The Local Thread Position
func computeMain(
    uint2 dtid: SV_DispatchThreadID,
    uint2 ltID: SV_GroupThreadID,
    )->void {
    // Queries the Texture ONCE for speedy access later
    uint2 dims = uint2(0, 0);
    OutTex.GetDimensions(dims.x, dims.y);
    // Checks if Coordinates are in Bounds
    if (dtid.x >= dims.x || dtid.y >= dims.y) {
        return;
    }
    if (ltID.x == 0 && ltID.y == 0) {
        InTex.GetDimensions(inputDims.x, inputDims.y);
    }
    // Makes sure all Threads have the groupshared variables
    GroupMemoryBarrierWithGroupSync();
    // If scaleFactor is 1, simply display the picture
    if (ResizeParams.scaleFactor == 1.0) {
        OutTex[dtid.xy] = InTex[dtid.xy];
        return;
    }
    // Prepares for Picture Scale Changing
    // Maps Coordinates
    float2 srcCoord = dtid.xy / ResizeParams.scaleFactor;
    float2 inDims = float2(inputDims);
    srcCoord = clamp(srcCoord, 0.0, inDims - 1.0);
    int2 ipos = int2(srcCoord);
    // Nearest neighbor sampling
    if (!ResizeParams.useBilinear) {
        OutTex[dtid.xy] = InTex[ipos];
        return;
    }
    // Bilinear Compute; Slower but looks better
    int inW = int(inputDims.x);
    int inH = int(inputDims.y);
    int x1 = (ipos.x < inW - 1) ? (ipos.x + 1) : ipos.x;
    int y1 = (ipos.y < inH - 1) ? (ipos.y + 1) : ipos.y;
    int2 ipos1 = int2(x1, y1);
    // Compute Fractional Part for Interpolation
    float2 frac = srcCoord - float2(ipos);
    // Sample the 4 Fixels Needed
    float4 c00 = InTex[ipos];
    float4 c10 = InTex[int2(x1, ipos.y)];
    float4 c01 = InTex[int2(ipos.x, y1)];
    float4 c11 = InTex[ipos1];
    // Perform Bilinear Interpolation
    float4 c0 = lerp(c00, c10, frac.x);
    float4 c1 = lerp(c01, c11, frac.x);
    float4 c = lerp(c0, c1, frac.y);

    OutTex[dtid.xy] = c;
}