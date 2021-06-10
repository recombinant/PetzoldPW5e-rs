// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 4 - SysMets1
//
// The original source code copyright:
//
// SYSMETS1.C -- System Metrics Display Program No. 1
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate sys_mets_data;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use sys_mets_data::SYS_METRICS;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{
    GetTextMetricsW, SetTextAlign, TextOutW, TA_LEFT, TA_RIGHT, TA_TOP, TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC, GetMessageW,
    GetSystemMetrics, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW,
    ReleaseDC, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_CREATE, WM_DESTROY,
    WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, WHITE_BRUSH};

fn main() {
    let app_name = to_wstr("sys_mets1");

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

        let caption = to_wstr("Get System Metrics No. 1");
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
    static mut CAPS_WIDTH: c_int = 0;
    static mut CHAR_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;

    match message {
        WM_CREATE => {
            let hdc = GetDC(hwnd);
            let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();

            GetTextMetricsW(hdc, &mut tm);
            CHAR_WIDTH = tm.tmAveCharWidth;
            CAPS_WIDTH = (if tm.tmPitchAndFamily & 1 == 1 { 3 } else { 2 }) * CHAR_WIDTH / 2;
            CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

            ReleaseDC(hwnd, hdc);

            0 // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            for (u, sys_metric) in SYS_METRICS.iter().enumerate() {
                let i = u as c_int;

                SetTextAlign(hdc, TA_LEFT | TA_TOP);

                let label = to_wstr(sys_metric.label);
                TextOutW(
                    hdc,
                    0,
                    CHAR_HEIGHT * i,
                    label.as_ptr(),
                    lstrlenW(label.as_ptr()),
                );

                let desc = to_wstr(sys_metric.desc);
                TextOutW(
                    hdc,
                    22 * CAPS_WIDTH,
                    CHAR_HEIGHT * i,
                    desc.as_ptr(),
                    lstrlenW(desc.as_ptr()),
                );

                SetTextAlign(hdc, TA_RIGHT | TA_TOP);

                let metric = to_wstr(&format!("{:5}", GetSystemMetrics(sys_metric.index)));
                TextOutW(
                    hdc,
                    22 * CAPS_WIDTH + 40 * CHAR_WIDTH,
                    CHAR_HEIGHT * i,
                    metric.as_ptr(),
                    lstrlenW(metric.as_ptr()),
                );
            }

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
