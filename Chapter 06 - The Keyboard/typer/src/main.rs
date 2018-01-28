// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 6 - Typer
//
// The original source code copyright:
//
// TYPER.C -- Typing Program
//            (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::cell::RefCell;
use std::mem;
use std::cmp;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC, InvalidateRect,
                          GetFocus, SetCaretPos, CreateCaret, ShowCaret, HideCaret,
                          DestroyCaret, SendMessageW,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_CREATE, WM_DESTROY, WM_PAINT, WM_SIZE,
                          WM_KEYDOWN, WM_CHAR,
                          WM_INPUTLANGCHANGE, WM_SETFOCUS, WM_KILLFOCUS,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT,
                          VK_END, VK_HOME, VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN, VK_NEXT, VK_PRIOR,
                          VK_DELETE,
};
use winapi::um::wingdi::{GetTextMetricsW, TextOutW, CreateFontW,
                         TEXTMETRICW, DEFAULT_CHARSET, FIXED_PITCH, };
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{LOWORD, DWORD, UINT, WPARAM, LPARAM, LRESULT, HINSTANCE,
                                TRUE, FALSE, };
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::LPCWSTR;

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH,
             to_wstr, GetStockBrush, SelectFont, GetStockFont, DeleteFont, SYSTEM_FONT, };


fn main() {
    let app_name = to_wstr("typer");
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

        let caption = to_wstr("Typing Program");
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
    static mut CHAR_SET: DWORD = DEFAULT_CHARSET;
    static mut CLIENT_X_PIXELS: c_int = 0;
    static mut CLIENT_Y_PIXELS: c_int = 0;
    static mut CHAR_PIXEL_WIDTH: c_int = 0;
    static mut CHAR_PIXEL_HEIGHT: c_int = 0;
    static mut CARET_X: c_int = 0;
    static mut CARET_Y: c_int = 0;

    static mut BUFFER_X_DIM: c_int = 0;
    static mut BUFFER_Y_DIM: c_int = 0;
    let buf_pos = |x: c_int, y: c_int| { (y * BUFFER_X_DIM + x) as usize };
    // This thread_local! macro contains the code that replaces the
    // malloc/free in the original C code. If in doubt there is help within
    // the Rust community - https://users.rust-lang.org/
    thread_local! {
        static BUFFER_VEC: RefCell<Vec<u16>> = RefCell::new(Vec::new());
    }

    match message {
        WM_CREATE | WM_SIZE | WM_INPUTLANGCHANGE => {
            if message == WM_INPUTLANGCHANGE {
                CHAR_SET = wparam as DWORD;
            }
            if message == WM_INPUTLANGCHANGE || message == WM_CREATE {
                let hdc = GetDC(hwnd);
                let mut tm: TEXTMETRICW = mem::uninitialized();

                SelectFont(hdc, CreateFontW(0, 0, 0, 0, 0, 0, 0, 0,
                                            CHAR_SET, 0, 0, 0, FIXED_PITCH, null()));
                GetTextMetricsW(hdc, &mut tm);
                CHAR_PIXEL_WIDTH = tm.tmAveCharWidth;
                CHAR_PIXEL_HEIGHT = tm.tmHeight; //TODO:  + tm.tmExternalLeading;

                // Delete font recently created by CreateFont()
                DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));
                ReleaseDC(hwnd, hdc);
            } else {  // message == WM_SIZE

                CLIENT_X_PIXELS = GET_X_LPARAM(lparam);
                CLIENT_Y_PIXELS = GET_Y_LPARAM(lparam);
            }

            // calculate window size in characters

            BUFFER_X_DIM = cmp::max(1, CLIENT_X_PIXELS / CHAR_PIXEL_WIDTH);
            BUFFER_Y_DIM = cmp::max(1, CLIENT_Y_PIXELS / CHAR_PIXEL_HEIGHT);

            // allocate memory for buffer and clear it

            let size: usize = (BUFFER_X_DIM * BUFFER_Y_DIM) as usize;
            const SPACE_CHAR: u16 = 0x20;  // space character
            BUFFER_VEC.with(|v| v.borrow_mut().clone_from(&vec![SPACE_CHAR; size]));

            // set caret to upper left corner

            CARET_X = 0;
            CARET_Y = 0;

            if hwnd == GetFocus() {
                SetCaretPos(CARET_X * CHAR_PIXEL_WIDTH,
                            CARET_Y * CHAR_PIXEL_HEIGHT);
            }

            InvalidateRect(hwnd, null(), TRUE);
            0 as LRESULT  // message processed
        }

        WM_SETFOCUS => {
            // create and show the caret

            CreateCaret(hwnd, null_mut(), CHAR_PIXEL_WIDTH, CHAR_PIXEL_HEIGHT);
            SetCaretPos(CARET_X * CHAR_PIXEL_WIDTH,
                        CARET_Y * CHAR_PIXEL_HEIGHT);
            ShowCaret(hwnd);
            0 as LRESULT  // message processed
        }

        WM_KILLFOCUS => {
            // hide and destroy the caret

            HideCaret(hwnd);
            DestroyCaret();
            0 as LRESULT  // message processed
        }

        WM_KEYDOWN => {
            match wparam as c_int {
                //@formatter:off
                VK_HOME  => { CARET_X = 0; }
                VK_END   => { CARET_X = BUFFER_X_DIM - 1; }
                VK_PRIOR => { CARET_Y = 0; }
                VK_NEXT  => { CARET_Y = BUFFER_Y_DIM - 1; }
                VK_LEFT  => { CARET_X = cmp::max(CARET_X - 1, 0); }
                VK_RIGHT => { CARET_X = cmp::min(CARET_X + 1, BUFFER_X_DIM - 1); }
                VK_UP    => { CARET_Y = cmp::max(CARET_Y - 1, 0); }
                VK_DOWN  => { CARET_Y = cmp::min(CARET_Y + 1, BUFFER_Y_DIM - 1); }
                //@formatter:on
                VK_DELETE => {
                    BUFFER_VEC.with(|v| {
                        v.borrow_mut().remove(buf_pos(CARET_X, CARET_Y));
                        // Insert space at the end of the line.
                        v.borrow_mut().insert(buf_pos(BUFFER_X_DIM - 1, CARET_Y) as usize, 0x20);
                    });

                    HideCaret(hwnd);
                    let hdc = GetDC(hwnd);

                    SelectFont(hdc, CreateFontW(0, 0, 0, 0, 0, 0, 0, 0,
                                                CHAR_SET, 0, 0, 0, FIXED_PITCH, null()));

                    BUFFER_VEC.with(|v| {
                        let b: std::cell::Ref<Vec<u16>> = v.borrow();
                        let slice = b.get(buf_pos(CARET_X, CARET_Y)).unwrap();
                        TextOutW(hdc,
                                 CARET_X * CHAR_PIXEL_WIDTH,
                                 CARET_Y * CHAR_PIXEL_HEIGHT,
                                 slice, BUFFER_X_DIM - CARET_X);
                    });

                    DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));
                    ReleaseDC(hwnd, hdc);
                    ShowCaret(hwnd);
                }
                _ => {}
            }
            SetCaretPos(CARET_X * CHAR_PIXEL_WIDTH,
                        CARET_Y * CHAR_PIXEL_HEIGHT);
            0 as LRESULT  // message processed
        }

        WM_CHAR => {
            for _ in 0..LOWORD(lparam as DWORD) {
                let ch = std::char::from_u32(wparam as u32).unwrap();
                match ch {
                    '\u{8}' => {                 // backspace
                        if CARET_X > 0 {
                            CARET_X -= 1;
                            SendMessageW(hwnd, WM_KEYDOWN, VK_DELETE as WPARAM, 1);
                        }
                    }
                    '\t' => {                    // tab
                        loop {
                            SendMessageW(hwnd, WM_CHAR, 0x20, 1);
                            if CARET_X % 8 == 0 {
                                break;
                            }
                        }
                    }
                    '\n' => {                    // line feed
                        CARET_Y += 1;
                        if CARET_Y == BUFFER_Y_DIM {
                            CARET_Y = 0;
                        }
                    }
                    '\r' => {                    // carriage return
                        CARET_X = 0;
                        CARET_Y += 1;
                        if CARET_Y == BUFFER_Y_DIM {
                            CARET_Y = 0;
                        }
                    }
                    '\x1B' => {                  // escape
                        BUFFER_VEC.with(|v| {
                            v.borrow_mut().iter_mut().for_each(|c| *c = 0x20);
                        });

                        CARET_X = 0;
                        CARET_Y = 0;

                        InvalidateRect(hwnd, null_mut(), FALSE);
                    }
                    _ => {                       // character codes
                        BUFFER_VEC.with(|v| {
                            v.borrow_mut()[buf_pos(CARET_X, CARET_Y)] = ch as u16;
                        });

                        HideCaret(hwnd);
                        let hdc = GetDC(hwnd);

                        SelectFont(hdc, CreateFontW(0, 0, 0, 0, 0, 0, 0, 0,
                                                    CHAR_SET, 0, 0, 0, FIXED_PITCH, null()));

                        let ch16: u16 = ch as u16;
                        TextOutW(hdc,
                                 CARET_X * CHAR_PIXEL_WIDTH,
                                 CARET_Y * CHAR_PIXEL_HEIGHT,
                                 &ch16, 1);

//                        // Alternative using BUFFER_VEC for TextOutW
//                        BUFFER_VEC.with(|v| {
//                            let b: std::cell::Ref<Vec<u16>> = v.borrow();
//                            let slice = b.get((yCaret * cxBuffer + xCaret) as usize).unwrap();
//                            TextOutW(hdc, xCaret * cxChar, yCaret * cyChar, slice, 1);
//                        });

                        DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));
                        ReleaseDC(hwnd, hdc);
                        ShowCaret(hwnd);

                        CARET_X += 1;
                        if CARET_X == BUFFER_X_DIM {
                            CARET_X = 0;
                            CARET_Y += 1;
                            if CARET_Y == BUFFER_Y_DIM {
                                CARET_Y = 0;
                            }
                        }
                    }
                }
            }

            SetCaretPos(CARET_X * CHAR_PIXEL_WIDTH,
                        CARET_Y * CHAR_PIXEL_HEIGHT);

            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(hdc, CreateFontW(0, 0, 0, 0, 0, 0, 0, 0,
                                        CHAR_SET, 0, 0, 0, FIXED_PITCH, null()));

            BUFFER_VEC.with(|v| {
                for y in 0..BUFFER_Y_DIM {
                    let b: std::cell::Ref<Vec<u16>> = v.borrow();
                    let slice = b.get(buf_pos(0, y)).unwrap();
                    TextOutW(hdc,
                             0,
                             y * CHAR_PIXEL_HEIGHT,
                             slice, BUFFER_X_DIM);
                }
            });

            // Delete font recently created by CreateFont()
            DeleteFont(SelectFont(hdc, GetStockFont(SYSTEM_FONT)));

            EndPaint(hwnd, &mut ps);
            0 as LRESULT  // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
