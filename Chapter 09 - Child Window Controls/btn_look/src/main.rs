// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - BtnLook
//
// The original source code copyright:
//
// BTNLOOK.C -- Button Look Program
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
use winapi::shared::ntdef::{LONG, LPCWSTR};
use winapi::shared::windef::{HDC, HMENU, HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{SetBkMode, TextOutW};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC,
    GetDialogBaseUnits, GetMessageW, InvalidateRect, LoadCursorW, LoadIconW, MessageBoxW,
    PostQuitMessage, RegisterClassExW, ReleaseDC, ScrollWindowEx, ShowWindow, TranslateMessage,
    UpdateWindow, ValidateRect, BS_3STATE, BS_AUTO3STATE, BS_AUTOCHECKBOX, BS_AUTORADIOBUTTON,
    BS_CHECKBOX, BS_DEFPUSHBUTTON, BS_GROUPBOX, BS_OWNERDRAW, BS_PUSHBUTTON, BS_RADIOBUTTON,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, LPCREATESTRUCTW,
    MB_ICONERROR, MSG, PAINTSTRUCT, SW_ERASE, SW_INVALIDATE, SW_SHOW, WM_COMMAND, WM_CREATE,
    WM_DESTROY, WM_DRAWITEM, WM_PAINT, WM_SIZE, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetStockBrush, GetStockFont, SelectFont, SYSTEM_FIXED_FONT, TRANSPARENT, WHITE_BRUSH,
};

const BUTTON_COUNT: usize = 10;

pub struct Button<'a> {
    pub style: DWORD,
    pub text: &'a str,
}

pub const BUTTONS: &[Button; BUTTON_COUNT] = &[
    Button {
        style: BS_PUSHBUTTON,
        text: "PUSHBUTTON",
    },
    Button {
        style: BS_DEFPUSHBUTTON,
        text: "DEFPUSHBUTTON",
    },
    Button {
        style: BS_CHECKBOX,
        text: "CHECKBOX",
    },
    Button {
        style: BS_AUTOCHECKBOX,
        text: "AUTOCHECKBOX",
    },
    Button {
        style: BS_RADIOBUTTON,
        text: "RADIOBUTTON",
    },
    Button {
        style: BS_3STATE,
        text: "3STATE",
    },
    Button {
        style: BS_AUTO3STATE,
        text: "AUTO3STATE",
    },
    Button {
        style: BS_GROUPBOX,
        text: "GROUPBOX",
    },
    Button {
        style: BS_AUTORADIOBUTTON,
        text: "AUTORADIO",
    },
    Button {
        style: BS_OWNERDRAW,
        text: "OWNERDRAW",
    },
];

fn main() {
    let app_name = to_wstr("btn_look");

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

        let caption = to_wstr("Button Look");
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
    static mut CHAR_X: c_int = 0;
    static mut CHAR_Y: c_int = 0;
    static mut TARGET_RECT: RECT = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    static mut HWND_BUTTON: [HWND; BUTTON_COUNT] = [null_mut(); BUTTON_COUNT];

    static HEADER1: &str = "message            wParam       lParam";
    static HEADER2: &str = "_______            ______       ______";

    match message {
        WM_CREATE => {
            CHAR_X = LOWORD(GetDialogBaseUnits() as DWORD) as c_int;
            CHAR_Y = HIWORD(GetDialogBaseUnits() as DWORD) as c_int;

            let text = to_wstr("button");

            for (i, btn) in BUTTONS.iter().enumerate() {
                let btn_text = to_wstr(btn.text);
                HWND_BUTTON[i] = CreateWindowExW(
                    0,                                 // dwExStyle
                    text.as_ptr(),                     // lpClassName
                    btn_text.as_ptr(),                 // lpWindowName
                    WS_CHILD | WS_VISIBLE | btn.style, // dwStyle
                    CHAR_X,
                    CHAR_Y * (1 + 2 * i as c_int), // x, y
                    20 * CHAR_X,
                    7 * CHAR_Y / 4,                           // nWidth, nHeight
                    hwnd,                                     // hwndParent
                    i as HMENU,                               // hMenu
                    (*(lparam as LPCREATESTRUCTW)).hInstance, // hInstance
                    null_mut(),
                ); // lpParam
            }

            0 // message processed
        }

        WM_SIZE => {
            TARGET_RECT.left = 24 * CHAR_X;
            TARGET_RECT.top = 2 * CHAR_Y;
            TARGET_RECT.right = LOWORD(lparam as DWORD) as LONG;
            TARGET_RECT.bottom = HIWORD(lparam as DWORD) as LONG;
            0 // message processed
        }

        WM_PAINT => {
            InvalidateRect(hwnd, &TARGET_RECT, TRUE);

            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);
            SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));
            SetBkMode(hdc, TRANSPARENT);

            let header1 = to_wstr(HEADER1);
            let header2 = to_wstr(HEADER2);
            TextOutW(
                hdc,
                24 * CHAR_X,
                CHAR_Y,
                header1.as_ptr(),
                header1.len() as c_int,
            );
            TextOutW(
                hdc,
                24 * CHAR_X,
                CHAR_Y,
                header2.as_ptr(),
                header2.len() as c_int,
            );

            EndPaint(hwnd, &ps);
            0 // message processed
        }

        WM_DRAWITEM | WM_COMMAND => {
            // ScrollWindow(hwnd, 0, -CHAR_Y, &TARGET_RECT, &TARGET_RECT);
            ScrollWindowEx(
                hwnd,
                0,
                -CHAR_Y,
                &TARGET_RECT,
                &TARGET_RECT,
                null_mut(),
                null_mut(),
                SW_INVALIDATE | SW_ERASE,
            );

            let hdc: HDC = GetDC(hwnd);
            SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));

            let text = to_wstr(&format!(
                "{:-16}{:04X}-{:04X}    {:04X}-{:04X}",
                if message == WM_DRAWITEM {
                    "WM_DRAWITEM"
                } else {
                    "WM_COMMAND"
                },
                HIWORD(wparam as DWORD),
                LOWORD(wparam as DWORD),
                HIWORD(lparam as DWORD),
                LOWORD(lparam as DWORD)
            ));

            TextOutW(
                hdc,
                24 * CHAR_X,
                CHAR_Y * (TARGET_RECT.bottom / CHAR_Y - 1),
                text.as_ptr(),
                text.len() as c_int,
            );

            ReleaseDC(hwnd, hdc);
            ValidateRect(hwnd, &TARGET_RECT);
            0 // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
