use glow::HasContext;

pub unsafe fn create_framebuffer(
    gl: &glow::Context,
    texture: glow::Texture,
) -> anyhow::Result<glow::Framebuffer> {
    let framebuffer = gl.create_framebuffer().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
    
    gl.framebuffer_texture_2d(
        glow::FRAMEBUFFER,
        glow::COLOR_ATTACHMENT0,
        glow::TEXTURE_2D,
        Some(texture),
        0,
    );

    if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
        anyhow::bail!("Framebuffer is not complete");
    }

    gl.bind_framebuffer(glow::FRAMEBUFFER, None);
    Ok(framebuffer)
}
