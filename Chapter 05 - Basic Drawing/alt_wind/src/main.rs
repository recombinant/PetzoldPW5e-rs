// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - AltWind
//
// The original source code copyright:
//
// ALTWIND.C -- Alternate and Winding Fill Modes
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::{c_int};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_DESTROY, WM_PAINT, WM_SIZE,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetStockObject, SelectObject, Polygon, SetPolyFillMode,
                         ALTERNATE, WINDING, };
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, };
use winapi::shared::windef::{HWND, HBRUSH, POINT};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM, };

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, GRAY_BRUSH, to_wstring};


fn main() {
    let app_name = to_wstring("alt_wind");
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
            hbrBackground: GetStockBrush(WHITE_BRUSH),
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

        let caption = to_wstring("Alternate and Winding Fill Modes");
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
    const FIGURE_POINTS: [POINT; 10] = [
        POINT { x: 10, y: 70 }, POINT { x: 50, y: 70 }, POINT { x: 50, y: 10 },
        POINT { x: 90, y: 10 }, POINT { x: 90, y: 50 }, POINT { x: 30, y: 50 },
        POINT { x: 30, y: 90 }, POINT { x: 70, y: 90 }, POINT { x: 70, y: 30 },
        POINT { x: 10, y: 30 }
    ];
    static mut CX_CLIENT: c_int = 0;
    static mut CY_CLIENT: c_int = 0;

    match message {
        WM_SIZE => {
            CX_CLIENT = GET_X_LPARAM(lparam);
            CY_CLIENT = GET_Y_LPARAM(lparam);

            0 as LRESULT  // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectObject(hdc, GetStockObject(GRAY_BRUSH));

            // TODO: could use FIGURE_POINTS.len() when Rust evolves.
            let mut poly_points: [POINT; 10] = mem::uninitialized();

            for (figure_point, poly_point) in FIGURE_POINTS.iter().zip(poly_points.iter_mut()) {
                poly_point.x = CX_CLIENT * figure_point.x / 200;
                poly_point.y = CY_CLIENT * figure_point.y / 100;
            }

            SetPolyFillMode(hdc, ALTERNATE);
            Polygon(hdc, &poly_points[0], poly_points.len() as c_int);

            for poly_point in poly_points.iter_mut() {
                poly_point.x += CX_CLIENT / 2;
            }

            SetPolyFillMode(hdc, WINDING);
            Polygon(hdc, &poly_points[0], poly_points.len() as c_int);

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
