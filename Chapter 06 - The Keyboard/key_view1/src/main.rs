// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 6 - KeyView1
//
// The original source code copyright:
//
// KEYVIEW1.C -- Displays Keyboard and Character Messages
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::cell::RefCell;
use std::cmp;
use std::mem;
use std::collections::VecDeque;
use std::ptr::{null_mut, null};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC, GetSystemMetrics, InvalidateRect, ScrollWindowEx,
                          GetKeyNameTextW,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_CREATE, WM_DESTROY, WM_PAINT, WM_SIZE, WM_KEYFIRST,
                          WM_DISPLAYCHANGE, WM_KEYDOWN, WM_KEYUP, WM_CHAR, WM_DEADCHAR,
                          WM_SYSKEYDOWN, WM_SYSKEYUP, WM_SYSCHAR, WM_SYSDEADCHAR,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT,
                          SM_CXMAXIMIZED, SM_CYMAXIMIZED, SW_INVALIDATE, SW_ERASE, };
use winapi::um::wingdi::{GetTextMetricsW, TextOutW, SetBkMode,
                         TEXTMETRICW, };
use winapi::um::winbase::lstrlenW;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{HIWORD, LOWORD, DWORD, UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, TRUE};
use winapi::shared::windef::{HWND, RECT, POINT};
use winapi::shared::ntdef::{LPCWSTR, LONG};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, SYSTEM_FIXED_FONT, TRANSPARENT,
             to_wstr, GetStockBrush, SelectFont, GetStockFont};


fn main() {
    let app_name = to_wstr("key_view1");
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

        let caption = to_wstr("Keyboard Message Viewer #1");
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
    static mut MAX_CLIENT_WIDTH: c_int = 0;
    static mut MAX_CLIENT_HEIGHT: c_int = 0;
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut CHAR_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;
    static mut MAX_LINES: usize = 0;
    static mut SCROLL_RECT: RECT = RECT { left: 0, right: 0, top: 0, bottom: 0 };
    static HEADER1: &'static str = "Message        Key       Char     Repeat Scan Ext ALT Prev Tran";
    static HEADER2: &'static str = "_______        ___       ____     ______ ____ ___ ___ ____ ____";

    static KEY_STRINGS: &'static [&'static str] = &["WM_KEYDOWN", "WM_KEYUP", "WM_CHAR",
        "WM_DEADCHAR", "WM_SYSKEYDOWN", "WM_SYSKEYUP", "WM_SYSCHAR", "WM_SYSDEADCHAR"];

    // This thread_local! macro contains the code that replaces the
    // malloc/free in the original C code. If in doubt there is help within
    // the Rust community - https://users.rust-lang.org/
    thread_local! {
        static MSG_VEC: RefCell<VecDeque<MSG>> = RefCell::new(VecDeque::new());
    }

    match message {
        WM_CREATE | WM_DISPLAYCHANGE | WM_SIZE => {
            if message == WM_CREATE || message == WM_DISPLAYCHANGE {
                // Get maximum size of client area

                MAX_CLIENT_WIDTH = GetSystemMetrics(SM_CXMAXIMIZED);
                MAX_CLIENT_HEIGHT = GetSystemMetrics(SM_CYMAXIMIZED);

                // Get character size for fixed−pitch font

                let hdc = GetDC(hwnd);
                let mut tm: TEXTMETRICW = mem::uninitialized();

                SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));
                GetTextMetricsW(hdc, &mut tm);
                CHAR_WIDTH = tm.tmAveCharWidth;
                CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

                ReleaseDC(hwnd, hdc);

                MAX_LINES = cmp::max((MAX_CLIENT_HEIGHT / CHAR_HEIGHT - 2).abs(), 2) as usize;
                MSG_VEC.with(|v| v.borrow_mut().clear());
            } else {  // message == WM_SIZE

                CLIENT_WIDTH = GET_X_LPARAM(lparam);
                CLIENT_HEIGHT = GET_Y_LPARAM(lparam);
            }

            // Calculate scrolling rectangle

            SCROLL_RECT = RECT {
                left: 0,
                right: CLIENT_WIDTH,
                top: CHAR_HEIGHT,
                bottom: CHAR_WIDTH * (CLIENT_WIDTH / CHAR_WIDTH),
            };

            InvalidateRect(hwnd, null_mut(), TRUE);

            0 as LRESULT  // message processed
        }

        WM_KEYDOWN |
        WM_KEYUP |
        WM_CHAR |
        WM_DEADCHAR |
        WM_SYSKEYDOWN |
        WM_SYSKEYUP |
        WM_SYSCHAR |
        WM_SYSDEADCHAR => {

            // Rearrange storage array, limit to MAX_LINES elements.

            if MAX_LINES == MSG_VEC.with(|v| v.borrow().len()) {
                MSG_VEC.with(|v| v.borrow_mut().pop_back());
            }

            // Store new message

            let mut msg: MSG = MSG {
                hwnd,
                message,
                wParam: wparam,
                lParam: lparam,
                pt: POINT { x: 0, y: 0 },
                time: 0,
            };

            MSG_VEC.with(|v| v.borrow_mut().push_front(msg));

            // Scroll up the display

            // ScrollWindow(hwnd, 0, -CHAR_HEIGHT, &SCROLL_RECT, &SCROLL_RECT);
            ScrollWindowEx(hwnd,
                           0, -CHAR_HEIGHT,
                           &SCROLL_RECT, &SCROLL_RECT,
                           null_mut(), null_mut(), SW_INVALIDATE|SW_ERASE);

            // call DefWindowProc so Sys messages work
            DefWindowProcW(hwnd, message, wparam, lparam)
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));
            SetBkMode(hdc, TRANSPARENT);
            let top_string = to_wstr(HEADER1);
            let und_string = to_wstr(HEADER2);
            TextOutW(hdc, 0, 0, top_string.as_ptr(), lstrlenW(top_string.as_ptr()));
            TextOutW(hdc, 0, 0, und_string.as_ptr(), lstrlenW(und_string.as_ptr()));

            // Accessing the VecDeque within the Thread Local Storage (TLS)
            // has to take place within the context of the .with() method.
            MSG_VEC.with(|v| {
                v.borrow().iter().enumerate().for_each(|(i, msg)| {
                    let mut win_text: Vec<u16> = vec![0; 32];
                    let size = GetKeyNameTextW(msg.lParam as LONG,
                                               win_text.as_mut_ptr(),
                                               win_text.len() as c_int);
                    let key_name = if size > 0 {
                        OsString::from_wide(&win_text[..size as usize]).into_string().unwrap()
                    } else {
                        String::from("")
                    };

                    let m = msg.message;
                    let fmt_idx: usize = if
                        m == WM_CHAR ||
                            m == WM_SYSCHAR ||
                            m == WM_DEADCHAR ||
                            m == WM_SYSDEADCHAR { 1 } else { 0 };

                    let buffer: String;
                    if fmt_idx == 0 {
                        buffer = format!("{:-14} {:3} {:-15}{:6} {:4} {:3} {:3} {:4} {:4}",
                                         KEY_STRINGS[(msg.message - WM_KEYFIRST) as usize],
                                         msg.wParam,
                                         key_name,
                                         LOWORD(msg.lParam as DWORD),
                                         HIWORD(msg.lParam as DWORD) & 0xff,
                                         if 0x01000000 & msg.lParam != 0 { "YES" } else { "NO" },
                                         if 0x20000000 & msg.lParam != 0 { "YES" } else { "NO" },
                                         if 0x40000000 & msg.lParam != 0 { "DOWN" } else { "UP" },
                                         if 0x80000000 & msg.lParam != 0 { "UP" } else { "DOWN" });
                        win_text = to_wstr(&buffer);
                    } else {
                        buffer = format!("{:-14}           0x{:04x}   {:6} {:4} {:3} {:3} {:4} {:4}",
                                         KEY_STRINGS[(msg.message - WM_KEYFIRST) as usize],
                                         msg.wParam,
                                         LOWORD(msg.lParam as DWORD),
                                         HIWORD(msg.lParam as DWORD) & 0xff,
                                         if 0x01000000 & msg.lParam != 0 { "YES" } else { "NO" },
                                         if 0x20000000 & msg.lParam != 0 { "YES" } else { "NO" },
                                         if 0x40000000 & msg.lParam != 0 { "DOWN" } else { "UP" },
                                         if 0x80000000 & msg.lParam != 0 { "UP" } else { "DOWN" });
                        win_text = to_wstr(&buffer);
                        // Manually insert the windows key character into the
                        // windows text. Mimics the %c in wsprintf()
                        win_text[32] = msg.wParam as u16;
                    }

                    let y: c_int = (CLIENT_HEIGHT / CHAR_HEIGHT + 1 - i as c_int) * CHAR_HEIGHT;
                    TextOutW(hdc,
                             0,
                             y,
                             win_text.as_ptr(),
                             lstrlenW(win_text.as_ptr()),
                    );
                })
            });

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
