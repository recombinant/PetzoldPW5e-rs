// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 8 - Beeper1
//
// The original source code copyright:
//
// BEEPER1.C -- Timer Demo Program No. 1
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW,
                          GetClientRect, SetTimer, MessageBeep, FillRect, InvalidateRect,
                          KillTimer,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_CREATE, WM_DESTROY, WS_OVERLAPPEDWINDOW, WM_PAINT, WM_TIMER,
                          SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{CreateSolidBrush, RGB, };
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, FALSE, };
use winapi::shared::windef::{HWND, RECT, };
use winapi::shared::ntdef::LPCWSTR;

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, to_wstr, GetStockBrush, DeleteBrush};


const ID_TIMER: usize = 1;


fn main() {
    let app_name = to_wstr("sys_mets1");
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

        let caption = to_wstr("Beeper1 Timer Demo");
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
    static mut FLIP_FLOP: bool = false;

    match message {
        WM_CREATE => {
            SetTimer(hwnd, ID_TIMER, 1000, None);
            0 as LRESULT  // message processed
        }

        WM_TIMER => {
            MessageBeep(0xFFFFFFFF);
            FLIP_FLOP = !FLIP_FLOP;
            InvalidateRect(hwnd, null(), FALSE);
            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rc: RECT = mem::uninitialized();
            GetClientRect(hwnd, &mut rc);
            let hbrush = CreateSolidBrush(if FLIP_FLOP { RGB(255, 0, 0) } else { RGB(0, 0, 255) });
            FillRect(hdc, &rc, hbrush);
            DeleteBrush(hbrush);

            EndPaint(hwnd, &ps);
            0 as LRESULT  // message processed
        }
        WM_DESTROY => {
            KillTimer(hwnd, ID_TIMER);
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
