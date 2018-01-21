// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - Bezier
//
// The original source code copyright:
//
// BEZIER.C -- Bezier Splines Demo
//             (c) Charles Petzold, 1998
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
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, InvalidateRect,
                          GetDC, ReleaseDC,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_DESTROY, WM_PAINT, WM_SIZE, WM_LBUTTONDOWN, WM_RBUTTONDOWN,
                          WM_MOUSEMOVE, MK_RBUTTON, MK_LBUTTON,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetStockObject, SelectObject, MoveToEx, LineTo, PolyBezier};
use winapi::shared::minwindef::{UINT, DWORD, WPARAM, LPARAM, LRESULT, HINSTANCE, LOWORD, HIWORD,
                                TRUE};
use winapi::shared::windef::{HWND, HBRUSH, POINT, HDC};
use winapi::shared::ntdef::{LPCWSTR, LONG};

// There are some mismatches in winapi types between constants and their usage...
const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;
const WHITE_PEN: c_int = winapi::um::wingdi::WHITE_PEN as c_int;
const BLACK_PEN: c_int = winapi::um::wingdi::BLACK_PEN as c_int;


fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn main() {
    let app_name = to_wstring("bezier");
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

        let caption = to_wstring("Bezier Splines");
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
    static mut CX_CLIENT: c_int = 0;
    static mut CY_CLIENT: c_int = 0;
    static mut BEZIER_POINTS: [POINT; 4] = [POINT { x: 0, y: 0 }; 4];

    match message {
        WM_SIZE => {
            CX_CLIENT = LOWORD(lparam as DWORD) as c_int;
            CY_CLIENT = HIWORD(lparam as DWORD) as c_int;

            BEZIER_POINTS = [
                POINT {
                    x: CX_CLIENT / 4,
                    y: CY_CLIENT / 2,
                },
                POINT {
                    x: CX_CLIENT / 2,
                    y: CY_CLIENT / 4,
                },
                POINT {
                    x: CX_CLIENT / 2,
                    y: 3 * CY_CLIENT / 4,
                },
                POINT {
                    x: 3 * CX_CLIENT / 4,
                    y: CY_CLIENT / 2,
                }, ];

            0 as LRESULT  // message processed
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MOUSEMOVE => {
            if (wparam & MK_LBUTTON) != 0 || (wparam & MK_RBUTTON) != 0 {
                let hdc = GetDC(hwnd);

                SelectObject(hdc, GetStockObject(WHITE_PEN));
                draw_bezier(hdc, &BEZIER_POINTS);

                if (wparam & MK_LBUTTON) != 0 {
                    BEZIER_POINTS[1].x = LOWORD(lparam as DWORD) as LONG;
                    BEZIER_POINTS[1].y = HIWORD(lparam as DWORD) as LONG;
                }
                if (wparam & MK_RBUTTON) != 0 {
                    BEZIER_POINTS[2].x = LOWORD(lparam as DWORD) as LONG;
                    BEZIER_POINTS[2].y = HIWORD(lparam as DWORD) as LONG;
                }

                SelectObject(hdc, GetStockObject(BLACK_PEN));
                draw_bezier(hdc, &BEZIER_POINTS);
                ReleaseDC(hwnd, hdc);
            }

            0 as LRESULT  // message processed
        }
        WM_PAINT => {
            InvalidateRect(hwnd, null_mut(), TRUE);

            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            draw_bezier(hdc, &BEZIER_POINTS);

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


unsafe fn draw_bezier(hdc: HDC, points: &[POINT; 4]) {
    PolyBezier(hdc, &points[0], 4);
    MoveToEx(hdc, points[0].x, points[0].y, null_mut());
    LineTo(hdc, points[1].x, points[1].y);
    MoveToEx(hdc, points[2].x, points[2].y, null_mut());
    LineTo(hdc, points[3].x, points[3].y);
}
