mod log;

fn main() {
    log::init();

    tracing::info!("Hello, world!");
}
