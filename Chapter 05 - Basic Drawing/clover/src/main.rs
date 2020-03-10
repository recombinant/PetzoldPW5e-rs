// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - Clover
//
// The original source code copyright:
//
// CLOVER.C -- Clover Drawing Program Using Regions
//             (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{FALSE, HRGN, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::HWND;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{
    CreateEllipticRgn, CreateRectRgn, LineTo, MoveToEx, SelectClipRgn, SetViewportOrgEx,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetMessageW,
    LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, SetCursor, ShowCursor,
    ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDC_WAIT, IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_DESTROY, WM_PAINT,
    WM_SIZE, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{to_wstr, DeleteRgn, GetStockBrush, UnionRgn, XorRgn, WHITE_BRUSH};

fn main() {
    let app_name = to_wstr("clover");

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

        let caption = to_wstr("Draw a Clover");
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
    static mut HRGN_CLIP: HRGN = 0 as HRGN;

    match message {
        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            let hcursor = SetCursor(LoadCursorW(null_mut(), IDC_WAIT));
            ShowCursor(TRUE);

            if !HRGN_CLIP.is_null() {
                DeleteRgn(HRGN_CLIP);
            }

            let hrgn_tmp: [HRGN; 6] = [
                CreateEllipticRgn(
                    0,
                    CLIENT_HEIGHT / 3,
                    CLIENT_WIDTH / 2,
                    2 * CLIENT_HEIGHT / 3,
                ),
                CreateEllipticRgn(
                    CLIENT_WIDTH / 2,
                    CLIENT_HEIGHT / 3,
                    CLIENT_WIDTH,
                    2 * CLIENT_HEIGHT / 3,
                ),
                CreateEllipticRgn(CLIENT_WIDTH / 3, 0, 2 * CLIENT_WIDTH / 3, CLIENT_HEIGHT / 2),
                CreateEllipticRgn(
                    CLIENT_WIDTH / 3,
                    CLIENT_HEIGHT / 2,
                    2 * CLIENT_WIDTH / 3,
                    CLIENT_HEIGHT,
                ),
                CreateRectRgn(0, 0, 1, 1),
                CreateRectRgn(0, 0, 1, 1),
            ];
            HRGN_CLIP = CreateRectRgn(0, 0, 1, 1);

            UnionRgn(hrgn_tmp[4], hrgn_tmp[0], hrgn_tmp[1]);
            UnionRgn(hrgn_tmp[5], hrgn_tmp[2], hrgn_tmp[3]);
            XorRgn(HRGN_CLIP, hrgn_tmp[4], hrgn_tmp[5]);

            //  // rustc 1.23.0 requires some ugly casting here...
            //
            //  for hrgn in hrgn_tmp.iter_mut() {
            //      DeleteRgn(hrgn as *mut HRGN as HRGN);
            //  }

            // Iterate over indices, saves unnecessary casting
            for i in 0..hrgn_tmp.len() {
                DeleteRgn(hrgn_tmp[i]);
            }

            SetCursor(hcursor);
            ShowCursor(FALSE);

            0 as LRESULT // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            SetViewportOrgEx(hdc, CLIENT_WIDTH / 2, CLIENT_HEIGHT / 2, null_mut());
            SelectClipRgn(hdc, HRGN_CLIP);

            let radius = (CLIENT_WIDTH as f64 / 2.0).hypot(CLIENT_HEIGHT as f64 / 2.0);

            for degree in 0..360 {
                let angle = (degree as f64).to_radians();
                MoveToEx(hdc, 0, 0, null_mut());
                LineTo(
                    hdc,
                    (radius * angle.cos() + 0.5) as c_int,
                    (-radius * angle.sin() + 0.5) as c_int,
                );
            }
            EndPaint(hwnd, &ps);

            0 as LRESULT // message processed
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0 as LRESULT // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
