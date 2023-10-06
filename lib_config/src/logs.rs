
use std::sync::Once;

static LOGGER_INITIALIZED: Once = Once::new();


pub fn setup_log() {

    LOGGER_INITIALIZED.call_once(|| {
        env_logger::Builder::from_default_env()
            .format_timestamp_secs()
            //.filter(None, log::LevelFilter::Info)
            .format_indent(Some(2))
            .init();
        
        log::info!("Logger initialized");
    });

}
