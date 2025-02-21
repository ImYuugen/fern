#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "debug") };
        log::info!("Launching Fern in debug mode, don't get banned!");
    }
    env_logger::init();
    fern_web::start_loop().await?;
    Ok(())
}
