use web_sys::console;

/// An enum representing the available verbosity levels of the logger.
///
/// These variants correspond to HTML5 'Console' methods.
///
/// All messages with a LogLevel smaller than the
/// console LogLevel will be printed to the console.
pub enum LogLevel {
    /// The "error" level.
    ///
    /// Designates critical errors.
    Error = 0,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates informational conditions.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

// TODO: Choose a less confusing name for this backend.
pub async fn print(console_level: LogLevel, message: &str) {
    match console_level {
        LogLevel::Error => console::error_1(&format!("[ERROR] {}", message).into()),
        LogLevel::Warn => console::warn_1(&format!("[WARN] {}", message).into()),
        LogLevel::Info => console::info_1(&format!("[INFO] {}", message).into()),
        LogLevel::Debug => console::log_1(&format!("[DEBUG] {}", message).into()),
        LogLevel::Trace => console::trace_1(&format!("[TRACE] {}", message).into()),
    }
}

/// A logging Macro that takes a variable number of arguments.
///
/// This macro will generically log with the specified `LogLevel`.
///
/// # Examples
///
/// ```edition2018
/// use rasm::core::log::{log, LogLevel};
///
/// # fn main() {
/// 
/// # TODO: add a practical example.
///
/// log!(LogLevel::Error, "Shader initialization failed, Err Code: {}-{}", x, y);
/// # }
/// ```
#[macro_export]
macro_rules! log {
    ($console_level:expr, $($args:tt)*) => (print($console_level, &format_args!($($args)*).to_string()))
}