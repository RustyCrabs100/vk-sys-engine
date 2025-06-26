module upscaler;

namespace upscaler {
void bipolation_fn(
    float useBilinear,
    float scaleFactor,
    uint3 dtid: SV_DispatchThreadID,
    readonly Texture2D<float4> InTex,
    RWTexture2D<float4> OutTex,
    uint2 dims[2]
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
}