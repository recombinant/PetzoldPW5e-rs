// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 7 - Checker2
//
// The original source code copyright:
//
// CHECKER2.C -- Mouse Hit-Test Demo Program No. 2
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::cmp;
use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::{c_int, c_short};
use winapi::shared::minwindef::{FALSE, LPARAM, LRESULT, TRUE, UINT, WORD, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{LineTo, MoveToEx, Rectangle};
use winapi::um::winuser::{
    BeginPaint, ClientToScreen, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint,
    GetCursorPos, GetMessageW, InvalidateRect, LoadCursorW, LoadIconW, MessageBeep, MessageBoxW,
    PostQuitMessage, RegisterClassExW, ScreenToClient, SendMessageW, SetCursorPos, ShowCursor,
    ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MK_LBUTTON, MSG, PAINTSTRUCT, SW_SHOW, VK_DOWN, VK_END, VK_HOME,
    VK_LEFT, VK_RETURN, VK_RIGHT, VK_SPACE, VK_UP, WM_DESTROY, WM_KEYDOWN, WM_KILLFOCUS,
    WM_LBUTTONDOWN, WM_PAINT, WM_SETFOCUS, WM_SIZE, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

use extras::{to_wstr, GetStockBrush, MAKELPARAM, WHITE_BRUSH};

const DIVISIONS: usize = 5;

fn main() {
    let app_name = to_wstr("checker2");

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

        let caption = to_wstr("Checker2 Mouse Hit-Test Demo");
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
    static mut BLOCK_X: c_int = 0;
    static mut BLOCK_Y: c_int = 0;
    static mut STATE: [[bool; DIVISIONS]; DIVISIONS] = [[false; DIVISIONS]; DIVISIONS];

    match message {
        WM_SIZE => {
            BLOCK_X = GET_X_LPARAM(lparam) / DIVISIONS as c_int;
            BLOCK_Y = GET_Y_LPARAM(lparam) / DIVISIONS as c_int;
            0 // message processed
        }

        WM_SETFOCUS => {
            ShowCursor(TRUE);
            0 // message processed
        }

        WM_KILLFOCUS => {
            ShowCursor(FALSE);
            0 // message processed
        }

        WM_KEYDOWN => {
            let mut point: POINT = mem::MaybeUninit::uninit().assume_init();
            GetCursorPos(&mut point);
            ScreenToClient(hwnd, &mut point);

            let divisions = DIVISIONS as c_int;

            let mut x: c_int = cmp::max(0, cmp::min(divisions - 1, point.x / BLOCK_X));
            let mut y: c_int = cmp::max(0, cmp::min(divisions - 1, point.y / BLOCK_Y));

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
                VK_RETURN | VK_SPACE => {
                    let lp: LPARAM = MAKELPARAM(
                        (x * BLOCK_X) as c_short as WORD,
                        (y * BLOCK_Y) as c_short as WORD,
                    );
                    SendMessageW(hwnd, WM_LBUTTONDOWN, MK_LBUTTON, lp);
                }
                _ => {}
            }
            x = (x + divisions) % divisions;
            y = (y + divisions) % divisions;

            point.x = x * BLOCK_X + BLOCK_X / 2;
            point.y = y * BLOCK_Y + BLOCK_Y / 2;

            ClientToScreen(hwnd, &mut point);
            SetCursorPos(point.x, point.y);
            0 // message processed
        }

        WM_LBUTTONDOWN => {
            let x = GET_X_LPARAM(lparam) / BLOCK_X;
            let y = GET_Y_LPARAM(lparam) / BLOCK_Y;

            if x < DIVISIONS as c_int && y < DIVISIONS as c_int {
                STATE[x as usize][y as usize] = !STATE[x as usize][y as usize];

                let rect = RECT {
                    left: x * BLOCK_X,
                    top: y * BLOCK_Y,
                    right: (x + 1) * BLOCK_X,
                    bottom: (y + 1) * BLOCK_Y,
                };

                InvalidateRect(hwnd, &rect, FALSE);
            } else {
                MessageBeep(0);
            }
            0 // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            for x in 0..DIVISIONS as c_int {
                for y in 0..DIVISIONS as c_int {
                    Rectangle(
                        hdc,
                        x * BLOCK_X,
                        y * BLOCK_Y,
                        (x + 1) * BLOCK_X,
                        (y + 1) * BLOCK_Y,
                    );

                    if STATE[x as usize][y as usize] {
                        MoveToEx(hdc, x * BLOCK_X, y * BLOCK_Y, null_mut());
                        LineTo(hdc, (x + 1) * BLOCK_X, (y + 1) * BLOCK_Y);
                        MoveToEx(hdc, x * BLOCK_X, (y + 1) * BLOCK_Y, null_mut());
                        LineTo(hdc, (x + 1) * BLOCK_X, y * BLOCK_Y);
                    }
                }
            }

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
