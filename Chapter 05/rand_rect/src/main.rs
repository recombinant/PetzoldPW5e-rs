// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - RandRect
//
// The original source code copyright:
//
// RANDRECT.C -- Displays Random Rectangles
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)] extern crate winapi;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng};
use std::mem;
use std::ptr::{null_mut, null};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::{c_int};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, TranslateMessage, DispatchMessageW,
                          MessageBoxW, LoadIconW, LoadCursorW, PeekMessageW,
                          GetDC, ReleaseDC, SetRect, FillRect,
                          MSG, WNDCLASSEXW,
                          WM_DESTROY, WM_SIZE, WM_QUIT, PM_REMOVE,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetStockObject, CreateSolidBrush, RGB, DeleteObject};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, LOWORD, HIWORD, DWORD};
use winapi::shared::windef::{HWND, HBRUSH, RECT, HGDIOBJ};
use winapi::shared::ntdef::LPCWSTR;

// There are some mismatches in winapi types between constants and their usage...
const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;


fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn main() {
    let app_name = to_wstring("line_demo");
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

        let caption = to_wstring("Line Demonstration");
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
            let ret = PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE);

            if ret == -1 {
                // handle the error and/or exit
                // for error call GetLastError();
                return;
            } else if ret == 0 {
                draw_rectangle(hwnd);
            } else {
                if msg.message == WM_QUIT {
                    break;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
// return msg.wParam;  // WM_QUIT
    }
}


static mut CX_CLIENT: c_int = 0;
static mut CY_CLIENT: c_int = 0;


unsafe extern "system" fn wnd_proc(hwnd: HWND,
                                   message: UINT,
                                   wparam: WPARAM,
                                   lparam: LPARAM)
                                   -> LRESULT {
    match message {
        WM_SIZE => {
            CX_CLIENT = LOWORD(lparam as DWORD) as c_int;
            CY_CLIENT = HIWORD(lparam as DWORD) as c_int;

            0 as LRESULT  // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}


unsafe fn draw_rectangle(hwnd: HWND)
{
    if CX_CLIENT == 0 || CY_CLIENT == 0 {
        return;
    }

    let mut rect: RECT = mem::uninitialized();
    let mut rng = thread_rng();

    let range_x = Range::new(0, CX_CLIENT);
    let range_y = Range::new(0, CY_CLIENT);

    SetRect(&mut rect,
            range_x.ind_sample(&mut rng),
            range_y.ind_sample(&mut rng),
            range_x.ind_sample(&mut rng),
            range_y.ind_sample(&mut rng));

    // need 16 bits to create range without overflow, cast to 8 after sample.
    // [low, high)
    let range_rgb = Range::new(0, 256);  // TODO: static in outer scope when rust evolves

    let hbrush: HBRUSH = CreateSolidBrush(RGB(range_rgb.ind_sample(&mut rng) as u8,
                                              range_rgb.ind_sample(&mut rng) as u8,
                                              range_rgb.ind_sample(&mut rng) as u8));
    let hdc = GetDC(hwnd);
    FillRect(hdc, &rect, hbrush);
    ReleaseDC(hwnd, hdc);
    DeleteObject(hbrush as HGDIOBJ);
}     
