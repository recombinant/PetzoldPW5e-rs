// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 6 - SysMets4
//
// The original source code copyright:
//
// SYSMETS4.C -- System Metrics Display Program No. 4
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate sys_mets_data;
extern crate winapi;

use std::cmp;
use std::mem;
use std::ptr::{null, null_mut};
use sys_mets_data::SYS_METRICS;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::HWND;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{
    GetTextMetricsW, SetTextAlign, TextOutW, TA_LEFT, TA_RIGHT, TA_TOP, TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC, GetMessageW,
    GetScrollInfo, GetSystemMetrics, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage,
    RegisterClassExW, ReleaseDC, ScrollWindowEx, SendMessageW, SetScrollInfo, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SB_BOTTOM, SB_LINEDOWN, SB_LINELEFT,
    SB_LINERIGHT, SB_LINEUP, SB_PAGEDOWN, SB_PAGELEFT, SB_PAGERIGHT, SB_PAGEUP, SB_THUMBPOSITION,
    SB_TOP, SCROLLINFO, SIF_ALL, SIF_PAGE, SIF_POS, SIF_RANGE, SW_ERASE, SW_INVALIDATE, SW_SHOW,
    VK_DOWN, VK_END, VK_HOME, VK_LEFT, VK_NEXT, VK_PRIOR, VK_RIGHT, VK_UP, WM_CREATE, WM_DESTROY,
    WM_HSCROLL, WM_KEYDOWN, WM_PAINT, WM_SIZE, WM_VSCROLL, WNDCLASSEXW, WS_HSCROLL,
    WS_OVERLAPPEDWINDOW, WS_VSCROLL,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetStockBrush, GET_WM_HSCROLL_CODE, GET_WM_VSCROLL_CODE, SB_HORZ, SB_VERT, WHITE_BRUSH,
};

fn main() {
    let app_name = to_wstr("sys_mets4");

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

        let caption = to_wstr("Get System Metrics No. 4");
        let hwnd = CreateWindowExW(
            0,                                             // dwExStyle:
            atom as LPCWSTR,                               // lpClassName: class name or atom
            caption.as_ptr(),                              // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW | WS_VSCROLL | WS_HSCROLL, // dwStyle: window style
            CW_USEDEFAULT,                                 // x: initial x position
            CW_USEDEFAULT,                                 // y: initial y position
            CW_USEDEFAULT,                                 // nWidth: initial x size
            CW_USEDEFAULT,                                 // nHeight: initial y size
            null_mut(),                                    // hWndParent: parent window handle
            null_mut(),                                    // hMenu: window menu handle
            hinstance,                                     // hInstance: program instance handle
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
    static mut CAPS_WIDTH: c_int = 0;
    static mut CHAR_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;
    static mut CLIENT_WIDTH: c_int = 0;
    static mut CLIENT_HEIGHT: c_int = 0;
    static mut MAX_COLUMN_WIDTH: c_int = 0;

    match message {
        WM_CREATE => {
            let hdc = GetDC(hwnd);
            let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();

            GetTextMetricsW(hdc, &mut tm);
            CHAR_WIDTH = tm.tmAveCharWidth;
            CAPS_WIDTH = (if tm.tmPitchAndFamily & 1 == 1 { 3 } else { 2 }) * CHAR_WIDTH / 2;
            CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

            ReleaseDC(hwnd, hdc);

            // Save the width of the three columns

            MAX_COLUMN_WIDTH = 40 * CHAR_WIDTH + 22 * CAPS_WIDTH;

            0 as LRESULT // message processed
        }

        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);

            // Set vertical scroll bar range and page size

            let mut si = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_RANGE | SIF_PAGE,
                nMin: 0,
                nMax: SYS_METRICS.len() as c_int - 1,
                nPage: (CLIENT_HEIGHT / CHAR_HEIGHT) as UINT,
                nPos: 0,
                nTrackPos: 0,
            };

            SetScrollInfo(hwnd, SB_VERT, &si, TRUE);

            // Set horizontal scroll bar range and page size

            si = SCROLLINFO {
                nMax: 2 + MAX_COLUMN_WIDTH / CHAR_WIDTH,
                nPage: (CLIENT_WIDTH / CHAR_WIDTH) as UINT,
                ..si
            };
            SetScrollInfo(hwnd, SB_HORZ, &si, TRUE);

            0 as LRESULT // message processed
        }

        WM_VSCROLL => {
            // Get all the vertical scroll bar information

            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_ALL,
                ..mem::MaybeUninit::uninit().assume_init()
            };
            GetScrollInfo(hwnd, SB_VERT, &mut si);

            // Save the position for comparison later on

            let vert_pos = si.nPos;

            match GET_WM_VSCROLL_CODE(wparam, lparam) as LPARAM {
                SB_TOP => {
                    si.nPos = si.nMin;
                }
                SB_BOTTOM => {
                    si.nPos = si.nMax;
                }
                SB_LINEUP => {
                    si.nPos -= 1;
                }
                SB_LINEDOWN => {
                    si.nPos += 1;
                }
                SB_PAGEUP => {
                    si.nPos -= si.nPage as c_int;
                }
                SB_PAGEDOWN => {
                    si.nPos += si.nPage as c_int;
                }
                SB_THUMBPOSITION => {
                    si.nPos = si.nTrackPos;
                }
                _ => {}
            }

            // Set the position and then retrieve it.  Due to adjustments
            // by Windows it may not be the same as the value set.

            si.fMask = SIF_POS;
            SetScrollInfo(hwnd, SB_VERT, &si, TRUE);
            GetScrollInfo(hwnd, SB_VERT, &mut si);

            // If the position has changed, scroll the window.

            if si.nPos != vert_pos {
                // ScrollWindow(hwnd, 0, CHAR_HEIGHT * (vert_pos - si.nPos), null(), null());
                ScrollWindowEx(
                    hwnd,
                    0,
                    CHAR_HEIGHT * (vert_pos - si.nPos),
                    null(),
                    null(),
                    null_mut(),
                    null_mut(),
                    SW_INVALIDATE | SW_ERASE,
                );
            }

            0 as LRESULT
        }

        WM_HSCROLL => {
            // Get all the horizontal scroll bar information

            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_ALL,
                ..mem::MaybeUninit::uninit().assume_init()
            };
            GetScrollInfo(hwnd, SB_HORZ, &mut si);

            // Save the position for comparison later on

            let horz_pos = si.nPos;

            match GET_WM_HSCROLL_CODE(wparam, lparam) as LPARAM {
                SB_LINELEFT => {
                    si.nPos -= 1;
                }
                SB_LINERIGHT => {
                    si.nPos += 1;
                }
                SB_PAGELEFT => {
                    si.nPos -= si.nPage as c_int;
                }
                SB_PAGERIGHT => {
                    si.nPos += si.nPage as c_int;
                }
                SB_THUMBPOSITION => {
                    si.nPos = si.nTrackPos;
                }
                _ => {}
            }

            // Set the position and then retrieve it.  Due to adjustments
            // by Windows it may not be the same as the value set.

            si.fMask = SIF_POS;
            SetScrollInfo(hwnd, SB_HORZ, &si, TRUE);
            GetScrollInfo(hwnd, SB_HORZ, &mut si);

            // If the position has changed, scroll the window.

            if si.nPos != horz_pos {
                // ScrollWindow(hwnd, CHAR_WIDTH * (horz_pos - si.nPos), 0, null(), null());
                ScrollWindowEx(
                    hwnd,
                    CHAR_WIDTH * (horz_pos - si.nPos),
                    0,
                    null(),
                    null(),
                    null_mut(),
                    null_mut(),
                    SW_INVALIDATE | SW_ERASE,
                );
            }

            0 as LRESULT
        }

        WM_KEYDOWN => {
            match wparam as c_int {
                VK_HOME => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_TOP as WPARAM, 0);
                }
                VK_END => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_BOTTOM as WPARAM, 0);
                }
                VK_PRIOR => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_PAGEUP as WPARAM, 0);
                }
                VK_NEXT => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_PAGEDOWN as WPARAM, 0);
                }
                VK_UP => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_LINEUP as WPARAM, 0);
                }
                VK_DOWN => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_LINEDOWN as WPARAM, 0);
                }
                VK_LEFT => {
                    SendMessageW(hwnd, WM_HSCROLL, SB_PAGEUP as WPARAM, 0);
                }
                VK_RIGHT => {
                    SendMessageW(hwnd, WM_HSCROLL, SB_PAGEDOWN as WPARAM, 0);
                }
                _ => {}
            }

            0 as LRESULT // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_POS,
                ..mem::MaybeUninit::uninit().assume_init()
            };

            // Get vertical scroll bar position

            GetScrollInfo(hwnd, SB_VERT, &mut si);
            let vert_pos = si.nPos;

            // Get horizontal scroll bar position

            GetScrollInfo(hwnd, SB_HORZ, &mut si);
            let horz_pos = si.nPos;

            // Find painting limits

            let paint_beg = cmp::max(0, vert_pos + ps.rcPaint.top / CHAR_HEIGHT);
            let paint_end = cmp::min(
                SYS_METRICS.len() as c_int - 1,
                vert_pos + ps.rcPaint.bottom / CHAR_HEIGHT,
            );

            for i in paint_beg..=paint_end + 1 {
                let sys_metric = &SYS_METRICS[i as usize];
                let x = CHAR_WIDTH * (1 - horz_pos);
                let y = CHAR_HEIGHT * (i - vert_pos);

                SetTextAlign(hdc, TA_LEFT | TA_TOP);

                let label = to_wstr(sys_metric.label);
                TextOutW(hdc, x, y, label.as_ptr(), lstrlenW(label.as_ptr()));

                let desc = to_wstr(sys_metric.desc);
                TextOutW(
                    hdc,
                    x + 22 * CAPS_WIDTH,
                    y,
                    desc.as_ptr(),
                    lstrlenW(desc.as_ptr()),
                );

                SetTextAlign(hdc, TA_RIGHT | TA_TOP);

                let metric = to_wstr(&format!("{:5}", GetSystemMetrics(sys_metric.index)));
                TextOutW(
                    hdc,
                    x + 22 * CAPS_WIDTH + 40 * CHAR_WIDTH,
                    y,
                    metric.as_ptr(),
                    lstrlenW(metric.as_ptr()),
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
