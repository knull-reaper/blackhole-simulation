#![allow(unsafe_op_in_unsafe_fn)]
use glow::HasContext;

pub unsafe fn create_quad_vao(gl: &glow::Context) -> anyhow::Result<glow::VertexArray> {
    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0,
        -1.0,  1.0, 0.0,
         1.0,  1.0, 0.0,
         1.0,  1.0, 0.0,
         1.0, -1.0, 0.0,
        -1.0, -1.0, 0.0,
    ];

    let vao = gl.create_vertex_array().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_vertex_array(Some(vao));

    let vbo = gl.create_buffer().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        bytemuck::cast_slice(&vertices),
        glow::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

    gl.bind_vertex_array(None);
    
    Ok(vao)
}
