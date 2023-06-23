// WGSL (WebGPU Shading Language) Test File - Comprehensive Syntax Coverage
// This file tests various WGSL syntax elements for lexer testing

// Struct definitions
struct VertexInput {
    ↯location(0) position: vec3<f32>,
    ↯location(1) normal: vec3<f32>,
    ↯location(2) uv: vec2<f32>,
    ↯location(3) color: vec4<f32>,
}

struct VertexOutput {
    ↯builtin(position) clip_position: vec4<f32>,
    ↯location(0) world_position: vec3<f32>,
    ↯location(1) world_normal: vec3<f32>,
    ↯location(2) uv: vec2<f32>,
    ↯location(3) color: vec4<f32>,
}

struct FragmentOutput {
    ↯location(0) color: vec4<f32>,
}

struct Camera {
    view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    position: vec3<f32>,
    direction: vec3<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    range: f32,
}

struct Material {
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32,
    emissive: vec3<f32>,
}

struct Transform {
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
}

// Uniform buffer bindings
↯group(0) ↯binding(0) var<uniform> camera: Camera;
↯group(0) ↯binding(1) var<uniform> transform: Transform;
↯group(0) ↯binding(2) var<uniform> material: Material;
↯group(0) ↯binding(3) var<uniform> lights: array<Light, 8>;
↯group(0) ↯binding(4) var<uniform> time: f32;

// Texture and sampler bindings
↯group(1) ↯binding(0) var albedo_texture: texture_2d<f32>;
↯group(1) ↯binding(1) var normal_texture: texture_2d<f32>;
↯group(1) ↯binding(2) var metallic_roughness_texture: texture_2d<f32>;
↯group(1) ↯binding(3) var ao_texture: texture_2d<f32>;
↯group(1) ↯binding(4) var emissive_texture: texture_2d<f32>;
↯group(1) ↯binding(5) var texture_sampler: sampler;

// Cube texture for environment mapping
↯group(2) ↯binding(0) var environment_map: texture_cube<f32>;
↯group(2) ↯binding(1) var environment_sampler: sampler;

// Storage buffers
↯group(3) ↯binding(0) var<storage, read> vertex_buffer: array<VertexInput>;
↯group(3) ↯binding(1) var<storage, read_write> compute_buffer: array<f32>;

// Constants
const PI: f32 = 3.14159265359;
const TWO_PI: f32 = 6.28318530718;
const HALF_PI: f32 = 1.57079632679;
const INV_PI: f32 = 0.31830988618;
const MAX_LIGHTS: u32 = 8u;

// Type aliases
alias Vec3 = vec3<f32>;
alias Vec4 = vec4<f32>;
alias Mat4 = mat4x4<f32>;

// Utility functions
fn saturate(value: f32) -> f32 {
    return clamp(value, 0.0, 1.0);
}

fn saturate_vec3(value: vec3<f32>) -> vec3<f32> {
    return clamp(value, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn pow5(x: f32) -> f32 {
    let x2 = x * x;
    return x2 * x2 * x;
}

fn luminance(color: vec3<f32>) -> f32 {
    return dot(color, vec3<f32>(0.299, 0.587, 0.114));
}

// Math utility functions
fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn distance_squared(a: vec3<f32>, b: vec3<f32>) -> f32 {
    let diff = a - b;
    return dot(diff, diff);
}

fn reflect_vector(incident: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return incident - 2.0 * dot(incident, normal) * normal;
}

// Fresnel calculations
fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(saturate(1.0 - cos_theta), 5.0);
}

fn fresnel_schlick_roughness(cos_theta: f32, f0: vec3<f32>, roughness: f32) -> vec3<f32> {
    return f0 + (max(vec3<f32>(1.0 - roughness), f0) - f0) * pow(saturate(1.0 - cos_theta), 5.0);
}

// Distribution functions
fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let n_dot_h2 = n_dot_h * n_dot_h;
    
    let num = a2;
    var denom = (n_dot_h2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
    
    return num / denom;
}

// Geometry functions
fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    
    let num = n_dot_v;
    let denom = n_dot_v * (1.0 - k) + k;
    
    return num / denom;
}

fn geometry_smith(normal: vec3<f32>, view: vec3<f32>, light: vec3<f32>, roughness: f32) -> f32 {
    let n_dot_v = max(dot(normal, view), 0.0);
    let n_dot_l = max(dot(normal, light), 0.0);
    let ggx2 = geometry_schlick_ggx(n_dot_v, roughness);
    let ggx1 = geometry_schlick_ggx(n_dot_l, roughness);
    
    return ggx1 * ggx2;
}

// PBR lighting calculation
fn calculate_pbr_lighting(
    world_pos: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32
) -> vec3<f32> {
    var color = vec3<f32>(0.0);
    
    // Calculate F0 (surface reflection at zero incidence)
    var f0 = vec3<f32>(0.04);
    f0 = mix(f0, albedo, metallic);
    
    // Loop through lights
    for (var i: u32 = 0u; i < MAX_LIGHTS; i = i + 1u) {
        let light = lights[i];
        
        // Calculate light direction and distance
        let light_dir = normalize(light.position - world_pos);
        let distance = length(light.position - world_pos);
        let attenuation = 1.0 / (distance * distance);
        let radiance = light.color * light.intensity * attenuation;
        
        // Calculate angles
        let half_vector = normalize(view_dir + light_dir);
        let n_dot_l = max(dot(normal, light_dir), 0.0);
        let n_dot_v = max(dot(normal, view_dir), 0.0);
        let n_dot_h = max(dot(normal, half_vector), 0.0);
        let v_dot_h = max(dot(view_dir, half_vector), 0.0);
        
        // Calculate BRDF components
        let ndf = distribution_ggx(n_dot_h, roughness);
        let g = geometry_smith(normal, view_dir, light_dir, roughness);
        let f = fresnel_schlick(v_dot_h, f0);
        
        // Calculate specular and diffuse
        let numerator = ndf * g * f;
        let denominator = 4.0 * n_dot_v * n_dot_l + 0.0001;
        let specular = numerator / denominator;
        
        let ks = f;
        var kd = vec3<f32>(1.0) - ks;
        kd = kd * (1.0 - metallic);
        
        color = color + (kd * albedo / PI + specular) * radiance * n_dot_l;
    }
    
    // Ambient lighting
    let ambient = vec3<f32>(0.03) * albedo * ao;
    color = color + ambient;
    
    return color;
}

// Tone mapping functions
fn reinhard_tone_mapping(color: vec3<f32>) -> vec3<f32> {
    return color / (color + vec3<f32>(1.0));
}

fn aces_tone_mapping(color: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return saturate_vec3((color * (a * color + b)) / (color * (c * color + d) + e));
}

// Gamma correction
fn gamma_correct(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / 2.2));
}

// Vertex shader
↯vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transform position to world space
    let world_position = transform.model * vec4<f32>(input.position, 1.0);
    output.world_position = world_position.xyz;
    
    // Transform to clip space
    output.clip_position = camera.view_proj * world_position;
    
    // Transform normal to world space
    output.world_normal = normalize(transform.normal * input.normal);
    
    // Pass through UV coordinates
    output.uv = input.uv;
    
    // Pass through vertex color
    output.color = input.color;
    
    return output;
}

// Fragment shader
↯fragment
fn fs_main(input: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;
    
    // Sample textures
    let albedo_sample = textureSample(albedo_texture, texture_sampler, input.uv);
    let normal_sample = textureSample(normal_texture, texture_sampler, input.uv);
    let metallic_roughness_sample = textureSample(metallic_roughness_texture, texture_sampler, input.uv);
    let ao_sample = textureSample(ao_texture, texture_sampler, input.uv);
    let emissive_sample = textureSample(emissive_texture, texture_sampler, input.uv);
    
    // Extract material properties
    let albedo = albedo_sample.rgb * material.albedo * input.color.rgb;
    let metallic = metallic_roughness_sample.b * material.metallic;
    let roughness = metallic_roughness_sample.g * material.roughness;
    let ao = ao_sample.r * material.ao;
    let emissive = emissive_sample.rgb * material.emissive;
    
    // Calculate normal from normal map
    let normal_map = normal_sample.rgb * 2.0 - 1.0;
    // Note: In a real implementation, you'd need to calculate TBN matrix
    let normal = normalize(input.world_normal);
    
    // Calculate view direction
    let view_dir = normalize(camera.position - input.world_position);
    
    // Calculate PBR lighting
    var color = calculate_pbr_lighting(
        input.world_position,
        normal,
        view_dir,
        albedo,
        metallic,
        roughness,
        ao
    );
    
    // Add emissive
    color = color + emissive;
    
    // Tone mapping
    color = aces_tone_mapping(color);
    
    // Gamma correction
    color = gamma_correct(color);
    
    output.color = vec4<f32>(color, albedo_sample.a * input.color.a);
    
    return output;
}

// Compute shader for particle simulation
struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    size: f32,
}

↯group(0) ↯binding(0) var<storage, read_write> particles: array<Particle>;
↯group(0) ↯binding(1) var<uniform> delta_time: f32;
↯group(0) ↯binding(2) var<uniform> gravity: vec3<f32>;

↯compute ↯workgroup_size(64)
fn cs_particle_update(↯builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    if (index >= arrayLength(&particles)) {
        return;
    }
    
    var particle = particles[index];
    
    // Update velocity with gravity
    particle.velocity = particle.velocity + gravity * delta_time;
    
    // Update position
    particle.position = particle.position + particle.velocity * delta_time;
    
    // Update life
    particle.life = particle.life - delta_time;
    
    // Reset particle if dead
    if (particle.life <= 0.0) {
        particle.position = vec3<f32>(0.0, 10.0, 0.0);
        particle.velocity = vec3<f32>(
            (f32(index) * 0.1) % 2.0 - 1.0,
            5.0,
            (f32(index) * 0.2) % 2.0 - 1.0
        );
        particle.life = 5.0;
        particle.size = 1.0;
    }
    
    particles[index] = particle;
}

// Post-processing compute shader
↯group(0) ↯binding(0) var input_texture: texture_2d<f32>;
↯group(0) ↯binding(1) var output_texture: texture_storage_2d<rgba8unorm, write>;

↯compute ↯workgroup_size(8, 8)
fn cs_post_process(↯builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dimensions = textureDimensions(input_texture);
    
    if (coords.x >= i32(dimensions.x) || coords.y >= i32(dimensions.y)) {
        return;
    }
    
    // Sample input texture
    let color = textureLoad(input_texture, coords, 0);
    
    // Apply some post-processing effect (simple blur)
    var blurred_color = color;
    let blur_radius = 1;
    var sample_count = 0.0;
    
    for (var x = -blur_radius; x <= blur_radius; x = x + 1) {
        for (var y = -blur_radius; y <= blur_radius; y = y + 1) {
            let sample_coords = coords + vec2<i32>(x, y);
            if (sample_coords.x >= 0 && sample_coords.x < i32(dimensions.x) &&
                sample_coords.y >= 0 && sample_coords.y < i32(dimensions.y)) {
                blurred_color = blurred_color + textureLoad(input_texture, sample_coords, 0);
                sample_count = sample_count + 1.0;
            }
        }
    }
    
    blurred_color = blurred_color / sample_count;
    
    // Write to output texture
    textureStore(output_texture, coords, blurred_color);
}

// Shadow mapping vertex shader
↯vertex
fn vs_shadow(↯location(0) position: vec3<f32>) -> ↯builtin(position) vec4<f32> {
    return camera.view_proj * transform.model * vec4<f32>(position, 1.0);
}

// Skybox vertex shader
struct SkyboxOutput {
    ↯builtin(position) position: vec4<f32>,
    ↯location(0) world_position: vec3<f32>,
}

↯vertex
fn vs_skybox(↯location(0) position: vec3<f32>) -> SkyboxOutput {
    var output: SkyboxOutput;
    
    // Remove translation from view matrix
    var view_no_translation = camera.view;
    view_no_translation[3][0] = 0.0;
    view_no_translation[3][1] = 0.0;
    view_no_translation[3][2] = 0.0;
    
    let clip_pos = camera.proj * view_no_translation * vec4<f32>(position, 1.0);
    output.position = clip_pos.xyww; // Ensure skybox is always at far plane
    output.world_position = position;
    
    return output;
}

↯fragment
fn fs_skybox(input: SkyboxOutput) -> ↯location(0) vec4<f32> {
    return textureSample(environment_map, environment_sampler, input.world_position);
}

// Instanced rendering structures
struct InstanceData {
    model_matrix: mat4x4<f32>,
    color: vec4<f32>,
}

↯group(0) ↯binding(0) var<storage, read> instances: array<InstanceData>;

↯vertex
fn vs_instanced(
    ↯location(0) position: vec3<f32>,
    ↯location(1) normal: vec3<f32>,
    ↯location(2) uv: vec2<f32>,
    ↯builtin(instance_index) instance_index: u32
) -> VertexOutput {
    var output: VertexOutput;
    
    let instance = instances[instance_index];
    
    // Transform position to world space using instance matrix
    let world_position = instance.model_matrix * vec4<f32>(position, 1.0);
    output.world_position = world_position.xyz;
    
    // Transform to clip space
    output.clip_position = camera.view_proj * world_position;
    
    // Transform normal (assuming uniform scaling)
    output.world_normal = normalize((instance.model_matrix * vec4<f32>(normal, 0.0)).xyz);
    
    output.uv = uv;
    output.color = instance.color;
    
    return output;
}