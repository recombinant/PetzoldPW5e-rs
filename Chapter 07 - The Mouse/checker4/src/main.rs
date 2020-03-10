// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - Checker4
//
// The original source code copyright:
//
// CHECKER4.C -- Mouse Hit-Test Demo Program No. 4
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::{c_int, c_long};
use winapi::shared::minwindef::{FALSE, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HMENU, HWND, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreatePen, LineTo, MoveToEx, Rectangle};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect,
    GetDlgItem, GetFocus, GetMessageW, GetParent, GetWindowLongPtrW, InvalidateRect, LoadCursorW,
    LoadIconW, MessageBeep, MessageBoxW, MoveWindow, PostQuitMessage, RegisterClassExW,
    SendMessageW, SetFocus, SetWindowLongPtrW, ShowWindow, TranslateMessage, UpdateWindow,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG,
    PAINTSTRUCT, SW_SHOW, VK_DOWN, VK_END, VK_HOME, VK_LEFT, VK_RETURN, VK_RIGHT, VK_SPACE, VK_UP,
    WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KILLFOCUS, WM_LBUTTONDOWN, WM_PAINT, WM_SETFOCUS,
    WM_SIZE, WNDCLASSEXW, WS_CHILDWINDOW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

use extras::{
    to_wstr, DeletePen, GetStockBrush, GetStockPen, GetWindowInstance, SelectBrush, SelectPen,
    BLACK_PEN, GWLP_ID, GWLP_USERDATA, NULL_BRUSH, PS_DASH, WHITE_BRUSH,
};

const DIVISIONS: usize = 5;
static CHILD_CLASS_NAME: &str = "checker3_child";
static mut FOCUS_ID: c_int = 0;

fn main() {
    let app_name = to_wstr("checker3");
    let child_class_name = to_wstr(CHILD_CLASS_NAME);

    unsafe {
        let hinstance = GetModuleHandleW(null());

        let mut wndclassex = WNDCLASSEXW {
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

        wndclassex = WNDCLASSEXW {
            lpfnWndProc: Some(child_wnd_proc),
            cbWndExtra: mem::size_of::<c_long>() as c_int,
            hIcon: null_mut(),
            lpszClassName: child_class_name.as_ptr(),
            ..wndclassex
        };

        RegisterClassExW(&wndclassex);

        let caption = to_wstr("Checker3 Mouse Hit-Test Demo");
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
    static mut HWND_CHILD: [[HWND; DIVISIONS]; DIVISIONS] = [[null_mut(); DIVISIONS]; DIVISIONS];

    match message {
        WM_CREATE => {
            let child_class_name = to_wstr(CHILD_CLASS_NAME);
            for (x, column) in HWND_CHILD.iter_mut().enumerate().take(DIVISIONS) {
                for (y, cell) in column.iter_mut().enumerate().take(DIVISIONS) {
                    *cell = CreateWindowExW(
                        0,
                        child_class_name.as_ptr(),
                        null(),
                        WS_CHILDWINDOW | WS_VISIBLE,
                        0,
                        0,
                        0,
                        0,
                        hwnd,
                        (y << 8 | x) as HMENU,
                        GetWindowInstance(hwnd),
                        null_mut(),
                    );
                }
            }
            0 as LRESULT // message processed
        }

        WM_SIZE => {
            let block_x: c_int = GET_X_LPARAM(lparam) / DIVISIONS as c_int;
            let block_y: c_int = GET_Y_LPARAM(lparam) / DIVISIONS as c_int;

            for (x, column) in HWND_CHILD.iter_mut().enumerate().take(DIVISIONS) {
                for (y, cell) in column.iter_mut().enumerate().take(DIVISIONS) {
                    MoveWindow(
                        *cell,
                        x as c_int * block_x,
                        y as c_int * block_y,
                        block_x,
                        block_y,
                        TRUE,
                    );
                }
            }
            0 as LRESULT // message processed
        }

        WM_LBUTTONDOWN => {
            MessageBeep(0);
            0 as LRESULT // message processed
        }

        // On set-focus message, set focus to child window
        WM_SETFOCUS => {
            SetFocus(GetDlgItem(hwnd, FOCUS_ID));
            0 as LRESULT // message processed
        }

        // On key-down message, possibly change the focus window
        WM_KEYDOWN => {
            let divisions = DIVISIONS as c_int;
            let mut x = FOCUS_ID & 0xFF;
            let mut y = FOCUS_ID >> 8;

            match wparam as c_int {
                VK_UP => {
                    y -= 1;
                }
                VK_DOWN => {
                    y += 1;
                }
                VK_LEFT => {
                    x -= 1;
                }
                VK_RIGHT => {
                    x += 1;
                }
                VK_HOME => {
                    x = 0;
                    y = 0;
                }
                VK_END => {
                    x = divisions - 1;
                    y = x;
                }
                _ => {
                    return 0 as LRESULT;
                }
            }

            x = (x + divisions) % divisions;
            y = (y + divisions) % divisions;

            FOCUS_ID = y << 8 | x;

            SetFocus(GetDlgItem(hwnd, FOCUS_ID));
            0 as LRESULT // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe extern "system" fn child_wnd_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_CREATE => {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0); // on/off flag
            0 as LRESULT // message processed
        }

        WM_KEYDOWN | WM_LBUTTONDOWN => {
            // Send most key presses to the parent window

            let wp = wparam as c_int;
            if message == WM_KEYDOWN && wp != VK_RETURN && wp != VK_SPACE {
                SendMessageW(GetParent(hwnd), message, wparam, lparam);
                return 0 as LRESULT;
            }
            // For Return and Space, fall through to toggle the square

            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                1 ^ GetWindowLongPtrW(hwnd, GWLP_USERDATA),
            );
            InvalidateRect(hwnd, null(), FALSE);
            0 as LRESULT // message processed
        }

        WM_SETFOCUS | WM_KILLFOCUS => {
            if message == WM_SETFOCUS {
                FOCUS_ID = GetWindowLongPtrW(hwnd, GWLP_ID) as c_int;
            }
            InvalidateRect(hwnd, null(), TRUE);
            0 as LRESULT // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rect: RECT = mem::MaybeUninit::uninit().assume_init();
            GetClientRect(hwnd, &mut rect);
            Rectangle(hdc, 0, 0, rect.right, rect.bottom);

            if GetWindowLongPtrW(hwnd, GWLP_USERDATA) != 0 {
                MoveToEx(hdc, 0, 0, null_mut());
                LineTo(hdc, rect.right, rect.bottom);
                MoveToEx(hdc, 0, rect.bottom, null_mut());
                LineTo(hdc, rect.right, 0);
            }

            // Draw the "focus" rectangle

            if hwnd == GetFocus() {
                rect.left += rect.right / 10;
                rect.right -= rect.left;
                rect.top += rect.bottom / 10;
                rect.bottom -= rect.top;

                SelectBrush(hdc, GetStockBrush(NULL_BRUSH));
                SelectPen(hdc, CreatePen(PS_DASH, 0, 0));
                Rectangle(hdc, rect.left, rect.top, rect.right, rect.bottom);
                DeletePen(SelectPen(hdc, GetStockPen(BLACK_PEN)));
            }

            EndPaint(hwnd, &ps);
            0 as LRESULT // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
