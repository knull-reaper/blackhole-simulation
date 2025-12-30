#![allow(unsafe_op_in_unsafe_fn)]
use glow::HasContext;
use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use std::path::Path;

pub unsafe fn create_shader_program(
    gl: &glow::Context,
    vertex_path: &str,
    fragment_path: &str,
) -> anyhow::Result<glow::Program> {
    let vertex_src = load_shader_source(vertex_path)?;
    let fragment_src = load_shader_source(fragment_path)?;

    let program = gl.create_program().map_err(|e| anyhow::anyhow!(e))?;

    let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_src.as_ref())?;
    gl.attach_shader(program, vertex_shader);

    let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src.as_ref())?;
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

#[cfg(not(target_arch = "wasm32"))]
fn load_shader_source(path: &str) -> anyhow::Result<Cow<'static, str>> {
    Ok(Cow::Owned(fs::read_to_string(path)?))
}

#[cfg(target_arch = "wasm32")]
fn load_shader_source(path: &str) -> anyhow::Result<Cow<'static, str>> {
    let name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid shader path: {}", path))?;
    let source = shader_source_for_name(name)
        .ok_or_else(|| anyhow::anyhow!("Unknown shader asset: {}", name))?;
    Ok(Cow::Owned(normalize_shader_source(name, source)))
}

#[cfg(target_arch = "wasm32")]
fn shader_source_for_name(name: &str) -> Option<&'static str> {
    match name {
        "simple.vert" => Some(include_str!("../shader/simple.vert")),
        "blackhole_main.frag" => Some(include_str!("../shader/blackhole_main.frag")),
        "bloom_brightness_pass.frag" => Some(include_str!("../shader/bloom_brightness_pass.frag")),
        "lens_flare.frag" => Some(include_str!("../shader/lens_flare.frag")),
        "bloom_downsample.frag" => Some(include_str!("../shader/bloom_downsample.frag")),
        "bloom_upsample.frag" => Some(include_str!("../shader/bloom_upsample.frag")),
        "bloom_composite.frag" => Some(include_str!("../shader/bloom_composite.frag")),
        "tonemapping.frag" => Some(include_str!("../shader/tonemapping.frag")),
        "passthrough.frag" => Some(include_str!("../shader/passthrough.frag")),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
fn normalize_shader_source(name: &str, source: &str) -> String {
    let normalized = source.replace("#version 330 core", "#version 300 es");
    let mut lines = normalized.lines();
    let mut output = String::new();

    if let Some(first) = lines.next() {
        output.push_str(first);
        output.push('\n');
    }

    if name.ends_with(".frag") && !normalized.contains("precision ") {
        output.push_str("precision highp float;\n");
        output.push_str("precision highp sampler2D;\n");
        output.push_str("precision highp sampler3D;\n");
        output.push_str("precision highp samplerCube;\n");
    }

    for line in lines {
        if name.ends_with(".frag") {
            output.push_str(&strip_uniform_initializer(line));
        } else {
            output.push_str(line);
        }
        output.push('\n');
    }

    output
}

#[cfg(target_arch = "wasm32")]
fn strip_uniform_initializer(line: &str) -> String {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("uniform ") {
        return line.to_string();
    }
    let eq_pos = match trimmed.find('=') {
        Some(pos) => pos,
        None => return line.to_string(),
    };
    let semi_pos = match trimmed.find(';') {
        Some(pos) => pos,
        None => return line.to_string(),
    };
    if eq_pos > semi_pos {
        return line.to_string();
    }
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];
    let left = trimmed[..eq_pos].trim_end();
    let suffix = if semi_pos + 1 < trimmed.len() {
        &trimmed[semi_pos + 1..]
    } else {
        ""
    };
    let mut output = String::new();
    output.push_str(indent);
    output.push_str(left);
    output.push(';');
    output.push_str(suffix);
    output
}

unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> anyhow::Result<glow::Shader> {
    let shader = gl.create_shader(shader_type).map_err(|e| anyhow::anyhow!(e))?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        let log = gl.get_shader_info_log(shader);
        anyhow::bail!("Shader compilation failed: {}", log);
    }

    Ok(shader)
}
