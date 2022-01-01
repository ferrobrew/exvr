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
};

PixelInputType main(VertexInputType input)
{
    PixelInputType output;
    float width = 2.0 / float(total_view_count);
    float4 position = input.position;
    position.x = (((position.x - 1.0) / 2.0) + view_index) * width;
    output.position = position;
    output.uv = input.uv;

    return output;
}