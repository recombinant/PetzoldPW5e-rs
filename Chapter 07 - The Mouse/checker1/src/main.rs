// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - Checker1
//
// The original source code copyright:
//
// CHECKER1.C -- Mouse Hit-Test Demo Program No. 1
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::{c_int, };
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW,
                          InvalidateRect, MessageBeep,
                          MSG, PAINTSTRUCT, WNDCLASSEXW, WM_DESTROY, WM_PAINT, WM_SIZE,
                          WM_LBUTTONDOWN, WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW, CS_VREDRAW,
                          IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{Rectangle, MoveToEx, LineTo};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, FALSE};
use winapi::shared::windef::{HWND, RECT};
use winapi::shared::ntdef::{LPCWSTR, };

use extras::{WHITE_BRUSH, to_wstr, GetStockBrush};


const DIVISIONS: usize = 5;

fn main() {
    let app_name = to_wstr("checker1");
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

        let caption = to_wstr("Checker1 Mouse Hit-Test Demo");
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
    static mut BLOCK_X: c_int = 0;
    static mut BLOCK_Y: c_int = 0;
    static mut STATE: [[bool; DIVISIONS]; DIVISIONS] = [[false; DIVISIONS]; DIVISIONS];

    match message {
        WM_SIZE => {
            BLOCK_X = GET_X_LPARAM(lparam) / DIVISIONS as c_int;
            BLOCK_Y = GET_Y_LPARAM(lparam) / DIVISIONS as c_int;
            0 as LRESULT  // message processed
        }

        WM_LBUTTONDOWN => {
            let x = GET_X_LPARAM(lparam) / BLOCK_X;
            let y = GET_Y_LPARAM(lparam) / BLOCK_Y;

            if x < DIVISIONS as c_int && y < DIVISIONS as c_int {
                STATE[x as usize][y as usize] = !STATE[x as usize][y as usize];

                let rect = RECT {
                    left: x * BLOCK_X,
                    top: y * BLOCK_Y,
                    right: (x + 1) * BLOCK_X,
                    bottom: (y + 1) * BLOCK_Y,
                };

                InvalidateRect(hwnd, &rect, FALSE);
            } else {
                MessageBeep(0);
            }
            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            for x in 0..DIVISIONS as c_int {
                for y in 0..DIVISIONS as c_int {
                    Rectangle(hdc,
                              x * BLOCK_X,
                              y * BLOCK_Y,
                              (x + 1) * BLOCK_X,
                              (y + 1) * BLOCK_Y);

                    if STATE[x as usize][y as usize] {
                        MoveToEx(hdc, x * BLOCK_X, y * BLOCK_Y, null_mut());
                        LineTo(hdc, (x + 1) * BLOCK_X, (y + 1) * BLOCK_Y);
                        MoveToEx(hdc, x * BLOCK_X, (y + 1) * BLOCK_Y, null_mut());
                        LineTo(hdc, (x + 1) * BLOCK_X, y * BLOCK_Y);
                    }
                }
            }

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
