Texture2D tex;
SamplerState samplerState;

struct PixelInputType
{
    float4 position : SV_POSITION;
    float2 uv : UV;
    uint view_index : VIEW_INDEX; // this should probably be a global constant?
};

float4 main(PixelInputType input) : SV_TARGET
{
    return lerp(float4(1.0, 0.0, 0.0, 1.0), float4(0.0, 1.0, 0.0, 1.0), input.view_index);
}