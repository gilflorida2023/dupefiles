//! This module provides debugging utilities that can be conditionally compiled.
use std::arch::asm;
use std::fmt;

/// Prints a debug message to stderr when the "debug" feature is enabled.
/// 
/// This function is only available when the "debug" feature is enabled. It prints
/// the formatted message to stderr.
/// 
/// # Examples
/// 
/// ```
/// # #[cfg(feature = "debug")]
/// # {
/// use std::fmt;
/// //use dupefiles::log;
/// use dupefiles::debug_message::debug_message;
/// debug_message(fmt::Arguments::new_v1(&["Debug: Test message"], &[]));
/// // This will print "Debug: Test message" to stderr when the "debug" feature is enabled.
/// # }
/// ```
#[cfg(feature = "debug")]
pub fn debug_message(args: fmt::Arguments) {
    eprintln!("{}", args);
}

/// A no-op function that replaces debug_message when the "debug" feature is disabled.
/// 
/// This function is used when the "debug" feature is not enabled. It does nothing
/// except execute a no-op assembly instruction.
/// 
/// # Examples
/// 
/// ```
/// # #[cfg(not(feature = "debug"))]
/// # {
/// use std::fmt;
/// use dupefiles::debug_message::debug_message;
/// debug_message(fmt::Arguments::new_v1(&["This will not be printed"], &[]));
/// // This will not print anything, and will be optimized to a no-op.
/// # }
/// ```
#[cfg(not(feature = "debug"))]
pub fn debug_message(_args: fmt::Arguments) {
    unsafe {
        asm!("nop");
    }
}

/// Macro for easier debug message formatting.
/// 
/// This macro allows for easier formatting of debug messages. It uses the `debug_message`
/// function internally and supports format strings similar to `println!`.
/// 
/// # Examples
/// 
/// ```
/// # #[macro_use] extern crate your_crate_name;
/// use dupefiles::debug_message::debug_message;
/// use dupefiles::log;
/// # fn main() {
/// let x = 42;
/// log!("The value of x is: {}", x);
/// // When the "debug" feature is enabled, this will print:
/// // "The value of x is: 42" to stderr.
/// // When the "debug" feature is disabled, this will do nothing.
/// # }
/// ```
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        debug_message(format_args!($($arg)*))
    };
}