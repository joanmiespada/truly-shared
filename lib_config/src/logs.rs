
use std::sync::Once;

static LOGGER_INITIALIZED: Once = Once::new();


pub fn setup_log() {

    LOGGER_INITIALIZED.call_once(|| {
        env_logger::init();
        log::info!("Logger initialized");
    });

}
