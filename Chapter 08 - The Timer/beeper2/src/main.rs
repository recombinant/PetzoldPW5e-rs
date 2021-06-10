// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 8 - Beeper2
//
// The original source code copyright:
//
// BEEPER2.C -- Timer Demo Program No. 2
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HDC, HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreateSolidBrush, RGB};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, FillRect, GetClientRect, GetDC, GetMessageW,
    KillTimer, LoadCursorW, LoadIconW, MessageBeep, MessageBoxW, PostQuitMessage, RegisterClassExW,
    ReleaseDC, SetTimer, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG, SW_SHOW, WM_CREATE, WM_DESTROY,
    WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, DeleteBrush, GetStockBrush, WHITE_BRUSH};

const ID_TIMER: usize = 1;

fn main() {
    let app_name = to_wstr("sys_mets2");

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

        let caption = to_wstr("Beeper2 Timer Demo");
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
    match message {
        WM_CREATE => {
            SetTimer(hwnd, ID_TIMER, 1000, Some(timer_proc));
            0 // message processed
        }
        WM_DESTROY => {
            KillTimer(hwnd, ID_TIMER);
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe extern "system" fn timer_proc(
    hwnd: HWND,
    _message: UINT,
    _event_id: UINT_PTR,
    _time: DWORD,
) {
    static mut FLIP_FLOP: bool = false;

    MessageBeep(0xffff_ffff);
    FLIP_FLOP = !FLIP_FLOP;

    let mut rc: RECT = mem::MaybeUninit::uninit().assume_init();
    GetClientRect(hwnd, &mut rc);

    let hdc: HDC = GetDC(hwnd);
    let hbrush: HBRUSH = CreateSolidBrush(if FLIP_FLOP {
        RGB(255, 0, 0)
    } else {
        RGB(0, 0, 255)
    });

    FillRect(hdc, &rc, hbrush);
    DeleteBrush(hbrush);
    ReleaseDC(hwnd, hdc);
}
