//!
//!

use web_sys::console::*;

/// A logging Macro that takes a variable number of arguments.
///
/// This macro will generically log with the specified `LogLevel`.
/// It is similar to rust's [println](https://doc.rust-lang.org/std/macro.println.html) macro.
///
/// # Examples
///
/// ```
/// use rasm::core::log::*;
///
/// # fn main() {
/// 
/// rasm::console!(info, "1 + 3 = {}. It Works!", 4);
///
/// // TODO: add a practical example.
///
/// rasm::console!(error, "Shader initialization failed, Err Code: {}-{}", 123, 456);
///
/// rasm::console!(log, "Condition", 404 ,"met, exiting now.");
/// # }
/// ```
#[macro_export]
macro_rules! console {
    ($console_level: ident, $($args: tt)*) => ({
        async {
            // Macros do not support arbitrary placement of concatenated identifiers due to the way they expand, hence the use of constants.
            let method = concat_idents!($console_level, _1); // TODO: Make this an immutable variable with 'expression' type.
            // ${console_level}_1 (e.g log_1) will be used as web-sys's suffixed methods will accept 'JsValue',
            // while unprefixed ones expect 'Array'; 'str' can only be converted to 'JsValue'.
            method(&concat!($($args)*).into()); // TODO: check if this line can be written in a shorter and more efficient way.
        }
    })
}