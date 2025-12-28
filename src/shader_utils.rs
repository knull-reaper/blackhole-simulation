#![allow(unsafe_op_in_unsafe_fn)]
use glow::HasContext;
use std::fs;

pub unsafe fn create_shader_program(
    gl: &glow::Context,
    vertex_path: &str,
    fragment_path: &str,
) -> anyhow::Result<glow::NativeProgram> {
    let vertex_src = fs::read_to_string(vertex_path)?;
    let fragment_src = fs::read_to_string(fragment_path)?;

    let program = gl.create_program().map_err(|e| anyhow::anyhow!(e))?;

    let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, &vertex_src)?;
    gl.attach_shader(program, vertex_shader);

    let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, &fragment_src)?;
    gl.attach_shader(program, fragment_shader);

    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        let log = gl.get_program_info_log(program);
        anyhow::bail!("Shader linking failed: {}", log);
    }

    gl.detach_shader(program, vertex_shader);
    gl.delete_shader(vertex_shader);
    gl.detach_shader(program, fragment_shader);
    gl.delete_shader(fragment_shader);

    Ok(program)
}

unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> anyhow::Result<glow::NativeShader> {
    let shader = gl.create_shader(shader_type).map_err(|e| anyhow::anyhow!(e))?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        let log = gl.get_shader_info_log(shader);
        anyhow::bail!("Shader compilation failed: {}", log);
    }

    Ok(shader)
}
