#![allow(unsafe_op_in_unsafe_fn)]
use glutin::context::ContextAttributesBuilder;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::{DisplayBuilder, GlWindow};
use glow::HasContext;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use std::ffi::CString;
use std::time::Instant;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use std::sync::Arc;

mod render_pass;
mod render_utils;
mod shader_utils;
mod texture;
mod noise_gen;
mod app_state;
mod framebuffer;
mod renderer;
mod gui;
use renderer::Renderer;

const SCR_WIDTH: u32 = 1200;
const SCR_HEIGHT: u32 = 800;

fn resize_surface(
    gl_surface: &glutin::surface::Surface<glutin::surface::WindowSurface>,
    gl_context: &glutin::context::PossiblyCurrentContext,
    size: PhysicalSize<u32>,
) {
    let width = NonZeroU32::new(size.width.max(1)).unwrap();
    let height = NonZeroU32::new(size.height.max(1)).unwrap();
    let _ = gl_surface.resize(gl_context, width, height);
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new().unwrap();
    
    // Get primary detector for fullscreen
    let monitor = event_loop.primary_monitor();

    let window_builder = WindowBuilder::new()
        .with_title("Blackhole Rust")
        .with_inner_size(PhysicalSize::new(SCR_WIDTH, SCR_HEIGHT))
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(monitor)));

    let template = glutin::config::ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(false);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder.build(&event_loop, template, |configs| {
        configs
            .reduce(|accum, config| {
                if config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap()
    })
    .map_err(|e| anyhow::anyhow!("Display builder error: {}", e))?;

    let window = window.ok_or(anyhow::anyhow!("Failed to create window"))?;
    let raw_window_handle = window.raw_window_handle();
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("failed to create context")
    };

    let attrs = window.build_surface_attributes(glutin::surface::SurfaceAttributesBuilder::new());
    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .unwrap();

    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            gl_display.get_proc_address(&CString::new(s).unwrap()) as *const _
        })
    };
    
    // Wrap gl in Arc to share with egui
    let gl = Arc::new(gl);

    let mut egui_glow = egui_glow::EguiGlow::new(&event_loop, gl.clone(), None, None);

    let mut window_size = window.inner_size();
    
    let mut renderer = unsafe { Renderer::new(&gl, window_size.width, window_size.height)? };

    let start_time = Instant::now();
    let mut app_state = app_state::AppState::default();
    
    resize_surface(&gl_surface, &gl_context, window_size);

    app_state.mouse_x = window_size.width as f32 * 0.5;
    app_state.mouse_y = window_size.height as f32 * 0.5;

    unsafe {
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut fps_display = 60.0;
    let mut gui_state = gui::Gui::new();

    event_loop.run(move |event, window_target| {
        window_target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                // Let egui handle the event first
                let response = egui_glow.on_window_event(&window, &event);
                if response.consumed {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::Resized(size) => {
                        window_size = size;
                        resize_surface(&gl_surface, &gl_context, size);
                        unsafe {
                            renderer.resize(&gl, size.width, size.height).unwrap();
                        }
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        window_size = window.inner_size();
                        resize_surface(&gl_surface, &gl_context, window_size);
                        unsafe {
                            renderer.resize(&gl, window_size.width, window_size.height).unwrap();
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        // Only update mouse pos if we aren't hovering UI
                        if !egui_glow.egui_ctx.is_pointer_over_area() {
                            app_state.mouse_x = position.x as f32;
                            app_state.mouse_y = position.y as f32;
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        if window_size.width == 0 || window_size.height == 0 {
                            return;
                        }

                        frame_count += 1;
                        let now = Instant::now();
                        let duration = now.duration_since(last_fps_update).as_secs_f32();
                        if duration >= 0.5 {
                            fps_display = frame_count as f32 / duration;
                            last_fps_update = now;
                            frame_count = 0;
                        }

                        let time = start_time.elapsed().as_secs_f32();
                        
                        unsafe {
                             renderer.render(&gl, &app_state, time);
                        }

                        // Render GUI
                        egui_glow.run(&window, |ctx| {
                            gui_state.ui(ctx, &mut app_state, fps_display);
                        });
                        egui_glow.paint(&window);
                        
                        gl_surface.swap_buffers(&gl_context).unwrap();
                    }
                    _ => (),
                }
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    })
    .map_err(|e| anyhow::anyhow!("Event loop error: {}", e))?;

    Ok(())
}
