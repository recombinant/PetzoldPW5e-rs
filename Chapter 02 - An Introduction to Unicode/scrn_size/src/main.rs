// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 2 - ScrnSize
//
// The original source code copyright:
//
// ScrnSize.c -- Displays screen size in a message box
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{MessageBoxW, MB_OK,
                          GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};


fn to_wstr(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn main() {
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let text = to_wstr(&format!("The screen is {} pixels wide by {} pixels high.",
                                screen_width,
                                screen_height));

    let caption = to_wstr("scrn_size");


    unsafe {
        MessageBoxW(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}
