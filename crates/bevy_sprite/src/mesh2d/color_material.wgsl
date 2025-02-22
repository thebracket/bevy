#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct ColorMaterial {
    color: vec4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
};
let COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> material: ColorMaterial;
[[group(1), binding(1)]]
var texture: texture_2d<f32>;
[[group(1), binding(2)]]
var texture_sampler: sampler;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
#ifdef VERTEX_COLORS
    [[location(4)]] colors: vec4<f32>;
#endif
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var output_color: vec4<f32> = material.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
#ifdef VERTEX_COLORS
        output_color = output_color * textureSample(texture, texture_sampler, in.uv) * in.colors;
#else
        output_color = output_color * textureSample(texture, texture_sampler, in.uv);
#endif
    }
    return output_color;
}