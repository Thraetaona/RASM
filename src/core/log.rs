//!
//!

pub use web_sys::console::*;

/// An asynchronous logging Macro that takes a variable number of arguments.
///
/// This macro will generically log with the specified log level.
/// For a full list of Console method's visit [here](https://developer.mozilla.org/en-US/docs/Web/API/Console).
///
/// It avoids all heap allocations, also formatting and everything else happens at compile time.
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
/// rasm::console!(log, "Condition (", 404 ,") met, exiting now.");
/// # }
/// ```
#[macro_export]
#[allow_internal_unstable(concat_idents)]
macro_rules! console {
    ($console_level: ident, $($args: tt)*) => ({
        // web_sys::console::${console_level}_1 (e.g log_1) will be used as web-sys's suffixed methods accept 'JsValue',
        // while unsuffixed (e.g log) ones expect 'Array'; 'str' can only be converted to 'JsValue' directly.
        concat_idents!($console_level, _1)(&stringify!(&format_args!(&$($args)*)).into())
    })
}