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
    output.position = input.position;
    output.uv = input.uv;

    return output;
}