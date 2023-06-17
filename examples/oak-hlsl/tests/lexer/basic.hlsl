// HLSL (High Level Shading Language) basic syntax test file

// Vertex shader
float4 main(float4 pos : POSITION) : SV_POSITION
{
    return pos;
}

// Pixel/Fragment shader with texture sampling
float4 main(float2 texCoord : TEXCOORD0) : SV_TARGET
{
    return float4(1.0f, 0.0f, 0.0f, 1.0f);
}

// Constant buffer
struct CB
{
    float4x4 worldMatrix;
    float4x4 viewMatrix;
    float4x4 projectionMatrix;
    float4 lightDirection;
    float4 lightColor;
    float time;
};

cbuffer ConstantBuffer : register(b0)
{
    CB cb;
};

// Texture and sampler
Texture2D gTexture : register(t0);
SamplerState gSampler : register(s0);

// Vertex structure
struct VS_INPUT
{
    float4 position : POSITION;
    float2 texcoord : TEXCOORD0;
    float3 normal : NORMAL;
    float4 color : COLOR0;
};

struct VS_OUTPUT
{
    float4 position : SV_POSITION;
    float2 texcoord : TEXCOORD0;
    float3 normal : NORMAL;
    float4 color : COLOR0;
    float3 worldPos : TEXCOORD1;
};

// Vertex shader with transformations
VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;
    
    // Transform position
    float4 worldPos = mul(input.position, cb.worldMatrix);
    output.worldPos = worldPos.xyz;
    
    // Transform to view space
    float4 viewPos = mul(worldPos, cb.viewMatrix);
    
    // Transform to projection space
    output.position = mul(viewPos, cb.projectionMatrix);
    
    // Pass through other attributes
    output.texcoord = input.texcoord;
    output.normal = mul(input.normal, (float3x3)cb.worldMatrix);
    output.color = input.color;
    
    return output;
}

// Pixel shader with lighting
float4 ps_main(VS_OUTPUT input) : SV_TARGET
{
    // Sample texture
    float4 texColor = gTexture.Sample(gSampler, input.texcoord);
    
    // Simple directional lighting
    float3 normal = normalize(input.normal);
    float lightIntensity = max(dot(normal, -cb.lightDirection.xyz), 0.0f);
    
    // Combine lighting with texture
    float4 finalColor = texColor * cb.lightColor;
    finalColor.rgb *= (0.3f + 0.7f * lightIntensity); // ambient + diffuse
    finalColor.a = texColor.a;
    
    return finalColor;
}

// Compute shader
[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    // Simple compute shader example
    uint index = id.x;
    // Process data here
}

// Geometry shader
[maxvertexcount(3)]
void gs_main(point VS_OUTPUT input[1], inout TriangleStream<GS_OUTPUT> triStream)
{
    // Geometry shader implementation
}

// Domain shader
DS_OUTPUT ds_main(HS_CONSTANT_DATA_OUTPUT input, float2 uv : SV_DomainLocation, const OutputPatch<HS_OUTPUT, 3> patch)
{
    DS_OUTPUT output;
    // Domain shader implementation
    return output;
}

// Hull shader
HS_CONSTANT_DATA_OUTPUT hs_main(InputPatch<VS_OUTPUT, 3> input, uint patchId : SV_PrimitiveID)
{
    HS_CONSTANT_DATA_OUTPUT output;
    // Hull shader implementation
    return output;
}