// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - OwnDraw
//
// The original source code copyright:
//
// OWNDRAW.C --  Owner−Draw Button Demo Program
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
#![allow(clippy::identity_op)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{
    DWORD, HINSTANCE, HIWORD, LOWORD, LPARAM, LRESULT, TRUE, UINT, WPARAM,
};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HDC, HMENU, HWND, POINT, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::Polygon;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawFocusRect, FillRect, FrameRect,
    GetDialogBaseUnits, GetMessageW, GetWindowRect, InvertRect, LoadCursorW, LoadIconW,
    MessageBoxW, MoveWindow, PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage,
    UpdateWindow, BS_OWNERDRAW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION,
    LPDRAWITEMSTRUCT, MB_ICONERROR, MSG, ODS_FOCUS, ODS_SELECTED, SW_SHOW, WM_COMMAND, WM_CREATE,
    WM_DESTROY, WM_DRAWITEM, WM_SIZE, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, GetStockBrush, SelectBrush, BLACK_BRUSH, WHITE_BRUSH};

static mut GLOBAL_HINST: HINSTANCE = null_mut();
const ID_SMALLER: UINT = 1;
const ID_LARGER: UINT = 2;

fn main() {
    let app_name = to_wstr("own_draw");

    unsafe {
        GLOBAL_HINST = GetModuleHandleW(null());

        let wndclassex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GLOBAL_HINST,
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

        let caption = to_wstr("Owner-Draw Button Demo");
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
            GLOBAL_HINST,        // hInstance: program instance handle
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
    static mut HWND_SMALLER: HWND = null_mut();
    static mut HWND_LARGER: HWND = null_mut();
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut CHAR_X: c_int = 0;
    static mut CHAR_Y: c_int = 0;
    let btn_width = || 8 * CHAR_X;
    let btn_height = || 4 * CHAR_Y;

    match message {
        WM_CREATE => {
            CHAR_X = LOWORD(GetDialogBaseUnits() as DWORD) as c_int;
            CHAR_Y = HIWORD(GetDialogBaseUnits() as DWORD) as c_int;

            // Create the owner-draw pushbuttons

            let text = to_wstr("button");
            let blank = to_wstr("");
            HWND_SMALLER = CreateWindowExW(
                0,
                text.as_ptr(),
                blank.as_ptr(),
                WS_CHILD | WS_VISIBLE | BS_OWNERDRAW,
                0,
                0,
                btn_width(),
                btn_height(),
                hwnd,
                ID_SMALLER as HMENU,
                GLOBAL_HINST,
                null_mut(),
            );

            HWND_LARGER = CreateWindowExW(
                0,
                text.as_ptr(),
                blank.as_ptr(),
                WS_CHILD | WS_VISIBLE | BS_OWNERDRAW,
                0,
                0,
                btn_width(),
                btn_height(),
                hwnd,
                ID_LARGER as HMENU,
                GLOBAL_HINST,
                null_mut(),
            );

            0 as LRESULT // message processed
        }

        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            MoveWindow(
                HWND_SMALLER,
                CLIENT_WIDTH / 2 - 3 * btn_width() / 2,
                CLIENT_HEIGHT / 2 - btn_height() / 2,
                btn_width(),
                btn_height(),
                TRUE,
            );

            MoveWindow(
                HWND_LARGER,
                CLIENT_WIDTH / 2 + btn_width() / 2,
                CLIENT_HEIGHT / 2 - btn_height() / 2,
                btn_width(),
                btn_height(),
                TRUE,
            );

            0 as LRESULT // message processed
        }

        WM_COMMAND => {
            let mut rc: RECT = mem::MaybeUninit::uninit().assume_init();
            GetWindowRect(hwnd, &mut rc);

            // Make the window 10% smaller or larger

            match wparam as UINT {
                ID_SMALLER => {
                    rc.left += CLIENT_WIDTH / 20;
                    rc.right -= CLIENT_WIDTH / 20;
                    rc.top += CLIENT_HEIGHT / 20;
                    rc.bottom -= CLIENT_HEIGHT / 20;
                }

                ID_LARGER => {
                    rc.left -= CLIENT_WIDTH / 20;
                    rc.right += CLIENT_WIDTH / 20;
                    rc.top -= CLIENT_HEIGHT / 20;
                    rc.bottom += CLIENT_HEIGHT / 20;
                }

                _ => {}
            }

            MoveWindow(
                hwnd,
                rc.left,
                rc.top,
                rc.right - rc.left,
                rc.bottom - rc.top,
                TRUE,
            );
            0 as LRESULT // message processed
        }

        WM_DRAWITEM => {
            let dis = *(lparam as LPDRAWITEMSTRUCT);

            // Fill area with white and frame it black

            FillRect(dis.hDC, &dis.rcItem, GetStockBrush(WHITE_BRUSH));
            FrameRect(dis.hDC, &dis.rcItem, GetStockBrush(BLACK_BRUSH));

            // Draw inward and outward black triangles

            let cx = dis.rcItem.right - dis.rcItem.left;
            let cy = dis.rcItem.bottom - dis.rcItem.top;

            let mut pt: [POINT; 3] = mem::MaybeUninit::uninit().assume_init();

            match dis.CtlID {
                ID_SMALLER => {
                    pt[0].x = 3 * cx / 8;
                    pt[0].y = 1 * cy / 8;
                    pt[1].x = 5 * cx / 8;
                    pt[1].y = 1 * cy / 8;
                    pt[2].x = 4 * cx / 8;
                    pt[2].y = 3 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 7 * cx / 8;
                    pt[0].y = 3 * cy / 8;
                    pt[1].x = 7 * cx / 8;
                    pt[1].y = 5 * cy / 8;
                    pt[2].x = 5 * cx / 8;
                    pt[2].y = 4 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 5 * cx / 8;
                    pt[0].y = 7 * cy / 8;
                    pt[1].x = 3 * cx / 8;
                    pt[1].y = 7 * cy / 8;
                    pt[2].x = 4 * cx / 8;
                    pt[2].y = 5 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 1 * cx / 8;
                    pt[0].y = 5 * cy / 8;
                    pt[1].x = 1 * cx / 8;
                    pt[1].y = 3 * cy / 8;
                    pt[2].x = 3 * cx / 8;
                    pt[2].y = 4 * cy / 8;

                    triangle(dis.hDC, &pt);
                }

                ID_LARGER => {
                    pt[0].x = 5 * cx / 8;
                    pt[0].y = 3 * cy / 8;
                    pt[1].x = 3 * cx / 8;
                    pt[1].y = 3 * cy / 8;
                    pt[2].x = 4 * cx / 8;
                    pt[2].y = 1 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 5 * cx / 8;
                    pt[0].y = 5 * cy / 8;
                    pt[1].x = 5 * cx / 8;
                    pt[1].y = 3 * cy / 8;
                    pt[2].x = 7 * cx / 8;
                    pt[2].y = 4 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 3 * cx / 8;
                    pt[0].y = 5 * cy / 8;
                    pt[1].x = 5 * cx / 8;
                    pt[1].y = 5 * cy / 8;
                    pt[2].x = 4 * cx / 8;
                    pt[2].y = 7 * cy / 8;

                    triangle(dis.hDC, &pt);

                    pt[0].x = 3 * cx / 8;
                    pt[0].y = 3 * cy / 8;
                    pt[1].x = 3 * cx / 8;
                    pt[1].y = 5 * cy / 8;
                    pt[2].x = 1 * cx / 8;
                    pt[2].y = 4 * cy / 8;

                    triangle(dis.hDC, &pt);
                }

                _ => {}
            }

            // Invert the rectangle if the button is selected

            if dis.itemState & ODS_SELECTED != 0 {
                InvertRect(dis.hDC, &dis.rcItem);
            }

            // Draw a focus rectangle if the button has the focus

            if dis.itemState & ODS_FOCUS != 0 {
                let rc_focus = RECT {
                    left: dis.rcItem.left + cx / 16,
                    top: dis.rcItem.top + cy / 16,
                    right: dis.rcItem.right - cx / 16,
                    bottom: dis.rcItem.bottom - cy / 16,
                };
                DrawFocusRect(dis.hDC, &rc_focus);
            }
            0 as LRESULT // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn triangle(hdc: HDC, pt: &[POINT; 3]) {
    SelectBrush(hdc, GetStockBrush(BLACK_BRUSH));
    Polygon(hdc, &pt[0], pt.len() as c_int);
    SelectBrush(hdc, GetStockBrush(WHITE_BRUSH));
}
