use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

pub fn init_logging(log_level: LevelFilter) {
    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )])
    .unwrap();
}
