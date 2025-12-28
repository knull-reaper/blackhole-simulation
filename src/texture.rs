#![allow(unsafe_op_in_unsafe_fn)]
use glow::HasContext;
use image::GenericImageView;
use std::path::Path;

use crate::noise_gen;

pub unsafe fn load_texture_2d(gl: &glow::Context, path: &str) -> anyhow::Result<glow::NativeTexture> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let data = img.to_rgba8();

    let texture = gl.create_texture().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA as i32,
        width as i32,
        height as i32,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&data),
    );

    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

    Ok(texture)
}

pub unsafe fn load_cubemap(gl: &glow::Context, dir_path: &str) -> anyhow::Result<glow::NativeTexture> {
    let texture = gl.create_texture().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_texture(glow::TEXTURE_CUBE_MAP, Some(texture));

    let faces = [
        ("right.png", glow::TEXTURE_CUBE_MAP_POSITIVE_X),
        ("left.png", glow::TEXTURE_CUBE_MAP_NEGATIVE_X),
        ("top.png", glow::TEXTURE_CUBE_MAP_POSITIVE_Y),
        ("bottom.png", glow::TEXTURE_CUBE_MAP_NEGATIVE_Y),
        ("front.png", glow::TEXTURE_CUBE_MAP_POSITIVE_Z),
        ("back.png", glow::TEXTURE_CUBE_MAP_NEGATIVE_Z),
    ];

    for (filename, target) in faces.iter() {
        let path = Path::new(dir_path).join(filename);
        let img = image::open(&path)?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();

        gl.tex_image_2d(
            *target,
            0,
            glow::RGBA as i32, // Source C++ uses RGB usually, but we loaded RGBA
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&data),
        );
    }

    gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_CUBE_MAP, glow::TEXTURE_WRAP_R, glow::CLAMP_TO_EDGE as i32);

    Ok(texture)
}

pub unsafe fn create_color_texture(gl: &glow::Context, width: u32, height: u32) -> anyhow::Result<glow::NativeTexture> {
    let texture = gl.create_texture().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    
    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGB16F as i32,
        width as i32,
        height as i32,
        0,
        glow::RGB,
        glow::FLOAT,
        None,
    );
    
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

    Ok(texture)
}

pub unsafe fn create_noise_texture_3d(gl: &glow::Context) -> anyhow::Result<glow::NativeTexture> {
    let size = noise_gen::NOISE_SIZE as i32;
    let data = noise_gen::generate_noise_3d();

    let texture = gl.create_texture().map_err(|e| anyhow::anyhow!(e))?;
    gl.bind_texture(glow::TEXTURE_3D, Some(texture));

    gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
    gl.tex_image_3d(
        glow::TEXTURE_3D,
        0,
        glow::R8 as i32,
        size,
        size,
        size,
        0,
        glow::RED,
        glow::UNSIGNED_BYTE,
        Some(&data),
    );

    gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
    gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
    gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
    gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_WRAP_R, glow::REPEAT as i32);
    gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 4);

    Ok(texture)
}
