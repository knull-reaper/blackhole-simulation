#![allow(unsafe_op_in_unsafe_fn)]
use glow::HasContext;
use std::collections::HashMap;

pub struct RenderPass {
    program: glow::Program,
    vao: glow::VertexArray,
    uniform_locations: HashMap<String, Option<glow::UniformLocation>>,
}

impl RenderPass {
    pub unsafe fn new(
        gl: &glow::Context,
        vertex_path: &str,
        fragment_path: &str,
        vao: glow::VertexArray,
    ) -> anyhow::Result<Self> {
        let program = crate::shader_utils::create_shader_program(gl, vertex_path, fragment_path)?;
        Ok(Self {
            program,
            vao,
            uniform_locations: HashMap::new(),
        })
    }

    fn uniform_location(
        &mut self,
        gl: &glow::Context,
        name: &str,
    ) -> Option<&glow::UniformLocation> {
        if !self.uniform_locations.contains_key(name) {
            let loc = unsafe { gl.get_uniform_location(self.program, name) };
            self.uniform_locations.insert(name.to_string(), loc);
        }
        self.uniform_locations.get(name).and_then(|loc| loc.as_ref())
    }

    pub unsafe fn render(
        &mut self,
        gl: &glow::Context,
        target_framebuffer: Option<glow::Framebuffer>,
        width: i32,
        height: i32,
        float_uniforms: &[(&str, f32)],
        texture_uniforms: &[(&str, glow::Texture)],
        texture_3d_uniforms: &[(&str, glow::Texture)],
        cubemap_uniforms: &[(&str, glow::Texture)],
    ) {
        gl.bind_framebuffer(glow::FRAMEBUFFER, target_framebuffer);
        gl.viewport(0, 0, width, height);

        gl.disable(glow::DEPTH_TEST);

        gl.use_program(Some(self.program));

        if let Some(loc) = self.uniform_location(gl, "resolution") {
            gl.uniform_2_f32(Some(loc), width as f32, height as f32);
        }

        for (name, val) in float_uniforms {
            if let Some(loc) = self.uniform_location(gl, name) {
                gl.uniform_1_f32(Some(loc), *val);
            }
        }

        let mut unit: u32 = 0;
        for (name, tex) in texture_uniforms {
            if let Some(loc) = self.uniform_location(gl, name) {
                gl.active_texture(glow::TEXTURE0 + unit);
                gl.bind_texture(glow::TEXTURE_2D, Some(*tex));
                gl.uniform_1_i32(Some(loc), unit as i32);
                unit += 1;
            }
        }

        for (name, tex) in texture_3d_uniforms {
            if let Some(loc) = self.uniform_location(gl, name) {
                gl.active_texture(glow::TEXTURE0 + unit);
                gl.bind_texture(glow::TEXTURE_3D, Some(*tex));
                gl.uniform_1_i32(Some(loc), unit as i32);
                unit += 1;
            }
        }
        
        for (name, tex) in cubemap_uniforms {
            if let Some(loc) = self.uniform_location(gl, name) {
                gl.active_texture(glow::TEXTURE0 + unit);
                gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(*tex));
                gl.uniform_1_i32(Some(loc), unit as i32);
                unit += 1;
            }
        }

        gl.bind_vertex_array(Some(self.vao));
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
        gl.bind_vertex_array(None);
        gl.use_program(None);
    }
}
