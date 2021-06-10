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
#![cfg(windows)]
extern crate extras;
extern crate rand;
extern crate winapi;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HGDIOBJ, HWND, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreateSolidBrush, DeleteObject, RGB};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, FillRect, GetDC, LoadCursorW, LoadIconW,
    MessageBoxW, PeekMessageW, PostQuitMessage, RegisterClassExW, ReleaseDC, SetRect, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MSG, PM_REMOVE, SW_SHOW, WM_DESTROY, WM_QUIT, WM_SIZE,
    WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, WHITE_BRUSH};

fn main() {
    let app_name = to_wstr("line_demo");

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

        let caption = to_wstr("Line Demonstration");
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

static mut CLIENT_WIDTH: c_int = 0;
static mut CLIENT_HEIGHT: c_int = 0;

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            0 // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn draw_rectangle(hwnd: HWND) {
    if CLIENT_WIDTH == 0 || CLIENT_HEIGHT == 0 {
        return;
    }

    let mut rect: RECT = mem::MaybeUninit::uninit().assume_init();
    let mut rng = thread_rng();

    let range_x = Uniform::new(0, CLIENT_WIDTH);
    let range_y = Uniform::new(0, CLIENT_HEIGHT);

    SetRect(
        &mut rect,
        range_x.sample(&mut rng),
        range_y.sample(&mut rng),
        range_x.sample(&mut rng),
        range_y.sample(&mut rng),
    );

    // need 16 bits to create range without overflow, cast to 8 after sample.
    // [low, high)
    let range_rgb = Uniform::new(0, 256); // TODO: static in outer scope when rust evolves

    let hbrush: HBRUSH = CreateSolidBrush(RGB(
        range_rgb.sample(&mut rng) as u8,
        range_rgb.sample(&mut rng) as u8,
        range_rgb.sample(&mut rng) as u8,
    ));
    let hdc = GetDC(hwnd);
    FillRect(hdc, &rect, hbrush);
    ReleaseDC(hwnd, hdc);
    DeleteObject(hbrush as HGDIOBJ);
}
