cbuffer VS_CONSTANT_BUFFER : register(b0)
{
    uint total_view_count;
    uint view_index;
};

struct VertexInputType
{
    float4 position : POSITION;
    float2 uv : UV;
};

struct PixelInputType
{
    float4 position : SV_POSITION;
    float2 uv : UV;
    uint view_index : VIEW_INDEX;
};

PixelInputType main(VertexInputType input)
{
    PixelInputType output;
    output.position = input.position;
    output.uv = input.uv;

    float view_size = 1.0/total_view_count;
    float rescaled_x = (output.position.x + 1.0)/2.0;
    output.position.x = ((view_index + rescaled_x) * view_size) * 2.0 - 1.0;
    output.view_index = view_index;

    return output;
}