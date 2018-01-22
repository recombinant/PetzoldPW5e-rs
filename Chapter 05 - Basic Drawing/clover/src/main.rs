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

#![cfg(windows)] extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, SetCursor,
                          ShowCursor,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_DESTROY, WM_PAINT, WM_SIZE,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT,
                          IDC_WAIT, };
use winapi::um::wingdi::{MoveToEx, LineTo, CreateEllipticRgn,
                         CreateRectRgn, SelectClipRgn, SetViewportOrgEx, };
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, HRGN, FALSE, TRUE, };
use winapi::shared::windef::{HWND, };
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM, };

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, to_wstring, GetStockBrush, UnionRgn, XorRgn, DeleteRgn, };


fn main() {
    let app_name = to_wstring("clover");
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
                        to_wstring("This program requires Windows NT!").as_ptr(),
                        app_name.as_ptr(),
                        MB_ICONERROR);
            return; //   premature exit
        }

        let caption = to_wstring("Draw a Clover");
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
            null_mut());         // lpParam: creation parameters

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
    static mut CX_CLIENT: c_int = 0;
    static mut CY_CLIENT: c_int = 0;
    static mut HRGN_CLIP: HRGN = 0 as HRGN;

    match message {
        WM_SIZE => {
            CX_CLIENT = GET_X_LPARAM(lparam);
            CY_CLIENT = GET_Y_LPARAM(lparam);

            let hcursor = SetCursor(LoadCursorW(null_mut(), IDC_WAIT));
            ShowCursor(TRUE);

            if !HRGN_CLIP.is_null() {
                DeleteRgn(HRGN_CLIP);
            }

            let mut hrgn_tmp: [HRGN; 6] = [
                CreateEllipticRgn(0, CY_CLIENT / 3, CX_CLIENT / 2, 2 * CY_CLIENT / 3),
                CreateEllipticRgn(CX_CLIENT / 2, CY_CLIENT / 3, CX_CLIENT, 2 * CY_CLIENT / 3),
                CreateEllipticRgn(CX_CLIENT / 3, 0, 2 * CX_CLIENT / 3, CY_CLIENT / 2),
                CreateEllipticRgn(CX_CLIENT / 3, CY_CLIENT / 2, 2 * CX_CLIENT / 3, CY_CLIENT),
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

            0 as LRESULT  // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SetViewportOrgEx(hdc, CX_CLIENT / 2, CY_CLIENT / 2, null_mut());
            SelectClipRgn(hdc, HRGN_CLIP);

            let radius = (CX_CLIENT as f64 / 2.0).hypot(CY_CLIENT as f64 / 2.0);

            for degree in 0..360 {
                let angle = (degree as f64).to_radians();
                MoveToEx(hdc, 0, 0, null_mut());
                LineTo(hdc,
                       (radius * angle.cos() + 0.5) as c_int,
                       (-radius * angle.sin() + 0.5) as c_int);
            }
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
