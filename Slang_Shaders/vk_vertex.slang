#language slang legacy

struct SceneParameters {
    float4x4 MVP;
}
ConstantBuffer<SceneParameters> SceneParams;

struct VSOutput {
    float4 Position : SV_Position;
    float2 TexCoord : TEXCOORD;
    __init() {
        Position = float4(0.0, 0.0, 0.0, 0.0);
        TexCoord = float2(0.0, 0.0);
    }
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
    var output : VSOutput = VSOutput();
    output.Position = posVS;
    output.TexCoord = TexCoord;
    return output;
}