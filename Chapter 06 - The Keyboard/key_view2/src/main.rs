// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 6 - KeyView2
//
// The original source code copyright:
//
// KEYVIEW2.C -- Displays Keyboard and Character Messages
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, HIWORD, LOWORD, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::{LONG, LPCWSTR};
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{
    CreateFontW, GetTextMetricsW, SetBkMode, TextOutW, DEFAULT_CHARSET, FIXED_PITCH, TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC,
    GetKeyNameTextW, GetMessageW, GetSystemMetrics, InvalidateRect, LoadCursorW, LoadIconW,
    MessageBoxW, PostQuitMessage, RegisterClassExW, ReleaseDC, ScrollWindow, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SM_CXMAXIMIZED, SM_CYMAXIMIZED, SW_SHOW,
    WM_CHAR, WM_CREATE, WM_DEADCHAR, WM_DESTROY, WM_DISPLAYCHANGE, WM_INPUTLANGCHANGE, WM_KEYDOWN,
    WM_KEYFIRST, WM_KEYUP, WM_PAINT, WM_SIZE, WM_SYSCHAR, WM_SYSDEADCHAR, WM_SYSKEYDOWN,
    WM_SYSKEYUP, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, DeleteFont, GetStockBrush, GetStockFont, SelectFont, SYSTEM_FONT, TRANSPARENT,
    WHITE_BRUSH,
};

fn main() {
    let app_name = to_wstr("key_view2");

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

        let caption = to_wstr("Keyboard Message Viewer #2");
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
    static mut CHAR_SET: DWORD = DEFAULT_CHARSET;
    static mut MAX_CLIENT_WIDTH: c_int = 0;
    static mut MAX_CLIENT_HEIGHT: c_int = 0;
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut CHAR_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;
    static mut MAX_LINES: usize = 0;
    static mut SCROLL_RECT: RECT = RECT {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    static HEADER1: &str = "Message        Key       Char     Repeat Scan Ext ALT Prev Tran";
    static HEADER2: &str = "_______        ___       ____     ______ ____ ___ ___ ____ ____";

    static KEY_STRINGS: &[&str] = &[
        "WM_KEYDOWN",
        "WM_KEYUP",
        "WM_CHAR",
        "WM_DEADCHAR",
        "WM_SYSKEYDOWN",
        "WM_SYSKEYUP",
        "WM_SYSCHAR",
        "WM_SYSDEADCHAR",
    ];

    // This thread_local! macro contains the code that replaces the
    // malloc/free in the original C code. If in doubt there is help within
    // the Rust community - https://users.rust-lang.org/
    thread_local! {
        static MSG_VEC: RefCell<VecDeque<MSG>> = RefCell::new(VecDeque::new());
    }

    match message {
        WM_CREATE | WM_DISPLAYCHANGE | WM_SIZE | WM_INPUTLANGCHANGE => {
            // Use array rather to reduce clutter (cf. key_view1)
            static MESSAGES: &[UINT] = &[WM_CREATE, WM_DISPLAYCHANGE, WM_INPUTLANGCHANGE];

            if MESSAGES.contains(&message) {
                if message == WM_INPUTLANGCHANGE {
                    CHAR_SET = wparam as DWORD;
                }

                // Get maximum size of client area

                MAX_CLIENT_WIDTH = GetSystemMetrics(SM_CXMAXIMIZED);
                MAX_CLIENT_HEIGHT = GetSystemMetrics(SM_CYMAXIMIZED);

                // Get character size for fixed−pitch font

                let hdc = GetDC(hwnd);
                let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();

                SelectFont(
                    hdc,
                    CreateFontW(
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        CHAR_SET,
                        0,
                        0,
                        0,
                        FIXED_PITCH,
                        null(),
                    ),
                );
                GetTextMetricsW(hdc, &mut tm);
                CHAR_WIDTH = tm.tmAveCharWidth;
                CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

                // Delete font recently created by CreateFont()
                DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));
                ReleaseDC(hwnd, hdc);

                MAX_LINES = (MAX_CLIENT_HEIGHT / CHAR_HEIGHT).abs() as usize;
                MSG_VEC.with(|v| v.borrow_mut().clear());
            } else {
                // message == WM_SIZE

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

            0 // message processed
        }

        WM_KEYDOWN | WM_KEYUP | WM_CHAR | WM_DEADCHAR | WM_SYSKEYDOWN | WM_SYSKEYUP
        | WM_SYSCHAR | WM_SYSDEADCHAR => {
            // Rearrange storage array, limit to MAX_LINES elements.

            if MAX_LINES == MSG_VEC.with(|v| v.borrow().len()) {
                MSG_VEC.with(|v| v.borrow_mut().pop_back());
            }

            // Store new message

            let msg: MSG = MSG {
                hwnd,
                message,
                wParam: wparam,
                lParam: lparam,
                pt: POINT { x: 0, y: 0 },
                time: 0,
            };

            MSG_VEC.with(|v| v.borrow_mut().push_front(msg));

            // Scroll up the display

            ScrollWindow(hwnd, 0, -CHAR_HEIGHT, &SCROLL_RECT, &SCROLL_RECT);

            // call DefWindowProc so Sys messages work
            DefWindowProcW(hwnd, message, wparam, lparam)
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(
                hdc,
                CreateFontW(
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    CHAR_SET,
                    0,
                    0,
                    0,
                    FIXED_PITCH,
                    null(),
                ),
            );

            SetBkMode(hdc, TRANSPARENT);
            let top_string = to_wstr(HEADER1);
            let und_string = to_wstr(HEADER2);
            TextOutW(
                hdc,
                0,
                0,
                top_string.as_ptr(),
                lstrlenW(top_string.as_ptr()),
            );
            TextOutW(
                hdc,
                0,
                0,
                und_string.as_ptr(),
                lstrlenW(und_string.as_ptr()),
            );

            // Accessing the VecDeque within the Thread Local Storage (TLS)
            // has to take place within the context of the .with() method.
            MSG_VEC.with(|v| {
                v.borrow().iter().enumerate().for_each(|(i, msg)| {
                    let mut win_text: Vec<u16> = vec![0; 32];
                    let size = GetKeyNameTextW(
                        msg.lParam as LONG,
                        win_text.as_mut_ptr(),
                        win_text.len() as c_int,
                    );
                    let key_name = if size > 0 {
                        OsString::from_wide(&win_text[..size as usize])
                            .into_string()
                            .unwrap()
                    } else {
                        String::from("")
                    };

                    let m = msg.message;
                    let fmt_idx: usize = if m == WM_CHAR
                        || m == WM_SYSCHAR
                        || m == WM_DEADCHAR
                        || m == WM_SYSDEADCHAR
                    {
                        1
                    } else {
                        0
                    };

                    let buffer: String;
                    if fmt_idx == 0 {
                        buffer = format!(
                            "{:-14} {:3} {:-15}{:6} {:4} {:3} {:3} {:4} {:4}",
                            KEY_STRINGS[(msg.message - WM_KEYFIRST) as usize],
                            msg.wParam,
                            key_name,
                            LOWORD(msg.lParam as DWORD),
                            HIWORD(msg.lParam as DWORD) & 0xff,
                            if 0x0100_0000 & msg.lParam != 0 {
                                "YES"
                            } else {
                                "NO"
                            },
                            if 0x2000_0000 & msg.lParam != 0 {
                                "YES"
                            } else {
                                "NO"
                            },
                            if 0x4000_0000 & msg.lParam != 0 {
                                "DOWN"
                            } else {
                                "UP"
                            },
                            if 0x8000_0000 & msg.lParam != 0 {
                                "UP"
                            } else {
                                "DOWN"
                            }
                        );
                        win_text = to_wstr(&buffer);
                    } else {
                        buffer = format!(
                            "{:-14}           0x{:04x}   {:6} {:4} {:3} {:3} {:4} {:4}",
                            KEY_STRINGS[(msg.message - WM_KEYFIRST) as usize],
                            msg.wParam,
                            LOWORD(msg.lParam as DWORD),
                            HIWORD(msg.lParam as DWORD) & 0xff,
                            if 0x0100_0000 & msg.lParam != 0 {
                                "YES"
                            } else {
                                "NO"
                            },
                            if 0x2000_0000 & msg.lParam != 0 {
                                "YES"
                            } else {
                                "NO"
                            },
                            if 0x4000_0000 & msg.lParam != 0 {
                                "DOWN"
                            } else {
                                "UP"
                            },
                            if 0x8000_0000 & msg.lParam != 0 {
                                "UP"
                            } else {
                                "DOWN"
                            }
                        );
                        win_text = to_wstr(&buffer);
                        // Manually insert the windows key character into the
                        // windows text. Mimics the %c in wsprintf()
                        win_text[32] = msg.wParam as u16;
                    }

                    let y: c_int = (CLIENT_HEIGHT / CHAR_HEIGHT + 1 - i as c_int) * CHAR_HEIGHT;
                    TextOutW(hdc, 0, y, win_text.as_ptr(), lstrlenW(win_text.as_ptr()));
                })
            });

            // Delete font recently created by CreateFont()
            DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));

            EndPaint(hwnd, &ps);
            0 // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
