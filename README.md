# Blackhole (Rust Lang)

Real-time black hole renderer in Rust with interactive tuning, Kerr-style visual approximation, and a cinematic post pipeline.

## Preview

![Blackhole Rust Preview](./blackhole_recorded.gif)

## Highlights

- Adaptive ray marching for high FPS in empty space with tight detail near the horizon.
- Precomputed 3D noise volume for the accretion disk (fast, consistent, no shader noise cost).
- Kerr-style visuals: frame dragging twist, asymmetric horizon, and transverse acceleration via a `spin` control.
- Cinematic post: bloom, anamorphic lens flare, chromatic aberration, film grain, and saturation.
- Live UI controls via `egui` for instant look dev.

## What This Is

This is a stylized, real-time renderer built for speed and cinematic output. It uses fast ray marching and a set of controlled approximations to evoke Kerr-like behavior without full geodesic integration. The goal is visual fidelity at 60+ FPS, with artist-friendly controls for look development.

## How It Works

- **Ray march + lensing** (`shader/blackhole_main.frag`):
  - Adaptive step sizing: large steps far away, small steps near the horizon.
  - Gravitational lensing via a lightweight acceleration term.
  - Kerr-style approximation:
    - Frame dragging twist: longitude shift proportional to `spin / radius`.
    - Asymmetric horizon: "D-shaped" hit radius based on view direction.
    - Transverse acceleration: a small lateral push based on `cross(dir, spin_axis)`.
- **Accretion disk noise** (`src/noise_gen.rs`, `src/texture.rs`):
  - 3D simplex noise generated once on CPU and uploaded as a 3D texture.
  - Shader samples the 3D noise volume instead of evaluating noise per step.
- **Post pipeline** (`src/renderer.rs`):
  - Brightness pass -> lens flare -> bloom down/upsample -> composite -> tonemapping.
  - Lens flare is a horizontal blur with a cyan tint (`shader/lens_flare.frag`).
  - Tonemapping adds chromatic aberration, grain, and saturation (`shader/tonemapping.frag`).

## Visual Targets

- Crisp inner ring, smooth lensing arcs, and a disk that feels energetic without losing detail.
- HDR-style bloom and flare that emphasize highlights, not the whole frame.
- Color preserved through ACES tonemapping with controllable saturation.

## Controls

- Camera: Front View, Top View, Mouse Control, and Roll.
- Rendering: Black Hole toggle, Gravitational Lensing, ACES Tonemapping, Bloom Strength, Gamma.
- Black Hole: Spin.
- Cinematic: Flare Strength, Chromatic Aberration, Film Grain, Saturation.
- Accretion Disk: Enable, Particles, Density, Height, Brightness, Noise, Speed.

## Build and Run

Prerequisites:

- Rust (install via https://rustup.rs)
- OpenGL 3.3+ compatible GPU

Run:

```sh
cargo run --release
```

## Web (Wasm)

Build the web bundle:

```powershell
.\scripts\build-web.ps1
```

Serve the `web/` directory (any static server) and open `http://localhost:8000/`.

## Releases

If you want a prebuilt binary, grab the latest release from the project Releases page.

## Future Plans

- Resolution Scaling ("DLSS at Home"):
  - Render the black hole pass at 50% resolution.
  - Render the star field and UI at full resolution.
  - Upscale the black hole texture during composition (bilinear or bicubic).
  - Expected gain: 2x-3x FPS on lower-end hardware.
- Blue Noise Dithering:
  - Replace standard random noise with a blue-noise texture to reduce banding.
  - Push noise into high frequencies for cleaner integration with TAA or simple blending.
- "Reference" Mode (Debug Comparison):
  - Debug toggle to switch between the visual approximation and a slow RK4 integration.
  - Helps validate how close the approximation is to a ground-truth path.

## Project Structure

- `src/`: Rust source code
- `shader/`: GLSL shaders
- `assets/`: textures and cubemaps

## References

These references informed the Kerr-style approximations and terminology used here:

- Kerr metric (rotating black hole solution): https://en.wikipedia.org/wiki/Kerr_metric
- Frame dragging: https://en.wikipedia.org/wiki/Frame-dragging
- Lense-Thirring precession: https://en.wikipedia.org/wiki/Lense-Thirring_precession
- Boyer-Lindquist coordinates: https://en.wikipedia.org/wiki/Boyer-Lindquist_coordinates

## Credits

Inspired by the classic real-time black hole rendering work by Ross Ning, with a full Rust rewrite and substantial visual and performance upgrades.

## Feedback

I'm actively learning Rust, so if you spot issues or have suggestions, please open an issue. I'll do my best to fix things quickly.
