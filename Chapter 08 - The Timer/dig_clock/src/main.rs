// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 8 - DigClock
//
// The original source code copyright:
//
// DIGCLOCK.C −− Digital Clock
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HDC, HWND, POINT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::sysinfoapi::GetLocalTime;
use winapi::um::wingdi::{
    CreateSolidBrush, OffsetWindowOrgEx, Polygon, SetMapMode, SetViewportExtEx, SetViewportOrgEx,
    SetWindowExtEx, SetWindowOrgEx, RGB,
};
use winapi::um::winnls::GetLocaleInfoEx;
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetMessageW,
    InvalidateRect, KillTimer, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage,
    RegisterClassExW, SetTimer, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_CREATE,
    WM_DESTROY, WM_PAINT, WM_SETTINGCHANGE, WM_SIZE, WM_TIMER, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, DeleteBrush, GetStockBrush, GetStockPen, SelectBrush, SelectPen,
    LOCALE_NAME_USER_DEFAULT, LOCALE_STIMEFORMAT, MM_ISOTROPIC, NULL_PEN, WHITE_BRUSH,
};

const ID_TIMER: usize = 1;

fn main() {
    let app_name = to_wstr("dig_clock");

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
            return; //   premature exit
        }

        let caption = to_wstr("Digital Clock");
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
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut HBRUSH_RED: HBRUSH = null_mut();
    static mut USE_24HR: bool = true;
    static mut SUPPRESS: bool = false; // suppress leading zeroes

    match message {
        WM_CREATE | WM_SETTINGCHANGE => {
            if message == WM_CREATE {
                HBRUSH_RED = CreateSolidBrush(RGB(255, 0, 0));
                SetTimer(hwnd, ID_TIMER, 1000, None);
            }

            // Documentation for LOCALE_STIMEFORMAT states 80 as the maximum
            // number of characters that will be returned.
            const MAX_LEN: usize = 80;
            let mut locale_buffer: Vec<u16> = vec![0; MAX_LEN];
            // len includes the trailing \0
            let len = GetLocaleInfoEx(
                LOCALE_NAME_USER_DEFAULT,
                LOCALE_STIMEFORMAT,
                locale_buffer.as_mut_ptr(),
                MAX_LEN as c_int,
            );
            if len > 0 {
                let fmt = OsString::from_wide(&locale_buffer[..(len - 1) as usize])
                    .into_string()
                    .unwrap();
                USE_24HR = fmt.contains('H'); // any capital H means 24hr
                SUPPRESS = !fmt.to_lowercase().contains("hh"); // single "h" means suppress
            } else {
                // call GetLastError() to find out what went wrong...
            }

            InvalidateRect(hwnd, null(), TRUE);
            0 // message processed
        }

        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);
            0 // message processed
        }

        WM_TIMER => {
            InvalidateRect(hwnd, null(), TRUE);
            0 // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            SetMapMode(hdc, MM_ISOTROPIC);
            SetWindowExtEx(hdc, 276, 72, null_mut());
            SetViewportExtEx(hdc, CLIENT_WIDTH, CLIENT_HEIGHT, null_mut());

            SetWindowOrgEx(hdc, 138, 36, null_mut());
            SetViewportOrgEx(hdc, CLIENT_WIDTH / 2, CLIENT_HEIGHT / 2, null_mut());

            SelectPen(hdc, GetStockPen(NULL_PEN));
            SelectBrush(hdc, HBRUSH_RED);

            display_time(hdc, USE_24HR, SUPPRESS);

            EndPaint(hwnd, &ps);
            0 // message processed
        }

        WM_DESTROY => {
            KillTimer(hwnd, ID_TIMER);
            DeleteBrush(HBRUSH_RED);
            PostQuitMessage(0);
            0 // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn display_digit(hdc: HDC, num: usize) {
    static SEVEN_SEGMENT: [[bool; 7]; 10] = [
        [true, true, true, false, true, true, true],     // 0
        [false, false, true, false, false, true, false], // 1
        [true, false, true, true, true, false, true],    // 2
        [true, false, true, true, false, true, true],    // 3
        [false, true, true, true, false, true, false],   // 4
        [true, true, false, true, false, true, true],    // 5
        [true, true, false, true, true, true, true],     // 6
        [true, false, true, false, false, true, false],  // 7
        [true, true, true, true, true, true, true],      // 8
        [true, true, true, true, false, true, true],
    ]; // 9

    static SEGMENTS: [[POINT; 6]; 7] = [
        [
            POINT { x: 7, y: 6 },
            POINT { x: 11, y: 2 },
            POINT { x: 31, y: 2 },
            POINT { x: 35, y: 6 },
            POINT { x: 31, y: 10 },
            POINT { x: 11, y: 10 },
        ],
        [
            POINT { x: 6, y: 7 },
            POINT { x: 10, y: 11 },
            POINT { x: 10, y: 31 },
            POINT { x: 6, y: 35 },
            POINT { x: 2, y: 31 },
            POINT { x: 2, y: 11 },
        ],
        [
            POINT { x: 36, y: 7 },
            POINT { x: 40, y: 11 },
            POINT { x: 40, y: 31 },
            POINT { x: 36, y: 35 },
            POINT { x: 32, y: 31 },
            POINT { x: 32, y: 11 },
        ],
        [
            POINT { x: 7, y: 36 },
            POINT { x: 11, y: 32 },
            POINT { x: 31, y: 32 },
            POINT { x: 35, y: 36 },
            POINT { x: 31, y: 40 },
            POINT { x: 11, y: 40 },
        ],
        [
            POINT { x: 6, y: 37 },
            POINT { x: 10, y: 41 },
            POINT { x: 10, y: 61 },
            POINT { x: 6, y: 65 },
            POINT { x: 2, y: 61 },
            POINT { x: 2, y: 41 },
        ],
        [
            POINT { x: 36, y: 37 },
            POINT { x: 40, y: 41 },
            POINT { x: 40, y: 61 },
            POINT { x: 36, y: 65 },
            POINT { x: 32, y: 61 },
            POINT { x: 32, y: 41 },
        ],
        [
            POINT { x: 7, y: 66 },
            POINT { x: 11, y: 62 },
            POINT { x: 31, y: 62 },
            POINT { x: 35, y: 66 },
            POINT { x: 31, y: 70 },
            POINT { x: 11, y: 70 },
        ],
    ];

    for (idx, segment) in SEGMENTS.iter().enumerate() {
        if SEVEN_SEGMENT[num][idx] {
            Polygon(hdc, &segment[0], 6);
        }
    }
}

unsafe fn display_time(hdc: HDC, use_24hr: bool, suppress: bool) {
    let mut st: SYSTEMTIME = mem::MaybeUninit::uninit().assume_init();
    GetLocalTime(&mut st);

    let mut num = st.wHour as usize;
    if !use_24hr && num < 13 {
        num %= 12;
    }
    display_two_digits(hdc, num, suppress);

    display_colon(hdc);
    display_two_digits(hdc, st.wMinute as usize, false);
    display_colon(hdc);
    display_two_digits(hdc, st.wSecond as usize, false);
}

unsafe fn display_two_digits(hdc: HDC, num: usize, suppress: bool) {
    if !suppress || (num / 10 != 0) {
        display_digit(hdc, num / 10);
    }
    OffsetWindowOrgEx(hdc, -42, 0, null_mut());
    display_digit(hdc, num % 10);
    OffsetWindowOrgEx(hdc, -42, 0, null_mut());
}

unsafe fn display_colon(hdc: HDC) {
    const COLON: [[POINT; 4]; 2] = [
        [
            POINT { x: 2, y: 21 },
            POINT { x: 6, y: 17 },
            POINT { x: 10, y: 21 },
            POINT { x: 6, y: 25 },
        ],
        [
            POINT { x: 2, y: 51 },
            POINT { x: 6, y: 47 },
            POINT { x: 10, y: 51 },
            POINT { x: 6, y: 55 },
        ],
    ];

    Polygon(hdc, &COLON[0][0], 4);
    Polygon(hdc, &COLON[1][0], 4);

    OffsetWindowOrgEx(hdc, -12, 0, null_mut());
}
