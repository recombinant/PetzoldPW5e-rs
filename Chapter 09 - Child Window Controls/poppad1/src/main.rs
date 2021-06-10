// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - PopPad1
//
// The original source code copyright:
//
// POPPAD1.C -- Popup Editor using child window edit box
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, HIWORD, LOWORD, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::windef::{HMENU, HWND};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, LoadIconW,
    MessageBoxW, MoveWindow, PostQuitMessage, RegisterClassExW, SetFocus, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, EN_ERRSPACE, EN_MAXTEXT,
    ES_AUTOHSCROLL, ES_AUTOVSCROLL, ES_LEFT, ES_MULTILINE, IDC_ARROW, IDI_APPLICATION,
    LPCREATESTRUCTW, MB_ICONERROR, MB_ICONSTOP, MB_OK, MSG, SW_SHOW, WM_COMMAND, WM_CREATE,
    WM_DESTROY, WM_SETFOCUS, WM_SIZE, WNDCLASSEXW, WS_BORDER, WS_CHILD, WS_HSCROLL,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_VSCROLL,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, WHITE_BRUSH};

const ID_EDIT: c_int = 1;
static APP_NAME: &str = "poppad1";

fn main() {
    let app_name = to_wstr(APP_NAME);

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

        let hwnd = CreateWindowExW(
            0,                   // dwExStyle:
            app_name.as_ptr(),   // lpClassName: class name or atom
            app_name.as_ptr(),   // lpWindowName: window caption
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
    static mut HWND_EDIT: HWND = null_mut();

    match message {
        WM_CREATE => {
            let text = to_wstr("edit");
            HWND_EDIT = CreateWindowExW(
                0,
                text.as_ptr(),
                null(),
                WS_CHILD
                    | WS_VISIBLE
                    | WS_HSCROLL
                    | WS_VSCROLL
                    | WS_BORDER
                    | ES_LEFT
                    | ES_MULTILINE
                    | ES_AUTOHSCROLL
                    | ES_AUTOVSCROLL,
                0,
                0,
                0,
                0,
                hwnd,
                ID_EDIT as HMENU,
                (*(lparam as LPCREATESTRUCTW)).hInstance,
                null_mut(),
            );

            0 // message processed
        }

        WM_SETFOCUS => {
            SetFocus(HWND_EDIT);
            0 // message processed
        }

        WM_SIZE => {
            let client_width = GET_X_LPARAM(lparam);
            let client_height = GET_Y_LPARAM(lparam);

            MoveWindow(HWND_EDIT, 0, 0, client_width, client_height, TRUE);
            0 // message processed
        }

        WM_COMMAND => {
            if LOWORD(wparam as DWORD) as c_int == ID_EDIT {
                let hiword = HIWORD(wparam as DWORD);
                if hiword == EN_ERRSPACE || hiword == EN_MAXTEXT {
                    let text = to_wstr("Edit control out of space.");
                    let caption = to_wstr(APP_NAME);
                    MessageBoxW(hwnd, text.as_ptr(), caption.as_ptr(), MB_OK | MB_ICONSTOP);
                }
            }
            0 // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
