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
    output.view_index = view_index;

    return output;
}