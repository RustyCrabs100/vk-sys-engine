#language slang legacy

Texture2D<float4> InTex;
RWTexture2D<float4> OutTex;

[shader("compute")]
[numthreads(16, 16, 1)]
func computeMain(uint2 dtid: SV_DispatchThreadID)->void {
    OutTex[dtid.xy] = InTex[dtid.xy];
}