// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - Checker3
//
// The original source code copyright:
//
// CHECKER3.C -- Mouse Hit-Test Demo Program No. 3
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
// 1.23 inadequate, requires nightly build
#![feature(const_ptr_null_mut)]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::{c_int, c_long, };
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          MoveWindow, GetClientRect,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW,
                          InvalidateRect, MessageBeep,
                          GetWindowLongPtrW, SetWindowLongPtrW,
                          MSG, PAINTSTRUCT, WNDCLASSEXW, WM_DESTROY, WM_PAINT, WM_SIZE,
                          WM_CREATE, WM_LBUTTONDOWN,
                          WS_OVERLAPPEDWINDOW, WS_CHILDWINDOW, WS_VISIBLE,
                          SW_SHOW, CS_HREDRAW, CS_VREDRAW,
                          IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{Rectangle, MoveToEx, LineTo, };
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, TRUE, FALSE, };
use winapi::shared::windef::{HWND, RECT, HMENU, };
use winapi::shared::ntdef::{LPCWSTR, };

use extras::{WHITE_BRUSH, to_wstr, GetStockBrush, GetWindowInstance, GWLP_USERDATA, };


const DIVISIONS: usize = 5;
static CHILD_CLASS_NAME: &'static str = "checker3_child";


fn main() {
    let app_name = to_wstr("checker3");
    let child_class_name = to_wstr(CHILD_CLASS_NAME);

    unsafe {
        let hinstance = GetModuleHandleW(null());

        let mut wndclassex = WNDCLASSEXW {
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

        wndclassex = WNDCLASSEXW {
            lpfnWndProc : Some(child_wnd_proc),
            cbWndExtra : mem::size_of::<c_long>() as c_int,
            hIcon : null_mut(),
            lpszClassName : child_class_name.as_ptr(),
            ..wndclassex
        };

        RegisterClassExW(&wndclassex);

        let caption = to_wstr("Checker3 Mouse Hit-Test Demo");
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
    static mut HWND_CHILD: [[HWND; DIVISIONS]; DIVISIONS] = [[null_mut(); DIVISIONS]; DIVISIONS];

    match message {
        WM_CREATE => {
            let child_class_name = to_wstr(CHILD_CLASS_NAME);
            for x in 0..DIVISIONS {
                for y in 0..DIVISIONS {
                    HWND_CHILD[x][y] = CreateWindowExW(
                        0,
                        child_class_name.as_ptr(),
                        null(),
                        WS_CHILDWINDOW | WS_VISIBLE,
                        0, 0, 0, 0,
                        hwnd, (y << 8 | x) as HMENU,
                        GetWindowInstance(hwnd),
                        null_mut());
                }
            }
            0 as LRESULT  // message processed
        }

        WM_SIZE => {
            let block_x: c_int = GET_X_LPARAM(lparam) / DIVISIONS as c_int;
            let block_y: c_int = GET_Y_LPARAM(lparam) / DIVISIONS as c_int;

            for x in 0..DIVISIONS {
                for y in 0..DIVISIONS {
                    MoveWindow(HWND_CHILD[x][y],
                               x as c_int * block_x, y as c_int * block_y,
                               block_x, block_y, TRUE);
                }
            }
            0 as LRESULT  // message processed
        }

        WM_LBUTTONDOWN => {
            MessageBeep(0);
            0 as LRESULT  // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}


unsafe extern "system" fn child_wnd_proc(hwnd: HWND,
                                         message: UINT,
                                         wparam: WPARAM,
                                         lparam: LPARAM)
                                         -> LRESULT {
    match message {
        WM_CREATE => {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);       // on/off flag
            0 as LRESULT  // message processed
        }

        WM_LBUTTONDOWN => {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 1 ^ GetWindowLongPtrW(hwnd, GWLP_USERDATA));
            InvalidateRect(hwnd, null(), FALSE);
            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rect: RECT = mem::uninitialized();
            GetClientRect(hwnd, &mut rect);
            Rectangle(hdc, 0, 0, rect.right, rect.bottom);

            if GetWindowLongPtrW(hwnd, GWLP_USERDATA) != 0 {
                MoveToEx(hdc, 0, 0, null_mut());
                LineTo(hdc, rect.right, rect.bottom);
                MoveToEx(hdc, 0, rect.bottom, null_mut());
                LineTo(hdc, rect.right, 0);
            }

            EndPaint(hwnd, &ps);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
