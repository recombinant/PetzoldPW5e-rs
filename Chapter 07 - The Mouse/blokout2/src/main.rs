// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - BlokOut2
//
// The original source code copyright:
//
// BLOKOUT2.C -- Mouse Button & Capture Demo Program
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::shared::minwindef::{LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HDC, HWND, POINT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{Rectangle, SetROP2, R2_NOT};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC, GetMessageW,
    InvalidateRect, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW,
    ReleaseCapture, ReleaseDC, SetCapture, SetCursor, ShowWindow, TranslateMessage, UpdateWindow,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDC_CROSS, IDI_APPLICATION, MB_ICONERROR,
    MSG, PAINTSTRUCT, SW_SHOW, WM_CHAR, WM_DESTROY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
    WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

use extras::{to_wstr, GetStockBrush, SelectBrush, BLACK_BRUSH, NULL_BRUSH, WHITE_BRUSH};

fn main() {
    let app_name = to_wstr("blokout2");

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

        let caption = to_wstr("Mouse Button & Capture Demo");
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
    static mut BLOCKING: bool = false;
    static mut VALID_BOX: bool = false;
    static mut BEGIN_POINT: POINT = POINT { x: 0, y: 0 };
    static mut END_POINT: POINT = POINT { x: 0, y: 0 };
    static mut BOX_BEGIN_POINT: POINT = POINT { x: 0, y: 0 };
    static mut BOX_END_POINT: POINT = POINT { x: 0, y: 0 };

    match message {
        WM_LBUTTONDOWN => {
            BEGIN_POINT.x = GET_X_LPARAM(lparam);
            BEGIN_POINT.y = GET_Y_LPARAM(lparam);
            END_POINT.x = GET_X_LPARAM(lparam);
            END_POINT.y = GET_Y_LPARAM(lparam);

            draw_box_outline(hwnd, BEGIN_POINT, END_POINT);

            SetCapture(hwnd);
            SetCursor(LoadCursorW(null_mut(), IDC_CROSS));

            BLOCKING = true;

            0 as LRESULT // message processed
        }

        WM_MOUSEMOVE => {
            if BLOCKING {
                SetCursor(LoadCursorW(null_mut(), IDC_CROSS));

                draw_box_outline(hwnd, BEGIN_POINT, END_POINT);

                END_POINT.x = GET_X_LPARAM(lparam);
                END_POINT.y = GET_Y_LPARAM(lparam);

                draw_box_outline(hwnd, BEGIN_POINT, END_POINT);
            }
            0 as LRESULT // message processed
        }

        WM_LBUTTONUP => {
            if BLOCKING {
                draw_box_outline(hwnd, BEGIN_POINT, END_POINT);

                BOX_BEGIN_POINT = BEGIN_POINT;
                BOX_END_POINT.x = GET_X_LPARAM(lparam);
                BOX_END_POINT.y = GET_Y_LPARAM(lparam);

                ReleaseCapture();
                SetCursor(LoadCursorW(null_mut(), IDC_ARROW));

                BLOCKING = false;
                VALID_BOX = true;

                InvalidateRect(hwnd, null(), TRUE);
            }
            0 as LRESULT // message processed
        }

        WM_CHAR => {
            let ch = std::char::from_u32(wparam as u32).unwrap();
            if BLOCKING && (ch == '\u{1b}') {
                // i.e., Escape

                draw_box_outline(hwnd, BEGIN_POINT, END_POINT);

                ReleaseCapture();
                SetCursor(LoadCursorW(null_mut(), IDC_ARROW));

                BLOCKING = false;
            }
            0 as LRESULT // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            if VALID_BOX {
                SelectBrush(hdc, GetStockBrush(BLACK_BRUSH));
                Rectangle(
                    hdc,
                    BOX_BEGIN_POINT.x,
                    BOX_BEGIN_POINT.y,
                    BOX_END_POINT.x,
                    BOX_END_POINT.y,
                );
            }

            if BLOCKING {
                SetROP2(hdc, R2_NOT);
                SelectBrush(hdc, GetStockBrush(NULL_BRUSH));
                Rectangle(hdc, BEGIN_POINT.x, BEGIN_POINT.y, END_POINT.x, END_POINT.y);
            }

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

unsafe fn draw_box_outline(hwnd: HWND, begin_point: POINT, end_point: POINT) {
    let hdc: HDC = GetDC(hwnd);

    SetROP2(hdc, R2_NOT);
    SelectBrush(hdc, GetStockBrush(NULL_BRUSH));
    Rectangle(hdc, begin_point.x, begin_point.y, end_point.x, end_point.y);

    ReleaseDC(hwnd, hdc);
}
