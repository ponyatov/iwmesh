#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// pub mod web {

/// IP address
pub const ip: &str = "127.0.0.1";

/// TCP/IP port
pub const port: u16 = 12345;

/// socket bind address
pub const bind: &str = const_format::formatcp!("{}:{}", ip, port);

/// http:-headed string
pub const url: &str = const_format::formatcp!("http://{bind}");

// }
