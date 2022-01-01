Texture2D tex;
SamplerState samplerState;

cbuffer VS_CONSTANT_BUFFER : register(b0)
{
    uint total_view_count;
    uint view_index;
};

struct PixelInputType
{
    float4 position : SV_POSITION;
    float2 uv : UV;
};

float4 main(PixelInputType input) : SV_TARGET
{
    float4 multiplier = float4(1.0, 1.0, 1.0, 1.0);
    // christmas debugging mode
    if (true)
    {
        multiplier = lerp(float4(1.0, 0.0, 0.0, 1.0), float4(0.0, 1.0, 0.0, 1.0), view_index / float(total_view_count - 1));
    }
    return multiplier * tex.Sample(samplerState, input.uv);
}