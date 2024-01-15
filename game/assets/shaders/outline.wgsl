#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct OutlineMaterial {
    color: vec4<f32>,
    thickness : f32
};
@group(1) @binding(0)
var<uniform> material: OutlineMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

fn get_sample(
    probe: vec2<f32>
) -> f32 {
    return textureSample(base_color_texture, base_color_sampler, probe).a;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var uv = in.uv;
    var outline : f32 = get_sample(uv + vec2<f32>(material.thickness,0.0));
    outline += get_sample(uv + vec2<f32>(-material.thickness,0.0));
    outline += get_sample(uv + vec2<f32>(0.0,material.thickness));
    outline += get_sample(uv + vec2<f32>(0.0,-material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness,-material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness,material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness,material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness,-material.thickness));
    outline = min(outline, 1.0);
    var color : vec4<f32> = textureSample(base_color_texture, base_color_sampler,uv);
    return mix(color, material.color, outline - color.a) - color;
}


// #import bevy_sprite::mesh2d_vertex_output VertexOutput

// struct CustomGridMaterial {
//     line_color: vec4<f32>,
//     grid_size: vec2<f32>,
//     cell_size: vec2<f32>,
//     major: f32,
// };

// @group(1) @binding(0)
// var<uniform> material: CustomGridMaterial;

// fn grid(point: vec2<f32>, cell_size: vec2<f32>, thickness: f32) -> f32 {
//   let x = (abs(fract(point.x / cell_size.x)) - thickness) * cell_size.x;
//   let y = (abs(fract(point.y / cell_size.y)) - thickness) * cell_size.y;
//   return min(x, y);
// }

// fn origin(point: vec2<f32>, thickness: f32) -> f32 {
//   return min(abs(point.x), abs(point.y)) - thickness;
// }

// @fragment
// fn fragment(
//     mesh: VertexOutput,
// ) -> @location(0) vec4<f32> {
//     let line_color: vec4<f32> = material.line_color;
//     let grid_size: vec2<f32> = material.grid_size;
//     let cell_size: vec2<f32> = material.cell_size;
//     let major: f32 = material.major;

//     let point = (mesh.uv - vec2(0.5)) * grid_size;

//     let t = grid(point, cell_size, 0.05);
//     let u = grid(point, cell_size * major, 0.2 / major);
//     let g = min(t, u);
//     let alpha =  1.0 - smoothstep(0.0, fwidth(g), g);
//     return vec4(line_color.rgb, alpha * line_color.a);
// }