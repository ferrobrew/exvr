Texture2D tex;
SamplerState samplerState;

struct PixelInputType
{
    float4 position : SV_POSITION;
    float2 uv : UV;
};

float4 main(PixelInputType input) : SV_TARGET
{
    return pow(tex.Sample(samplerState, input.uv), 1/2.2);
}