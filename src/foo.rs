use anyhow::Result;
use slog::{debug, error, info, o, trace, warn};

macro_rules! sl {
    () => {
        slog_scope::logger().new(o!("subsystem" => "foo"))
    };
}

pub fn init() -> Result<()> {
    debug!(sl!(), "a global debug message"; "a" => "1", "b" => "2", "c" => "33");
    info!(sl!(), "a global info message");
    warn!(sl!(), "a warning message"; "warning-var-1" => "warning-value1");
    trace!(sl!(), "a trace message"; "trace-var-1" => "trace-value1");
    error!(sl!(), "an error message"; "error-var-1" => "error-value1");

    Ok(())
}
