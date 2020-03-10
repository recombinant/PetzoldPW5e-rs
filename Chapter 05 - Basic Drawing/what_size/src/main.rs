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
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HDC, HWND, POINT, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{
    DPtoLP, GetTextMetricsW, RestoreDC, SaveDC, SetMapMode, SetViewportExtEx, SetWindowExtEx,
    TextOutW, TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect, GetDC,
    GetMessageW, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, ReleaseDC,
    ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_DESTROY, WM_PAINT, WM_SIZE,
    WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetStockBrush, GetStockFont, SelectFont, MM_ANISOTROPIC, MM_HIENGLISH, MM_HIMETRIC,
    MM_LOENGLISH, MM_LOMETRIC, MM_TEXT, MM_TWIPS, SYSTEM_FIXED_FONT, WHITE_BRUSH,
};

fn main() {
    let app_name = to_wstr("what_size");

    unsafe {
        let hinstance = GetModuleHandleW(null());

        let wndclassex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: GetStockBrush(WHITE_BRUSH),
            lpszClassName: app_name.as_ptr(),
            hIconSm: null_mut(),
            lpszMenuName: null(),
        };
        let atom = RegisterClassExW(&wndclassex);

        if atom == 0 {
            MessageBoxW(
                null_mut(),
                to_wstr("This program requires Windows NT!").as_ptr(),
                app_name.as_ptr(),
                MB_ICONERROR,
            );
            return; // premature exit
        }

        let caption = to_wstr("What Size is the Window?");
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
    static mut CHAR_WIDTH: c_int = 0;
    static mut CAPS_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;

    match message {
        WM_SIZE => {
            let hdc = GetDC(hwnd);
            let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();

            GetTextMetricsW(hdc, &mut tm);
            CHAR_WIDTH = tm.tmAveCharWidth;
            CAPS_WIDTH = (if tm.tmPitchAndFamily & 1 == 1 { 3 } else { 2 }) * CHAR_WIDTH / 2;
            CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

            ReleaseDC(hwnd, hdc);

            0 as LRESULT // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));

            SetMapMode(hdc, MM_ANISOTROPIC);
            SetWindowExtEx(hdc, 1, 1, null_mut());
            SetViewportExtEx(hdc, CHAR_WIDTH, CHAR_HEIGHT, null_mut());

            // TODO: move to const when Rust evolves
            let heading = to_wstr("Mapping Mode            Left   Right     Top  Bottom");
            let underln = to_wstr("------------            ----   -----     ---  ------");

            TextOutW(hdc, 1, 1, heading.as_ptr(), lstrlenW(heading.as_ptr()));
            TextOutW(hdc, 1, 2, underln.as_ptr(), lstrlenW(underln.as_ptr()));

            show(hwnd, hdc, 1, 3, MM_TEXT, &"TEXT (pixels)");
            show(hwnd, hdc, 1, 4, MM_LOMETRIC, &"LOMETRIC (.1 mm)");
            show(hwnd, hdc, 1, 5, MM_HIMETRIC, &"HIMETRIC (.01 mm)");
            show(hwnd, hdc, 1, 6, MM_LOENGLISH, &"LOENGLISH (.01 in)");
            show(hwnd, hdc, 1, 7, MM_HIENGLISH, &"HIENGLISH (.001 in)");
            show(hwnd, hdc, 1, 8, MM_TWIPS, &"TWIPS (1/1440 in)");

            EndPaint(hwnd, &ps);

            0 as LRESULT // message processed
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn show(
    hwnd: HWND,
    hdc: HDC,
    text_x: c_int,
    text_y: c_int,
    map_mode: c_int,
    map_mode_name: &str,
) {
    SaveDC(hdc);

    SetMapMode(hdc, map_mode);

    let mut rect: RECT = mem::MaybeUninit::uninit().assume_init();
    GetClientRect(hwnd, &mut rect);
    DPtoLP(hdc, &mut rect as *mut RECT as *mut POINT, 2);

    RestoreDC(hdc, -1);

    let buffer = to_wstr(&format!(
        "{:-20} {:7} {:7} {:7} {:7}",
        map_mode_name, rect.left, rect.right, rect.top, rect.bottom
    ));

    TextOutW(
        hdc,
        text_x,
        text_y,
        buffer.as_ptr(),
        lstrlenW(buffer.as_ptr()),
    );
}
