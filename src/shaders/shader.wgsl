
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_mat_0: vec4<f32>,
    @location(6) model_mat_1: vec4<f32>,
    @location(7) model_mat_2: vec4<f32>,
    @location(8) model_mat_3: vec4<f32>,
    @location(9) model_color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let model_mat = mat4x4<f32>(
    	instance.model_mat_0, 
    	instance.model_mat_1, 
    	instance.model_mat_2, 
    	instance.model_mat_3, 
    );

    var out: VertexOutput;
    out.color = instance.model_color;
    out.clip_position = model_mat * vec4<f32>(in.position, 1.0);
    return out;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
 