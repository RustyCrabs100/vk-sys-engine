#language slang legacy

struct SceneParameters {
    float4x4 MVP;
}
ConstantBuffer<SceneParameters> SceneParams;

struct VSOutput {
    float4 Position : SV_Position;
    float2 TexCoord : TEXCOORD;
    __init(in float4 pos, in float2 tex) {
        Position = pos;
        TexCoord = tex;
    }
}


[shader("vertex")]
func vertexMain(
    float3 positionOS: POSITION,
    float2 TexCoord: TEXCOORD
) ->VSOutput {
    var posVS = mul(SceneParams.MVP, float4(positionOS, 1));
    return VSOutput(posVS, TexCoord);
}