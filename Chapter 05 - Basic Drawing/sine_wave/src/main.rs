// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - SineWave
//
// The original source code copyright:
//
// SINEWAVE.C -- Sine Wave Using Polyline
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::f64::consts::PI;
use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HWND, POINT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{LineTo, MoveToEx, Polyline};
use winapi::um::winnt::LONG;
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetMessageW,
    LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_DESTROY, WM_PAINT, WM_SIZE,
    WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, WHITE_BRUSH};

//
const NUM: usize = 1000;

fn main() {
    let app_name = to_wstr("sine_wave");

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

        let caption = to_wstr("Sine Wave using Polyline");
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
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut SINE_POINTS: [POINT; NUM] = [POINT { x: 0, y: 0 }; NUM];

    match message {
        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            0 // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            MoveToEx(hdc, 0, CLIENT_HEIGHT / 2, null_mut());
            LineTo(hdc, CLIENT_WIDTH, CLIENT_HEIGHT / 2);

            for (u, point) in SINE_POINTS.iter_mut().enumerate() {
                let d = u as f64;

                point.x = (d * CLIENT_WIDTH as f64 / NUM as f64) as LONG;
                point.y = (CLIENT_HEIGHT as f64 / 2.0 * (1.0 - (PI * 2.0 * d / NUM as f64).sin()))
                    as LONG;
            }

            Polyline(hdc, &SINE_POINTS[0], SINE_POINTS.len() as c_int);

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
