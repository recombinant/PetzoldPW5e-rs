// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 8 - whatclr
//
// The original source code copyright:
//
// WHATCLR.C -- Displays Color Under Cursor
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{FALSE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{COLORREF, HDC, HWND, POINT, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{
    CreateDCW, CreateICW, DeleteDC, GetBValue, GetGValue, GetPixel, GetRValue, GetTextMetricsW,
    TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawTextW, EndPaint,
    GetClientRect, GetCursorPos, GetMessageW, GetSystemMetrics, InvalidateRect, KillTimer,
    LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, SetTimer, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, DT_CENTER,
    DT_SINGLELINE, DT_VCENTER, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT,
    SM_CXBORDER, SM_CYBORDER, SM_CYCAPTION, SW_SHOW, WM_CREATE, WM_DESTROY, WM_DISPLAYCHANGE,
    WM_PAINT, WM_TIMER, WNDCLASSEXW, WS_BORDER, WS_CAPTION, WS_OVERLAPPED, WS_SYSMENU,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, WHITE_BRUSH};

const ID_TIMER: usize = 1;

fn main() {
    let app_name = to_wstr("what_clr");

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

        let (width, height): (c_int, c_int) = find_window_size();

        let caption = to_wstr("What Color");
        let hwnd = CreateWindowExW(
            0,                                                   // dwExStyle:
            atom as LPCWSTR,                                     // lpClassName: class name or atom
            caption.as_ptr(),                                    // lpWindowName: window caption
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_BORDER, // dwStyle: window style
            CW_USEDEFAULT,                                       // x: initial x position
            CW_USEDEFAULT,                                       // y: initial y position
            width,                                               // nWidth: initial x size
            height,                                              // nHeight: initial y size
            null_mut(),                                          // hWndParent: parent window handle
            null_mut(),                                          // hMenu: window menu handle
            hinstance, // hInstance: program instance handle
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
    static mut CR: COLORREF = 0;
    static mut CR_LAST: COLORREF = 0;
    static mut HDC_SCREEN: HDC = null_mut();

    match message {
        WM_CREATE => {
            let text: Vec<u16> = to_wstr("DISPLAY");
            HDC_SCREEN = CreateDCW(text.as_ptr(), null(), null(), null());
            SetTimer(hwnd, ID_TIMER, 100, None);
            0 as LRESULT // message processed
        }

        WM_DISPLAYCHANGE => {
            let text: Vec<u16> = to_wstr("DISPLAY");
            DeleteDC(HDC_SCREEN);
            HDC_SCREEN = CreateDCW(text.as_ptr(), null(), null(), null());
            0 as LRESULT // message processed
        }

        WM_TIMER => {
            let mut pt: POINT = mem::MaybeUninit::uninit().assume_init();
            GetCursorPos(&mut pt);
            CR = GetPixel(HDC_SCREEN, pt.x, pt.y);

            if CR != CR_LAST {
                CR_LAST = CR;
                InvalidateRect(hwnd, null(), FALSE);
            }
            0 as LRESULT // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rc: RECT = mem::MaybeUninit::uninit().assume_init();
            GetClientRect(hwnd, &mut rc);

            let r = GetRValue(CR);
            let g = GetGValue(CR);
            let b = GetBValue(CR);

            let buffer = to_wstr(&format!("  {:02X} {:02X} {:02X}  ", r, g, b));

            DrawTextW(
                hdc,
                buffer.as_ptr(),
                -1,
                &mut rc,
                DT_SINGLELINE | DT_CENTER | DT_VCENTER,
            );

            EndPaint(hwnd, &ps);
            0 as LRESULT // message processed
        }

        WM_DESTROY => {
            KillTimer(hwnd, ID_TIMER);
            PostQuitMessage(0);
            0 as LRESULT // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn find_window_size() -> (c_int, c_int) {
    let text: Vec<u16> = to_wstr("DISPLAY");
    let hdc_screen: HDC = CreateICW(text.as_ptr(), null(), null(), null());
    let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();
    GetTextMetricsW(hdc_screen, &mut tm);
    DeleteDC(hdc_screen);

    let window_x: c_int = 2 * GetSystemMetrics(SM_CXBORDER) + 12 * tm.tmAveCharWidth;

    let window_y: c_int =
        2 * GetSystemMetrics(SM_CYBORDER) + GetSystemMetrics(SM_CYCAPTION) + 2 * tm.tmHeight;

    (window_x, window_y)
}
