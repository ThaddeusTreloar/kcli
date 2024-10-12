use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

pub fn init_logging() {
    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )])
    .unwrap();
}
