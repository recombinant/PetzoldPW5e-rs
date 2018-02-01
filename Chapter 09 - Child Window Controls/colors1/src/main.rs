// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - Colors1
//
// The original source code copyright:
//
// COLORS1.C -- Colors Using Scroll Bars
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;


use std::mem;
use std::cmp;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_int;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          MessageBoxW, LoadIconW, LoadCursorW,
                          SetScrollInfo,
                          GetWindowLongPtrW, GetSysColor,
                          GetDialogBaseUnits, SetRect, MoveWindow, SetFocus, SetClassLongPtrW,
                          GetDlgItem, GetParent, GetKeyState, InvalidateRect,
                          SetWindowTextW,
                          MSG, WNDCLASSEXW, SCROLLINFO,
                          WM_CREATE, WM_DESTROY, WM_SIZE, WM_VSCROLL,
                          WM_KEYDOWN, WM_SETFOCUS, WM_CTLCOLORSCROLLBAR, WM_CTLCOLORSTATIC,
                          WM_SYSCOLORCHANGE, WM_NCDESTROY,
                          WS_OVERLAPPEDWINDOW, WS_CHILD, WS_VISIBLE,
                          WS_TABSTOP, COLOR_BTNHIGHLIGHT,
                          SW_SHOW, SS_WHITERECT, SS_CENTER, SBS_VERT,
                          CS_HREDRAW, CS_VREDRAW, IDC_ARROW, IDI_APPLICATION,
                          MB_ICONERROR, CW_USEDEFAULT,
                          SIF_RANGE, SIF_POS, VK_SHIFT, VK_TAB, };
use winapi::um::wingdi::{CreateSolidBrush, SetTextColor, SetBkColor, RGB, };
use winapi::um::commctrl::{DefSubclassProc, SetWindowSubclass, RemoveWindowSubclass, };
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::minwindef::{BYTE, UINT, DWORD, WPARAM, LPARAM, LRESULT, TRUE, FALSE,
                                HIWORD, LOWORD, };
use winapi::shared::windef::{HWND, COLORREF, HBRUSH, RECT, HMENU, HDC, };
use winapi::shared::basetsd::{UINT_PTR, DWORD_PTR, LONG_PTR, };
use winapi::shared::ntdef::LPCWSTR;

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, to_wstr, GetStockBrush, DeleteBrush, GetWindowInstance,
             SB_THUMBPOSITION, SB_THUMBTRACK, SB_PAGEDOWN, SB_PAGEUP, SB_LINEDOWN, SB_LINEUP,
             SB_BOTTOM, SB_TOP, SB_CTL, GWLP_ID, GCLP_HBRBACKGROUND, };


const NUM_CTRLS: usize = 3;
static mut ID_FOCUS: usize = 0;


fn main() {
    let app_name = to_wstr("colors1");

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
            hbrBackground: CreateSolidBrush(0),  // black
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

        let caption = to_wstr("Color Scroll");
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
    static PRIMARY_COLORS: [COLORREF; NUM_CTRLS] = [0x0000ff, 0x00ff00, 0xff0000];  // r g b
    static mut HBRUSH_ARRAY: [HBRUSH; NUM_CTRLS] = [0 as HBRUSH; NUM_CTRLS];
    static mut HBRUSH_STATIC: HBRUSH = 0 as HBRUSH;
    static mut HWND_SCROLL: [HWND; NUM_CTRLS] = [0 as HWND; NUM_CTRLS];
    static mut HWND_LABEL: [HWND; NUM_CTRLS] = [0 as HWND; NUM_CTRLS];
    static mut HWND_VALUE: [HWND; NUM_CTRLS] = [0 as HWND; NUM_CTRLS];
    static mut HWND_RECT: HWND = 0 as HWND;
    static mut CTRL_COLOR: [BYTE; NUM_CTRLS] = [0; NUM_CTRLS];
    static mut CHAR_Y: c_int = 0;
    static mut RECT_COLOR: RECT = RECT { left: 0, right: 0, top: 0, bottom: 0 };
    static COLOR_LABELS: [&'static str; NUM_CTRLS] = ["Red", "Green", "Blue"];

    match message {
        WM_CREATE => {
            let hinstance = GetWindowInstance(hwnd);

            // Create the white-rectangle window against which the
            // scroll bars will be positioned. The child window ID is 9.

            let text = to_wstr("static");
            HWND_RECT = CreateWindowExW(0, text.as_ptr(), null(),
                                       WS_CHILD | WS_VISIBLE | SS_WHITERECT,
                                       0, 0, 0, 0,
                                       hwnd, 9 as HMENU, hinstance, null_mut());

            let text_scrollbar = to_wstr("scrollbar");
            let text_rgb: [Vec<u16>; NUM_CTRLS] = [
                to_wstr(COLOR_LABELS[0]),
                to_wstr(COLOR_LABELS[1]),
                to_wstr(COLOR_LABELS[2])];
            let text_zero = to_wstr("0");

            for i in 0..NUM_CTRLS {
                // The three scroll bars have IDs 0, 1, and 2, with
                // scroll bar ranges from 0 through 255.

                HWND_SCROLL[i] = CreateWindowExW(0, text_scrollbar.as_ptr(), null(),
                                                WS_CHILD | WS_VISIBLE | WS_TABSTOP | SBS_VERT,
                                                0, 0, 0, 0,
                                                hwnd, i as HMENU, hinstance, null_mut());

                let mut si = SCROLLINFO {
                    cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                    fMask: SIF_RANGE | SIF_POS,
                    nMin: 0,
                    nMax: 255,
                    nPos: 0,
                    ..mem::uninitialized()
                };

                SetScrollInfo(HWND_SCROLL[i], SB_CTL, &si, FALSE);
                // SetScrollRange (HWND_SCROLL[i], SB_CTL, 0, 255, FALSE) ;
                // SetScrollPos   (HWND_SCROLL[i], SB_CTL, 0, FALSE) ;


                // The three CTRL_COLOR-name labels have IDs 3, 4, and 5,
                // and text strings "Red", "Green", and "Blue".

                HWND_LABEL[i] = CreateWindowExW(0, text.as_ptr(), text_rgb[i].as_ptr(),
                                               WS_CHILD | WS_VISIBLE | SS_CENTER,
                                               0, 0, 0, 0,
                                               hwnd, (i + 3) as HMENU,
                                               hinstance, null_mut());

                // The three CTRL_COLOR-value text fields have IDs 6, 7,
                // and 8, and initial text strings of "0".

                HWND_VALUE[i] = CreateWindowExW(0, text.as_ptr(), text_zero.as_ptr(),
                                               WS_CHILD | WS_VISIBLE | SS_CENTER,
                                               0, 0, 0, 0,
                                               hwnd, (i + 6) as HMENU,
                                               hinstance, null_mut());

                SetWindowSubclass(HWND_SCROLL[i], Some(scroll_proc), 0, 0);

                HBRUSH_ARRAY[i] = CreateSolidBrush(PRIMARY_COLORS[i]);
            }

            HBRUSH_STATIC = CreateSolidBrush(GetSysColor(COLOR_BTNHIGHLIGHT));

            CHAR_Y = HIWORD(GetDialogBaseUnits() as DWORD) as c_int;

            0 as LRESULT  // message processed
        }

        WM_SIZE => {
            let client_width = GET_X_LPARAM(lparam);
            let client_height = GET_Y_LPARAM(lparam);

            SetRect(&mut RECT_COLOR, client_width / 2, 0, client_width, client_height);

            MoveWindow(HWND_RECT, 0, 0, client_width / 2, client_height, TRUE);

            for i in 0..NUM_CTRLS {
                let ii = i as c_int;
                MoveWindow(HWND_SCROLL[i],
                           (2 * ii + 1) * client_width / 14, 2 * CHAR_Y,
                           client_width / 14, client_height - 4 * CHAR_Y, TRUE);

                MoveWindow(HWND_LABEL[i],
                           (4 * ii + 1) * client_width / 28, CHAR_Y / 2,
                           client_width / 7, CHAR_Y, TRUE);

                MoveWindow(HWND_VALUE[i],
                           (4 * ii + 1) * client_width / 28,
                           client_height - 3 * CHAR_Y / 2,
                           client_width / 7, CHAR_Y, TRUE);
            }
            SetFocus(hwnd);
            0 as LRESULT  // message processed
        }

        WM_SETFOCUS => {
            SetFocus(HWND_SCROLL[ID_FOCUS]);
            0 as LRESULT  // message processed
        }

        WM_VSCROLL => {
            let i = GetWindowLongPtrW(lparam as HWND, GWLP_ID) as usize;

            let request = LOWORD(wparam as DWORD) as c_int; // user's scrolling request
            match request {
                SB_PAGEDOWN |
                SB_LINEDOWN => {
                    if request == SB_PAGEDOWN {
                        CTRL_COLOR[i] += 15;
                    }
                    CTRL_COLOR[i] = cmp::min(255, CTRL_COLOR[i] + 1);
                }

                SB_PAGEUP |
                SB_LINEUP => {
                    if request == SB_PAGEUP {
                        CTRL_COLOR[i] -= 15;
                    }
                    CTRL_COLOR[i] = cmp::max(0, CTRL_COLOR[i] - 1);
                }

                SB_TOP => {
                    CTRL_COLOR[i] = 0;
                }

                SB_BOTTOM => {
                    CTRL_COLOR[i] = 255;
                }

                SB_THUMBPOSITION |
                SB_THUMBTRACK => {
                    CTRL_COLOR[i] = HIWORD(wparam as DWORD) as BYTE;
                }
                _ => {}
            }

            let mut si = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_POS,
                nPos: CTRL_COLOR[i] as c_int,
                ..mem::uninitialized()
            };
            SetScrollInfo(HWND_SCROLL[i], SB_CTL, &si, TRUE);
            // SetScrollPos(HWND_SCROLL[i], SB_CTL, CTRL_COLOR[i], TRUE);

            let text = to_wstr(&format!("{}", CTRL_COLOR[i]));
            SetWindowTextW(HWND_VALUE[i], text.as_ptr());

            DeleteBrush(
                SetClassLongPtrW(hwnd,
                                 GCLP_HBRBACKGROUND,
                                 CreateSolidBrush(RGB(CTRL_COLOR[0],
                                                      CTRL_COLOR[1],
                                                      CTRL_COLOR[2])) as LONG_PTR)
                    as HBRUSH);
            InvalidateRect(hwnd, &RECT_COLOR, TRUE);

            0 as LRESULT
        }

        WM_CTLCOLORSCROLLBAR => {
            let i = GetWindowLongPtrW(lparam as HWND, GWLP_ID) as usize;
            return HBRUSH_ARRAY[i] as LRESULT;
        }

        WM_CTLCOLORSTATIC => {
            let i = GetWindowLongPtrW(lparam as HWND, GWLP_ID) as usize;

            if i >= 3 && i <= 8 {    // static text controls
                SetTextColor(wparam as HDC, PRIMARY_COLORS[i % 3]);
                SetBkColor(wparam as HDC, GetSysColor(COLOR_BTNHIGHLIGHT));
                return HBRUSH_STATIC as LRESULT;
            }
            0 as LRESULT  // message processed
        }

        WM_SYSCOLORCHANGE => {
            DeleteBrush(HBRUSH_STATIC);
            HBRUSH_STATIC = CreateSolidBrush(GetSysColor(COLOR_BTNHIGHLIGHT));
            0 as LRESULT  // message processed
        }

        WM_DESTROY => {
            DeleteBrush(SetClassLongPtrW(hwnd,
                                         GCLP_HBRBACKGROUND,
                                         GetStockBrush(WHITE_BRUSH) as LONG_PTR) as HBRUSH);

            for i in 0..NUM_CTRLS {
                DeleteBrush(HBRUSH_ARRAY[i]);
            }
            DeleteBrush(HBRUSH_STATIC);

            PostQuitMessage(0);
            0 as LRESULT  // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}


unsafe extern "system" fn scroll_proc(hwnd: HWND,
                                      message: UINT,
                                      wparam: WPARAM,
                                      lparam: LPARAM,
                                      _id_subclass: UINT_PTR,
                                      _ref_data: DWORD_PTR)
                                      -> LRESULT
{
    let id = GetWindowLongPtrW(hwnd, GWLP_ID) as c_int;

    match message {
        WM_KEYDOWN => {
            if wparam as c_int == VK_TAB {
                let val = if GetKeyState(VK_SHIFT) < 0 { 2 } else { 1 };
                SetFocus(GetDlgItem(GetParent(hwnd), (id + val) % NUM_CTRLS as c_int));
            }
        }

        WM_SETFOCUS => {
            ID_FOCUS = id as usize;
        }

        WM_NCDESTROY => {
            RemoveWindowSubclass(hwnd, Some(scroll_proc), 1);
        }

        _ => {}
    }
    DefSubclassProc(hwnd, message, wparam, lparam)
}
