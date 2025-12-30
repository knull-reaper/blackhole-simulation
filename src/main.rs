#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    blackhole_web::run_app()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
