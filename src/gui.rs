use crate::app_state::AppState;
use egui::{Context, Slider, Window};

pub struct Gui {
    // We can add local gui state here if needed
}

impl Gui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ctx: &Context, state: &mut AppState, fps: f32) {
        Window::new("Settings").show(ctx, |ui| {
            ui.label(format!("FPS: {:.1}", fps));
            ui.separator();

            ui.heading("Camera");
            ui.add(Slider::new(&mut state.camera_roll, -180.0..=180.0).text("Roll"));
            ui.checkbox(&mut state.mouse_control, "Mouse Control");
            ui.checkbox(&mut state.front_view, "Front View");
            ui.checkbox(&mut state.top_view, "Top View");

            ui.separator();
            ui.heading("Rendering");
            ui.checkbox(&mut state.render_black_hole, "Render Black Hole");
            ui.checkbox(&mut state.gravitational_lensing, "Gravitational Lensing");
            ui.checkbox(&mut state.tonemapping_enabled, "ACES Tonemapping");
            ui.add(Slider::new(&mut state.bloom_strength, 0.0..=1.0).text("Bloom Strength"));
            ui.add(Slider::new(&mut state.gamma, 0.1..=5.0).text("Gamma"));

            ui.separator();
            ui.heading("Black Hole");
            ui.add(Slider::new(&mut state.spin, 0.0..=1.0).text("Spin"));

            ui.separator();
            ui.heading("Cinematic");
            ui.add(Slider::new(&mut state.flare_strength, 0.0..=1.0).text("Flare Strength"));
            ui.add(Slider::new(&mut state.chroma_aberration, 0.0..=0.02).text("Chromatic Aberration"));
            ui.add(Slider::new(&mut state.grain_strength, 0.0..=0.05).text("Film Grain"));
            ui.add(Slider::new(&mut state.saturation, 0.5..=2.0).text("Saturation"));

            ui.separator();
            ui.heading("Accretion Disk");
            ui.checkbox(&mut state.adisk_enabled, "Enable Disk");
            ui.checkbox(&mut state.adisk_particle, "Particles");
            ui.add(Slider::new(&mut state.adisk_density_v, 0.1..=10.0).text("Density V"));
            ui.add(Slider::new(&mut state.adisk_density_h, 0.1..=10.0).text("Density H"));
            ui.add(Slider::new(&mut state.adisk_height, 0.0..=2.0).text("Height"));
            ui.add(Slider::new(&mut state.adisk_lit, 0.0..=2.0).text("Brightness"));
            ui.add(Slider::new(&mut state.adisk_speed, 0.0..=5.0).text("Speed"));
            ui.add(Slider::new(&mut state.adisk_noise_scale, 0.1..=5.0).text("Noise Scale"));
            ui.add(Slider::new(&mut state.adisk_noise_lod, 1.0..=10.0).text("Noise LOD"));

        });
    }
}
