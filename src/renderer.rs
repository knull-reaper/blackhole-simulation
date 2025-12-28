use anyhow::Context;
use glow::HasContext;
use std::path::PathBuf;
use crate::app_state::AppState;
use crate::render_pass::RenderPass;

const MAX_BLOOM_ITER: usize = 8;

fn flag(value: bool) -> f32 {
    if value {
        1.0
    } else {
        0.0
    }
}

fn delete_optional_texture(gl: &glow::Context, texture: &mut Option<glow::NativeTexture>) {
    if let Some(texture) = texture.take() {
        unsafe {
            gl.delete_texture(texture);
        }
    }
}

fn delete_optional_framebuffer(
    gl: &glow::Context,
    framebuffer: &mut Option<glow::NativeFramebuffer>,
) {
    if let Some(framebuffer) = framebuffer.take() {
        unsafe {
            gl.delete_framebuffer(framebuffer);
        }
    }
}

fn delete_textures(gl: &glow::Context, textures: &mut Vec<glow::NativeTexture>) {
    for texture in textures.drain(..) {
        unsafe {
            gl.delete_texture(texture);
        }
    }
}

fn delete_framebuffers(gl: &glow::Context, framebuffers: &mut Vec<glow::NativeFramebuffer>) {
    for framebuffer in framebuffers.drain(..) {
        unsafe {
            gl.delete_framebuffer(framebuffer);
        }
    }
}

fn find_asset_root() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe().context("Failed to resolve executable path")?;
    let mut dir = exe_path
        .parent()
        .context("Executable has no parent directory")?
        .to_path_buf();

    for _ in 0..5 {
        if dir.join("assets").is_dir() && dir.join("shader").is_dir() {
            return Ok(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }

    Err(anyhow::anyhow!(
        "Unable to locate `assets/` and `shader/`. Place them next to the binary or run from the project root."
    ))
}

pub struct Renderer {
    pass_blackhole: RenderPass,
    pass_brightness: RenderPass,
    pass_lens_flare: RenderPass,
    pass_downsample: RenderPass,
    pass_upsample: RenderPass,
    pass_composite: RenderPass,
    pass_tonemapping: RenderPass,
    pass_passthrough: RenderPass,

    tex_blackhole: Option<glow::NativeTexture>,
    fbo_blackhole: Option<glow::NativeFramebuffer>,
    
    tex_brightness: Option<glow::NativeTexture>,
    fbo_brightness: Option<glow::NativeFramebuffer>,

    tex_lens_flare: Option<glow::NativeTexture>,
    fbo_lens_flare: Option<glow::NativeFramebuffer>,

    tex_downsampled: Vec<glow::NativeTexture>,
    fbo_downsampled: Vec<glow::NativeFramebuffer>,

    tex_upsampled: Vec<glow::NativeTexture>,
    fbo_upsampled: Vec<glow::NativeFramebuffer>,

    tex_bloom_final: Option<glow::NativeTexture>,
    fbo_bloom_final: Option<glow::NativeFramebuffer>,

    tex_tonemapped: Option<glow::NativeTexture>,
    fbo_tonemapped: Option<glow::NativeFramebuffer>,

    galaxy_cubemap: glow::NativeTexture,
    color_map: glow::NativeTexture,
    noise_tex: glow::NativeTexture,

    width: u32,
    height: u32,
}

impl Renderer {
    pub unsafe fn new(gl: &glow::Context, width: u32, height: u32) -> anyhow::Result<Self> {
        let base_dir = find_asset_root()?;
        let shader_dir = base_dir.join("shader");
        let asset_dir = base_dir.join("assets");
        let quad_vao = crate::render_utils::create_quad_vao(gl)?;

        let load_pass = |v_name, f_name| -> anyhow::Result<RenderPass> {
            let v_path = shader_dir.join(v_name);
            let f_path = shader_dir.join(f_name);
            let v_path = v_path
                .to_str()
                .with_context(|| format!("Non-UTF8 shader path: {}", v_path.display()))?;
            let f_path = f_path
                .to_str()
                .with_context(|| format!("Non-UTF8 shader path: {}", f_path.display()))?;
            RenderPass::new(gl, v_path, f_path, quad_vao)
        };

        let pass_blackhole = load_pass("simple.vert", "blackhole_main.frag")?;
        let pass_brightness = load_pass("simple.vert", "bloom_brightness_pass.frag")?;
        let pass_lens_flare = load_pass("simple.vert", "lens_flare.frag")?;
        let pass_downsample = load_pass("simple.vert", "bloom_downsample.frag")?;
        let pass_upsample = load_pass("simple.vert", "bloom_upsample.frag")?;
        let pass_composite = load_pass("simple.vert", "bloom_composite.frag")?;
        let pass_tonemapping = load_pass("simple.vert", "tonemapping.frag")?;
        let pass_passthrough = load_pass("simple.vert", "passthrough.frag")?;

        let color_map_path = asset_dir.join("color_map.png");
        let galaxy_path = asset_dir.join("skybox_nebula_dark");
        let color_map_path = color_map_path
            .to_str()
            .with_context(|| format!("Non-UTF8 texture path: {}", color_map_path.display()))?;
        let galaxy_path = galaxy_path
            .to_str()
            .with_context(|| format!("Non-UTF8 cubemap path: {}", galaxy_path.display()))?;
        let color_map = crate::texture::load_texture_2d(gl, color_map_path)?;
        let galaxy_cubemap = crate::texture::load_cubemap(gl, galaxy_path)?;
        let noise_tex = crate::texture::create_noise_texture_3d(gl)?;

        let mut renderer = Self {
            pass_blackhole,
            pass_brightness,
            pass_lens_flare,
            pass_downsample,
            pass_upsample,
            pass_composite,
            pass_tonemapping,
            pass_passthrough,

            tex_blackhole: None,
            fbo_blackhole: None,
            tex_brightness: None,
            fbo_brightness: None,
            tex_lens_flare: None,
            fbo_lens_flare: None,
            tex_downsampled: vec![],
            fbo_downsampled: vec![],
            tex_upsampled: vec![],
            fbo_upsampled: vec![],
            tex_bloom_final: None,
            fbo_bloom_final: None,
            tex_tonemapped: None,
            fbo_tonemapped: None,

            galaxy_cubemap,
            color_map,
            noise_tex,

            width: 0,
            height: 0,
        };

        renderer.resize(gl, width, height)?;
        Ok(renderer)
    }

    unsafe fn destroy_targets(&mut self, gl: &glow::Context) {
        delete_optional_texture(gl, &mut self.tex_blackhole);
        delete_optional_framebuffer(gl, &mut self.fbo_blackhole);
        delete_optional_texture(gl, &mut self.tex_brightness);
        delete_optional_framebuffer(gl, &mut self.fbo_brightness);
        delete_optional_texture(gl, &mut self.tex_lens_flare);
        delete_optional_framebuffer(gl, &mut self.fbo_lens_flare);
        delete_optional_texture(gl, &mut self.tex_bloom_final);
        delete_optional_framebuffer(gl, &mut self.fbo_bloom_final);
        delete_optional_texture(gl, &mut self.tex_tonemapped);
        delete_optional_framebuffer(gl, &mut self.fbo_tonemapped);

        delete_textures(gl, &mut self.tex_downsampled);
        delete_framebuffers(gl, &mut self.fbo_downsampled);
        delete_textures(gl, &mut self.tex_upsampled);
        delete_framebuffers(gl, &mut self.fbo_upsampled);
    }

    pub unsafe fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) -> anyhow::Result<()> {
        if self.width == width && self.height == height {
            return Ok(());
        }
        self.width = width;
        self.height = height;
        self.destroy_targets(gl);
        if width == 0 || height == 0 {
            return Ok(());
        }

        let tex_blackhole = crate::texture::create_color_texture(gl, width, height)?;
        let fbo_blackhole = crate::framebuffer::create_framebuffer(gl, tex_blackhole)?;
        self.tex_blackhole = Some(tex_blackhole);
        self.fbo_blackhole = Some(fbo_blackhole);

        let tex_brightness = crate::texture::create_color_texture(gl, width, height)?;
        let fbo_brightness = crate::framebuffer::create_framebuffer(gl, tex_brightness)?;
        self.tex_brightness = Some(tex_brightness);
        self.fbo_brightness = Some(fbo_brightness);

        let tex_lens_flare = crate::texture::create_color_texture(gl, width, height)?;
        let fbo_lens_flare = crate::framebuffer::create_framebuffer(gl, tex_lens_flare)?;
        self.tex_lens_flare = Some(tex_lens_flare);
        self.fbo_lens_flare = Some(fbo_lens_flare);

        for i in 0..MAX_BLOOM_ITER {
            let mip_w = width >> (i + 1);
            let mip_h = height >> (i + 1);
            if mip_w == 0 || mip_h == 0 { break; }

            let tex_down = crate::texture::create_color_texture(gl, mip_w, mip_h)?;
            let fbo_down = crate::framebuffer::create_framebuffer(gl, tex_down)?;
            self.tex_downsampled.push(tex_down);
            self.fbo_downsampled.push(fbo_down);

            let tex_up_w = width >> i;
            let tex_up_h = height >> i;
            let tex_up = crate::texture::create_color_texture(gl, tex_up_w, tex_up_h)?;
            let fbo_up = crate::framebuffer::create_framebuffer(gl, tex_up)?;
            self.tex_upsampled.push(tex_up);
            self.fbo_upsampled.push(fbo_up);
        }

        let tex_bloom_final = crate::texture::create_color_texture(gl, width, height)?;
        let fbo_bloom_final = crate::framebuffer::create_framebuffer(gl, tex_bloom_final)?;
        self.tex_bloom_final = Some(tex_bloom_final);
        self.fbo_bloom_final = Some(fbo_bloom_final);

        let tex_tonemapped = crate::texture::create_color_texture(gl, width, height)?;
        let fbo_tonemapped = crate::framebuffer::create_framebuffer(gl, tex_tonemapped)?;
        self.tex_tonemapped = Some(tex_tonemapped);
        self.fbo_tonemapped = Some(fbo_tonemapped);

        Ok(())
    }

    pub unsafe fn render(&mut self, gl: &glow::Context, state: &AppState, time: f32) {
        let tex_blackhole = self.tex_blackhole.clone().expect("missing tex_blackhole");
        let fbo_blackhole = self.fbo_blackhole.clone().expect("missing fbo_blackhole");
        let tex_brightness = self.tex_brightness.clone().expect("missing tex_brightness");
        let fbo_brightness = self.fbo_brightness.clone().expect("missing fbo_brightness");
        let tex_lens_flare = self.tex_lens_flare.clone().expect("missing tex_lens_flare");
        let fbo_lens_flare = self.fbo_lens_flare.clone().expect("missing fbo_lens_flare");
        let tex_bloom_final = self.tex_bloom_final.clone().expect("missing tex_bloom_final");
        let fbo_bloom_final = self.fbo_bloom_final.clone().expect("missing fbo_bloom_final");
        let tex_tonemapped = self.tex_tonemapped.clone().expect("missing tex_tonemapped");
        let fbo_tonemapped = self.fbo_tonemapped.clone().expect("missing fbo_tonemapped");

        // Reset state that might be messed up by egui
        gl.disable(glow::SCISSOR_TEST);
        gl.disable(glow::BLEND);
        gl.disable(glow::CULL_FACE);

        let blackhole_uniforms = [
            ("time", time),
            ("mouseX", state.mouse_x),
            ("mouseY", state.mouse_y),
            ("cameraRoll", state.camera_roll),
            ("gravatationalLensing", flag(state.gravitational_lensing)),
            ("renderBlackHole", flag(state.render_black_hole)),
            ("mouseControl", flag(state.mouse_control)),
            ("fovScale", 1.0),
            ("frontView", flag(state.front_view)),
            ("topView", flag(state.top_view)),
            ("adiskEnabled", flag(state.adisk_enabled)),
            ("adiskParticle", flag(state.adisk_particle)),
            ("adiskDensityV", state.adisk_density_v),
            ("adiskDensityH", state.adisk_density_h),
            ("adiskHeight", state.adisk_height),
            ("adiskLit", state.adisk_lit),
            ("adiskNoiseLOD", state.adisk_noise_lod),
            ("adiskNoiseScale", state.adisk_noise_scale),
            ("adiskSpeed", state.adisk_speed),
            ("spin", state.spin),
        ];
        let blackhole_textures = [("colorMap", self.color_map)];
        let blackhole_textures_3d = [("noiseTex", self.noise_tex)];
        let blackhole_cubemaps = [("galaxy", self.galaxy_cubemap)];

        self.pass_blackhole.render(
            gl,
            Some(fbo_blackhole),
            self.width as i32,
            self.height as i32,
            &blackhole_uniforms,
            &blackhole_textures,
            &blackhole_textures_3d,
            &blackhole_cubemaps,
        );

        let brightness_textures = [("texture0", tex_blackhole)];
        self.pass_brightness.render(
            gl,
            Some(fbo_brightness),
            self.width as i32,
            self.height as i32,
            &[],
            &brightness_textures,
            &[],
            &[],
        );

        let flare_textures = [("texture0", tex_brightness)];
        self.pass_lens_flare.render(
            gl,
            Some(fbo_lens_flare),
            self.width as i32,
            self.height as i32,
            &[],
            &flare_textures,
            &[],
            &[],
        );

        let chain_len = self.tex_downsampled.len();
        for i in 0..chain_len {
            let src_tex = if i == 0 {
                tex_brightness
            } else {
                self.tex_downsampled[i - 1]
            };
            let dst_fbo = self.fbo_downsampled[i];
            let w = self.width >> (i + 1);
            let h = self.height >> (i + 1);

            let downsample_textures = [("texture0", src_tex)];
            self.pass_downsample.render(
                gl,
                Some(dst_fbo),
                w as i32,
                h as i32,
                &[],
                &downsample_textures,
                &[],
                &[],
            );
        }

        for i in (0..chain_len).rev() {
            let tex0 = if i == chain_len - 1 {
                self.tex_downsampled[i]
            } else {
                self.tex_upsampled[i + 1]
            };
            let tex1 = if i == 0 {
                tex_brightness
            } else {
                self.tex_downsampled[i - 1]
            };
            let dst_fbo = self.fbo_upsampled[i];
            let w = self.width >> i;
            let h = self.height >> i;

            let upsample_textures = [("texture0", tex0), ("texture1", tex1)];
            self.pass_upsample.render(
                gl,
                Some(dst_fbo),
                w as i32,
                h as i32,
                &[],
                &upsample_textures,
                &[],
                &[],
            );
        }

        let bloom_tex = if self.tex_upsampled.is_empty() {
            tex_blackhole
        } else {
            self.tex_upsampled[0]
        };
        let composite_textures = [
            ("texture0", tex_blackhole),
            ("texture1", bloom_tex),
            ("texture2", tex_lens_flare),
        ];
        let composite_uniforms = [
            ("tone", 1.0),
            ("bloomStrength", state.bloom_strength),
            ("flareStrength", state.flare_strength),
        ];
        self.pass_composite.render(
            gl,
            Some(fbo_bloom_final),
            self.width as i32,
            self.height as i32,
            &composite_uniforms,
            &composite_textures,
            &[],
            &[],
        );

        let tone_textures = [("texture0", tex_bloom_final)];
        let tone_uniforms = [
            ("tonemappingEnabled", flag(state.tonemapping_enabled)),
            ("gamma", state.gamma),
            ("time", time),
            ("chromaStrength", state.chroma_aberration),
            ("grainStrength", state.grain_strength),
            ("saturation", state.saturation),
        ];
        self.pass_tonemapping.render(
            gl,
            Some(fbo_tonemapped),
            self.width as i32,
            self.height as i32,
            &tone_uniforms,
            &tone_textures,
            &[],
            &[],
        );

        let pass_textures = [("texture0", tex_tonemapped)];
        self.pass_passthrough.render(
            gl,
            None,
            self.width as i32,
            self.height as i32,
            &[],
            &pass_textures,
            &[],
            &[],
        );
    }
}
