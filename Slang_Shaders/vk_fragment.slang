#language slang legacy

struct ColorData {
    float4 color;
}
ConstantBuffer<ColorData> InputColor;

[shader("fragment")]
func fragmentMain()->float4 {
    return InputColor.color;
}