use anyhow::Result;
use slog::{o, Drain, Logger};

fn create_logger(use_json: bool) -> Logger {
    let writer = std::io::stdout();

    let async_drain = if use_json {
        let json_drain = slog_json::Json::new(writer)
            .add_default_keys()
            .build()
            .fuse();

        slog_async::Async::default(json_drain).fuse()
    } else {
        let term_drain = slog_term::term_compact().fuse();

        slog_async::Async::default(term_drain).fuse()
    };

    // Create the required root logger
    Logger::root(async_drain, o!())
}

pub fn setup_logging(use_json: bool) -> Result<(Logger, slog_scope::GlobalLoggerGuard)> {
    let logger = create_logger(use_json);

    let global_logger = slog_scope::set_global_logger(logger.new(o!()));

    Ok((logger, global_logger))
}
