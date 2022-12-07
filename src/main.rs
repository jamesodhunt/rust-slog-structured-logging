use anyhow::Result;
use slog::{debug, error, info, o, trace, warn};
use std::env;
use std::process::exit;

mod bar;
mod foo;
mod logging;

// Convenience macro to obtain the scope logger
macro_rules! sl {
    () => {
        slog_scope::logger().new(o!("subsystem" => "main"))
    };
}

fn test_logging(use_json: bool) -> Result<()> {
    let (_logger, _guard) = logging::setup_logging(use_json)?;

    debug!(sl!(), "a global debug message"; "a" => "1", "b" => "2", "c" => "33");
    info!(sl!(), "a global info message");
    warn!(sl!(), "a warning message"; "warning-var-1" => "warning-value1");
    trace!(sl!(), "a trace message"; "trace-var-1" => "trace-value1");
    error!(sl!(), "an error message"; "error-var-1" => "error-value1");

    foo::init()?;
    bar::init()?;

    debug!(sl!(), "Done");

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    if args.len() < 2 {
        println!(
            "ERROR: {}: specify bool (true: json logging, false: terminal logging)",
            program_name
        );
        exit(1);
    }

    let use_json_str = &args[1];

    let use_json = use_json_str.parse::<bool>()?;

    test_logging(use_json)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        exit(1);
    }
}
