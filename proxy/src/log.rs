use serde::Serialize;
use std::io::{self, Write};

use crate::lsp::encode_lsp;

mod message_type {
    pub const ERROR: u8 = 1;
    pub const WARNING: u8 = 2;
    pub const INFO: u8 = 3;
    pub const LOG: u8 = 4;
    pub const DEBUG: u8 = 5;
}

#[derive(Serialize)]
struct LogMessageNotification<'a> {
    jsonrpc: &'static str,
    method: &'static str,
    params: LogMessageParams<'a>,
}

#[derive(Serialize)]
struct LogMessageParams<'a> {
    r#type: u8,
    message: &'a str,
}

fn send_log_message(level: u8, message: &str) {
    let notification = LogMessageNotification {
        jsonrpc: "2.0",
        method: "window/logMessage",
        params: LogMessageParams {
            r#type: level,
            message,
        },
    };

    let encoded = encode_lsp(&notification);

    let stdout = io::stdout();
    let mut w = stdout.lock();
    let _ = w.write_all(encoded.as_bytes());
    let _ = w.flush();
}

pub fn error(message: &str) {
    send_log_message(message_type::ERROR, message);
}

pub fn warn(message: &str) {
    send_log_message(message_type::WARNING, message);
}

pub fn info(message: &str) {
    send_log_message(message_type::INFO, message);
}

pub fn log(message: &str) {
    send_log_message(message_type::LOG, message);
}

pub fn debug(message: &str) {
    send_log_message(message_type::DEBUG, message);
}

#[macro_export]
macro_rules! lsp_error {
    ($($arg:tt)*) => {
        $crate::log::error(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_warn {
    ($($arg:tt)*) => {
        $crate::log::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_info {
    ($($arg:tt)*) => {
        $crate::log::info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_log {
    ($($arg:tt)*) => {
        $crate::log::log(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_debug {
    ($($arg:tt)*) => {
        $crate::log::debug(&format!($($arg)*))
    };
}
