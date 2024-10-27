use std::arch::asm;
use std::fmt;
//pub use crate::log;
//use crate::debug_message::debug_message;
#[cfg(feature = "debug")]
pub fn debug_message(args: fmt::Arguments) {
    eprintln!("{}", args);
}

#[cfg(not(feature = "debug"))]
pub fn debug_message(_args: fmt::Arguments) {
    unsafe {
        asm!("nop");
    }
}

// Macro to make it easier to use debug_message with format strings
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        debug_message(format_args!($($arg)*))
    };
}