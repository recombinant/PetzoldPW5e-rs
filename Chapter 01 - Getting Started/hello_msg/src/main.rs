// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 1 - HelloMsg
//
// The original source code copyright:
//
// HelloMsg.c -- Displays "Hello, Windows 98!" in a message box
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)] extern crate winapi;

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{MB_OK, MessageBoxW};


fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}

fn main() {
    let text = to_wstring("Hello, Windows 98!");
    let caption = to_wstring("hello_msg");

    unsafe {
        MessageBoxW(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}
