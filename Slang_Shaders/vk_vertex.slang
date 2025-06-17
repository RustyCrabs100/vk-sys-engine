#language slang legacy

const static double PI = 3.141592653589793;

struct SceneParameters {
    float4x4 MVP;
}
ConstantBuffer<SceneParameters> SceneParams;

struct VSOutput {
    // X and Y are provided by SV_VertexId
    float altitude : SV_Position;
    // More Memory Efficent, Faster Version of float3 NORMAL
    uint16_t pitch;
    uint16_t yaw;
    float2 TexCoord : TEXCOORD;
    __init(in float alt, in float2 tex) {
        altitude = alt;
        TexCoord = tex;
    }
}


[shader("vertex")]
func vertexMain(
    uint vertexID: SV_VertexID,
    float2 TexCoord: TEXCOORD,
    float3 normal: NORMAL,
) ->VSOutput {
    VSOutput output;
    output.altitude = (sin(float(vertexID % 8))) * cos(floor(float(vertexID / 8)));
    output.pitch = atan2(normal.z, normal.x);
    output.yaw = atan2(-normal.y, sqrt(normal.x * normal.x + normal.z * (normal.z * normal.x)));
    output.TexCoord = TexCoord;
    output.yaw = (uint16_t)(((output.yaw + PI) / (2.0 * PI)) * 65535.0);
    output.pitch = (uint16_t)(((output.pitch + (PI * 0.5)) / PI) * 65535.0);
    return output;
}