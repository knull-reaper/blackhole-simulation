
pub struct AppState {
    // Toggles
    pub gravitational_lensing: bool,
    pub render_black_hole: bool,
    pub mouse_control: bool,
    pub front_view: bool,
    pub top_view: bool,
    pub adisk_enabled: bool,
    pub adisk_particle: bool,
    pub tonemapping_enabled: bool,

    // Sliders
    pub camera_roll: f32, // -180 to 180
    pub adisk_density_v: f32,
    pub adisk_density_h: f32,
    pub adisk_height: f32,
    pub adisk_lit: f32,
    pub adisk_noise_lod: f32,
    pub adisk_noise_scale: f32,
    pub adisk_speed: f32,
    pub bloom_strength: f32,
    pub flare_strength: f32,
    pub chroma_aberration: f32,
    pub grain_strength: f32,
    pub saturation: f32,
    pub gamma: f32,
    pub spin: f32,

    // Mouse state
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            gravitational_lensing: true,
            render_black_hole: true,
            mouse_control: false,
            front_view: true,
            top_view: false,
            adisk_enabled: true,
            adisk_particle: true,
            tonemapping_enabled: true,

            camera_roll: -10.0,
            adisk_density_v: 2.0,
            adisk_density_h: 4.0,
            adisk_height: 0.55,
            adisk_lit: 0.20,
            adisk_noise_lod: 5.0,
            adisk_noise_scale: 0.8,
            adisk_speed: 0.5,
            bloom_strength: 0.08,
            flare_strength: 0.10,
            chroma_aberration: 0.003,
            grain_strength: 0.01,
            saturation: 1.30,
            gamma: 2.0,
            spin: 0.20,

            mouse_x: 400.0,
            mouse_y: 300.0,
        }
    }
}
