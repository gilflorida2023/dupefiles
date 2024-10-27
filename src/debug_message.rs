//! This module provides debugging utilities that can be conditionally compiled.
use std::arch::asm;
use std::fmt;

/// Prints a debug message to stderr when the "debug" feature is enabled.
/// 
/// This function is only available when the "debug" feature is enabled. It prints
/// the formatted message to stderr.
/// 
#[cfg(feature = "debug")]
pub fn debug_message(args: fmt::Arguments) {
    eprintln!("{}", args);
}

/// A no-op function that replaces debug_message when the "debug" feature is disabled.
/// 
/// This function is used when the "debug" feature is not enabled. It does nothing
/// except execute a no-op assembly instruction.
/// 
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
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        debug_message(format_args!($($arg)*))
    };
}