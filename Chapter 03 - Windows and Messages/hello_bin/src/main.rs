// Transliterated from Charles Petzold's Programming Windows
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 3 - HelloWin
//
// The original source code copyright:
//
// HelloWin.c -- Displays "Hello, Windows 98!" in client area
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate winapi;

use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HICON, HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::playsoundapi::{PlaySoundW, SND_ASYNC, SND_FILENAME};
use winapi::um::wingdi::GetStockObject;
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawTextW, EndPaint,
    GetClientRect, GetMessageW, LoadImageW, PostQuitMessage, RegisterClassExW, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, DT_CENTER,
    DT_SINGLELINE, DT_VCENTER, IDC_ARROW, IDI_APPLICATION, IMAGE_CURSOR, IMAGE_ICON, LR_SHARED,
    MSG, PAINTSTRUCT, SW_SHOW, WM_CREATE, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some mismatches in winapi types between constants and their usage...
const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;

// This performs the conversion from Rust str to Windows WSTR
// Use this function to convert and then use its returned value's .as_ptr()
// method to get the LPWSTR.
pub fn to_wstr(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}

fn main() {
    let app_name = to_wstr("hello_win");

    unsafe {
        let hinstance = GetModuleHandleW(null());

        let wndclassex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadImageW(
                hinstance,
                IDI_APPLICATION, // name
                IMAGE_ICON,      // type
                0,
                0,
                LR_SHARED,
            ) as HICON, // cx, cy, fuLoad
            hCursor: LoadImageW(hinstance, IDC_ARROW, IMAGE_CURSOR, 0, 0, LR_SHARED) as HICON,
            hbrBackground: GetStockObject(WHITE_BRUSH) as HBRUSH,
            lpszClassName: app_name.as_ptr(),
            hIconSm: null_mut(),
            lpszMenuName: null(),
        };
        let atom = RegisterClassExW(&wndclassex);

        if atom == 0 {
            return; // premature exit
        }

        let caption = to_wstr("The Hello Program");
        let hwnd = CreateWindowExW(
            0,                   // dwExStyle:
            atom as LPCWSTR,     // lpClassName: class name or atom
            caption.as_ptr(),    // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW, // dwStyle: window style
            CW_USEDEFAULT,       // x: initial x position
            CW_USEDEFAULT,       // y: initial y position
            CW_USEDEFAULT,       // nWidth: initial x size
            CW_USEDEFAULT,       // nHeight: initial y size
            null_mut(),          // hWndParent: parent window handle
            null_mut(),          // hMenu: window menu handle
            hinstance,           // hInstance: program instance handle
            null_mut(),
        ); // lpParam: creation parameters

        if hwnd.is_null() {
            return; // premature exit
        }

        ShowWindow(hwnd, SW_SHOW);
        if UpdateWindow(hwnd) == 0 {
            return; // premature exit
        }

        let mut msg: MSG = mem::MaybeUninit::uninit().assume_init();

        loop {
            // three states: -1, 0 or non-zero
            let ret = GetMessageW(&mut msg, null_mut(), 0, 0);

            if ret == -1 {
                // handle the error and/or exit
                // for error call GetLastError();
                return;
            } else if ret == 0 {
                break;
            } else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        // return msg.wParam;  // WM_QUIT
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_CREATE => {
            // This file path is a hack. It works with "cargo run"
            let mut path = env::current_dir().unwrap();
            path.push("hello_win.wav");

            let sound: Vec<u16> = path.into_os_string().encode_wide().chain(once(0)).collect();
            PlaySoundW(sound.as_ptr(), null_mut(), SND_FILENAME | SND_ASYNC);
            0 // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rect: RECT = mem::MaybeUninit::uninit().assume_init();
            GetClientRect(hwnd, &mut rect);

            DrawTextW(
                hdc,
                to_wstr("Hello, Windows 98!").as_ptr(),
                -1,
                &mut rect,
                DT_SINGLELINE | DT_CENTER | DT_VCENTER,
            );

            EndPaint(hwnd, &ps);
            0 // message processed
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
