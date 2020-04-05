//!
//!


pub use web_sys::console::*;


/// A fast (and soon to be Asynchronous) logging Macro that takes a variable number of arguments.
///
/// This macro will generically log with the specified log level.
/// For a full list of Console method's visit [here](https://developer.mozilla.org/en-US/docs/Web/API/Console).
///
/// This macro is a perfect example of RASM's minimalism and efficiency.
/// It does not need to be initialized, filtered based on log level or sacrifice runtime performance to print exact timestamps.
/// Instead, it relies on the browser's DevTools to implement such things;
/// a good browser is probably going to have its own verbosity filter and the ability to print precise timestamps.
/// This simplifies the codebase, prevents redundancy and does not hinder performance.
///
/// Also, all the complex formatting happens at compile time.
///
/// # Examples
///
/// ```
/// use rasm::core::log::*;
///
/// # fn main() {
/// 
/// rasm::console!(info, "1 + 3 = {}", 1 + 3);
///
/// // TODO: add a more practical example that is related to Gamedev.
///
/// rasm::console!(error, "Shader initialization failed, Err Code: {}-{}", 123, 456);
///
/// rasm::console!(log, "Condition {} met, exiting now.", 404);
/// # }
/// ```
#[macro_export]
#[allow_internal_unstable(concat_idents)]
macro_rules! console {
    ($console_level: ident, $($args: tt)*) => ( /*async*/ {
        let mut message: String = String::new();
        std::fmt::Write::write_fmt(&mut message, format_args!($($args)*));
        
        // web_sys::console::${console_level}_1 (e.g log_1) will be used as web-sys's suffixed methods accept 'JsValue',
        // while unsuffixed (e.g log) ones expect 'Array'; 'String' can only be converted to 'JsValue' directly.
        concat_idents!($console_level, _1)(&message.into());
    })
}