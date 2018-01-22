// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - WhatSize
//
// The original source code copyright:
//
// WHATSIZE.C -- What Size is the Window?
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)] extern crate winapi;

use std::mem;
use std::ptr::{null_mut, null};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::{c_int};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC, GetClientRect,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_DESTROY, WM_PAINT, WM_SIZE,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetStockObject, SelectObject, GetTextMetricsW, TextOutW, SetMapMode,
                         SetWindowExtEx, SetViewportExtEx, SaveDC, RestoreDC, DPtoLP,
                         TEXTMETRICW,
};
use winapi::um::winbase::lstrlenW;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE};
use winapi::shared::windef::{HWND, HBRUSH, RECT, POINT, HDC};
use winapi::shared::ntdef::LPCWSTR;

// There are some mismatches in winapi types between constants and their usage...
const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;
const SYSTEM_FIXED_FONT: c_int = winapi::um::wingdi::SYSTEM_FIXED_FONT as c_int;
const MM_ANISOTROPIC: c_int = winapi::um::wingdi::MM_ANISOTROPIC as c_int;
const MM_TEXT: c_int = winapi::um::wingdi::MM_TEXT as c_int;
const MM_LOMETRIC: c_int = winapi::um::wingdi::MM_LOMETRIC as c_int;
const MM_HIMETRIC: c_int = winapi::um::wingdi::MM_HIMETRIC as c_int;
const MM_LOENGLISH: c_int = winapi::um::wingdi::MM_LOENGLISH as c_int;
const MM_HIENGLISH: c_int = winapi::um::wingdi::MM_HIENGLISH as c_int;
const MM_TWIPS: c_int = winapi::um::wingdi::MM_TWIPS as c_int;


fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn main() {
    let app_name = to_wstring("what_size");
    let hinstance = 0 as HINSTANCE;

    unsafe {
        let wndclassex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: GetStockObject(WHITE_BRUSH) as HBRUSH,
            lpszClassName: app_name.as_ptr(),
            hIconSm: null_mut(),
            lpszMenuName: null(),
        };
        let atom = RegisterClassExW(&wndclassex);

        if atom == 0 {
            MessageBoxW(null_mut(),
                        to_wstring("This program requires Windows NT!").as_ptr(),
                        app_name.as_ptr(),
                        MB_ICONERROR);
            return; //   premature exit
        }

        let caption = to_wstring("What Size is the Window?");
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
            null_mut());         // lpParam: creation parameters

        if hwnd.is_null() {
            return;  // premature exit
        }

        ShowWindow(hwnd, SW_SHOW);
        if UpdateWindow(hwnd) == 0 {
            return;  // premature exit
        }

        let mut msg: MSG = mem::uninitialized();

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


unsafe extern "system" fn wnd_proc(hwnd: HWND,
                                   message: UINT,
                                   wparam: WPARAM,
                                   lparam: LPARAM)
                                   -> LRESULT {
    static mut CX_CHAR: c_int = 0;
    static mut CX_CAPS: c_int = 0;
    static mut CY_CHAR: c_int = 0;

    match message {
        WM_SIZE => {
            let hdc = GetDC(hwnd);
            let mut tm: TEXTMETRICW = mem::uninitialized();

            GetTextMetricsW(hdc, &mut tm);
            CX_CHAR = tm.tmAveCharWidth;
            CX_CAPS = (if tm.tmPitchAndFamily & 1 == 1 { 3 } else { 2 }) * CX_CHAR / 2;
            CY_CHAR = tm.tmHeight + tm.tmExternalLeading;

            ReleaseDC(hwnd, hdc);

            0 as LRESULT  // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectObject(hdc, GetStockObject(SYSTEM_FIXED_FONT));

            SetMapMode(hdc, MM_ANISOTROPIC);
            SetWindowExtEx(hdc, 1, 1, null_mut());
            SetViewportExtEx(hdc, CX_CHAR, CY_CHAR, null_mut());

            // TODO: move to const when Rust evolves
            let heading = to_wstring("Mapping Mode            Left   Right     Top  Bottom");
            let underln = to_wstring("------------            ----   -----     ---  ------");

            TextOutW(hdc, 1, 1, heading.as_ptr(), lstrlenW(heading.as_ptr()));
            TextOutW(hdc, 1, 2, underln.as_ptr(), lstrlenW(underln.as_ptr()));

            show(hwnd, hdc, 1, 3, MM_TEXT, &"TEXT (pixels)");
            show(hwnd, hdc, 1, 4, MM_LOMETRIC, &"LOMETRIC (.1 mm)");
            show(hwnd, hdc, 1, 5, MM_HIMETRIC, &"HIMETRIC (.01 mm)");
            show(hwnd, hdc, 1, 6, MM_LOENGLISH, &"LOENGLISH (.01 in)");
            show(hwnd, hdc, 1, 7, MM_HIENGLISH, &"HIENGLISH (.001 in)");
            show(hwnd, hdc, 1, 8, MM_TWIPS, &"TWIPS (1/1440 in)");

            EndPaint(hwnd, &mut ps);

            0 as LRESULT  // message processed
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}


unsafe fn show(hwnd: HWND,
               hdc: HDC,
               text_x: c_int,
               text_y: c_int,
               map_mode: c_int,
               map_mode_name: &str) {
    SaveDC(hdc);

    SetMapMode(hdc, map_mode);

    let mut rect: RECT = mem::uninitialized();
    GetClientRect(hwnd, &mut rect);
    DPtoLP(hdc, &mut rect as *mut RECT as *mut POINT, 2);

    RestoreDC(hdc, -1);

    let buffer = to_wstring(&format!("{:-20} {:7} {:7} {:7} {:7}",
                                     map_mode_name,
                                     rect.left, rect.right,
                                     rect.top, rect.bottom));

    TextOutW(hdc, text_x, text_y, buffer.as_ptr(), lstrlenW(buffer.as_ptr()));
}
