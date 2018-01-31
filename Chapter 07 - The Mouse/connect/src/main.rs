// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - SysMets
//
// The original source code copyright:
//
// CONNECT.C −− Connect−the−Dots Mouse Demo Program
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_int;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC, InvalidateRect, SetCursor, ShowCursor,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_DESTROY, WM_PAINT, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW, CS_VREDRAW, IDC_ARROW,
                          IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, MK_LBUTTON, IDC_WAIT, };
use winapi::um::wingdi::{MoveToEx, LineTo, SetPixel};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, TRUE, FALSE};
use winapi::shared::windef::{HWND, POINT};
use winapi::shared::ntdef::LPCWSTR;

use extras::{WHITE_BRUSH, to_wstr, GetStockBrush};


const MAX_POINTS: usize = 100;

fn main() {
    let app_name = to_wstr("connect");

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
            MessageBoxW(null_mut(),
                        to_wstr("This program requires Windows NT!").as_ptr(),
                        app_name.as_ptr(),
                        MB_ICONERROR);
            return; //   premature exit
        }

        let caption = to_wstr("Connect−the−Points Mouse Demo");
        let hwnd = CreateWindowExW(
            0,                 // dwExStyle:
            atom as LPCWSTR,   // lpClassName: class name or atom
            caption.as_ptr(),  // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW,  // dwStyle: window style
            CW_USEDEFAULT,     // x: initial x position
            CW_USEDEFAULT,     // y: initial y position
            CW_USEDEFAULT,     // nWidth: initial x size
            CW_USEDEFAULT,     // nHeight: initial y size
            null_mut(),        // hWndParent: parent window handle
            null_mut(),        // hMenu: window menu handle
            hinstance,         // hInstance: program instance handle
            null_mut());       // lpParam: creation parameters

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
    static mut POINT_ARRAY: [POINT; MAX_POINTS] = [POINT { x: 0, y: 0 }; MAX_POINTS];
    static mut POINT_COUNT: usize = 0;

    match message {
        WM_LBUTTONDOWN => {
            POINT_COUNT = 0;
            InvalidateRect(hwnd, null(), TRUE);
            0 as LRESULT  // message processed
        }

        WM_MOUSEMOVE => {
            let x: c_int = GET_X_LPARAM(lparam);
            let y: c_int = GET_Y_LPARAM(lparam);

            // Not Windows 98 these days.
            // Modern machines are too quick. Check for gap between pixels.
            // Original code relied on a slow processor giving a good pixel
            // Spacing.
            let ok = if POINT_COUNT > 0 {
                let x2 = POINT_ARRAY[POINT_COUNT - 1].x;
                let y2 = POINT_ARRAY[POINT_COUNT - 1].y;

                // Shortcut hypotenuse by not doing sqrt()
                (x2 - x) * (x2 - x) + (y2 - y) * (y2 - y) > 4900  // <-adjust this
            } else {
                true
            };
            // back to Windows 98

            if ok {
                if wparam & MK_LBUTTON != 0 && POINT_COUNT < MAX_POINTS {
                    POINT_ARRAY[POINT_COUNT] = POINT { x, y };
                    POINT_COUNT += 1;
                }
                let hdc = GetDC(hwnd);
                SetPixel(hdc, x, y, 0);
                ReleaseDC(hwnd, hdc);
            }

            0 as LRESULT  // message processed
        }

        WM_LBUTTONUP => {
            InvalidateRect(hwnd, null(), FALSE);
            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SetCursor(LoadCursorW(null_mut(), IDC_WAIT));
            ShowCursor(TRUE);

            if POINT_COUNT > 1 {
                for i in 0..POINT_COUNT - 1 {
                    for j in i + 1..POINT_COUNT {
                        MoveToEx(hdc, POINT_ARRAY[i].x, POINT_ARRAY[i].y, null_mut());
                        LineTo(hdc, POINT_ARRAY[j].x, POINT_ARRAY[j].y);
                    }
                }
            }

            ShowCursor(FALSE);
            SetCursor(LoadCursorW(null_mut(), IDC_ARROW));

            EndPaint(hwnd, &ps);
            0 as LRESULT  // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
