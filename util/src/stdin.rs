use std::io::prelude::*;
use std::io::stdin;

use anyhow::Result;

pub fn stdin_read() -> Result<String> {
    let mut s = String::new();
    stdin().lock().read_to_string(&mut s)?;
    Ok(s)
}

pub fn stdin_lines() -> impl Iterator<Item = String> {
    stdin().lock().lines().map_while(Result::ok)
}

pub fn stdin_lines_u8() -> impl Iterator<Item = Vec<u8>> {
    stdin().lock().split(b'\n').map_while(Result::ok)
}
