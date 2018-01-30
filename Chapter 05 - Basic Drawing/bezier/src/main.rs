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

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
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
use winapi::um::wingdi::{MoveToEx, LineTo, PolyBezier};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, TRUE};
use winapi::shared::windef::{HWND, POINT, HDC};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, WHITE_PEN, BLACK_PEN, to_wstr, SelectPen, GetStockPen, GetStockBrush};


fn main() {
    let app_name = to_wstr("bezier");
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
                        to_wstr("This program requires Windows NT!").as_ptr(),
                        app_name.as_ptr(),
                        MB_ICONERROR);
            return; //   premature exit
        }

        let caption = to_wstr("Bezier Splines");
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
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut BEZIER_POINTS: [POINT; 4] = [POINT { x: 0, y: 0 }; 4];

    match message {
        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            BEZIER_POINTS = [
                POINT {
                    x: CLIENT_WIDTH / 4,
                    y: CLIENT_HEIGHT / 2,
                },
                POINT {
                    x: CLIENT_WIDTH / 2,
                    y: CLIENT_HEIGHT / 4,
                },
                POINT {
                    x: CLIENT_WIDTH / 2,
                    y: 3 * CLIENT_HEIGHT / 4,
                },
                POINT {
                    x: 3 * CLIENT_WIDTH / 4,
                    y: CLIENT_HEIGHT / 2,
                }, ];

            0 as LRESULT  // message processed
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MOUSEMOVE => {
            if (wparam & MK_LBUTTON) != 0 || (wparam & MK_RBUTTON) != 0 {
                let hdc = GetDC(hwnd);

                SelectPen(hdc, GetStockPen(WHITE_PEN));
                draw_bezier(hdc, &BEZIER_POINTS);

                if (wparam & MK_LBUTTON) != 0 {
                    BEZIER_POINTS[1].x = GET_X_LPARAM(lparam);
                    BEZIER_POINTS[1].y = GET_Y_LPARAM(lparam);
                }
                if (wparam & MK_RBUTTON) != 0 {
                    BEZIER_POINTS[2].x = GET_X_LPARAM(lparam);
                    BEZIER_POINTS[2].y = GET_Y_LPARAM(lparam);
                }

                SelectPen(hdc, GetStockPen(BLACK_PEN));
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


unsafe fn draw_bezier(hdc: HDC, points: &[POINT; 4]) {
    PolyBezier(hdc, &points[0], 4);
    MoveToEx(hdc, points[0].x, points[0].y, null_mut());
    LineTo(hdc, points[1].x, points[1].y);
    MoveToEx(hdc, points[2].x, points[2].y, null_mut());
    LineTo(hdc, points[3].x, points[3].y);
}
