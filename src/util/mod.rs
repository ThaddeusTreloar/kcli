use simplelog::{ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};


pub(crate) fn init_logging() {
    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Info, 
                config, 
                TerminalMode::Stderr, 
                ColorChoice::Auto
            ),
        ]
    ).unwrap();
}