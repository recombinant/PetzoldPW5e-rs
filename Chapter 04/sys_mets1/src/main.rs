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

#![cfg(windows)] extern crate winapi;
extern crate sys_mets;

use sys_mets::SYS_METRICS;
use std::mem;
use std::ptr::{null_mut, null};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC, GetSystemMetrics,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_CREATE, WM_DESTROY, WS_OVERLAPPEDWINDOW, WM_PAINT, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetStockObject, GetTextMetricsW, TextOutW, SetTextAlign,
                         TEXTMETRICW,
                         TA_LEFT, TA_RIGHT, TA_TOP, };
use winapi::um::winbase::lstrlenW;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE};
use winapi::shared::windef::{HWND, HBRUSH};
use winapi::shared::ntdef::LPCWSTR;

// There are some mismatches in winapi types between constants and their usage...
const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;


fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn main() {
    let app_name = to_wstring("sys_mets1");
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

        let caption = to_wstring("Get System Metrics No. 1");
        let hwnd = CreateWindowExW(
            0,                    // dwExStyle:
            atom as LPCWSTR,      // lpClassName: class name or atom
            caption.as_ptr(),     // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW,  // dwStyle: window style
            CW_USEDEFAULT,        // x: initial x position
            CW_USEDEFAULT,        // y: initial y position
            CW_USEDEFAULT,        // nWidth: initial x size
            CW_USEDEFAULT,        // nHeight: initial y size
            null_mut(),           // hWndParent: parent window handle
            null_mut(),           // hMenu: window menu handle
            hinstance,            // hInstance: program instance handle
            null_mut());          // lpParam: creation parameters

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
    static mut CX_CAPS: c_int = 0;
    static mut CX_CHAR: c_int = 0;
    static mut CY_CHAR: c_int = 0;

    match message {
        WM_CREATE => {
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

            for (u, sys_metric) in SYS_METRICS.iter().enumerate() {
                let i = u as c_int;

                SetTextAlign(hdc, TA_LEFT | TA_TOP);

                let label = to_wstring(sys_metric.label);
                TextOutW(hdc,
                         0,
                         CY_CHAR * i,
                         label.as_ptr(),
                         lstrlenW(label.as_ptr()));

                let desc = to_wstring(sys_metric.desc);
                TextOutW(hdc,
                         22 * CX_CAPS,
                         CY_CHAR * i,
                         desc.as_ptr(),
                         lstrlenW(desc.as_ptr()));

                SetTextAlign(hdc, TA_RIGHT | TA_TOP);

                let metric = to_wstring(&format!("{:5}", GetSystemMetrics(sys_metric.index)));
                TextOutW(hdc,
                         22 * CX_CAPS + 40 * CX_CHAR,
                         CY_CHAR * i,
                         metric.as_ptr(),
                         lstrlenW(metric.as_ptr()));
            }

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
