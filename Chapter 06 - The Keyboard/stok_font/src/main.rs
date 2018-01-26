// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 6 - StokFont
//
// The original source code copyright:
//
// STOKFONT.C -- Stock Font Objects
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::cmp;
use std::ptr::{null_mut, null};
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, SendMessageW, TranslateMessage,
                          DispatchMessageW, BeginPaint, EndPaint, MessageBoxW, LoadIconW,
                          LoadCursorW, InvalidateRect, GetScrollInfo, SetScrollInfo,
                          VK_HOME, VK_END, VK_NEXT, VK_PRIOR, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT,
                          MSG, PAINTSTRUCT, WNDCLASSEXW, SCROLLINFO,
                          WM_CREATE, WM_DESTROY, WM_PAINT, WM_DISPLAYCHANGE, WM_KEYDOWN, WM_VSCROLL,
                          WS_OVERLAPPEDWINDOW, WS_VSCROLL, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT,
                          SIF_RANGE, SIF_POS, SIF_ALL,};
use winapi::um::wingdi::{GetTextMetricsW, TextOutW, GetTextFaceW,
                         SetTextAlign, MoveToEx, LineTo,
                         TEXTMETRICW, TA_TOP, TA_CENTER, LF_FACESIZE};
use winapi::um::winbase::lstrlenW;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE,
                                TRUE};
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::LPCWSTR;

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, SB_VERT,
             GET_WM_VSCROLL_CODE,
             to_wstr, GetStockBrush, SelectFont, GetStockFont,
             OEM_FIXED_FONT, ANSI_FIXED_FONT, ANSI_VAR_FONT, SYSTEM_FONT, DEVICE_DEFAULT_FONT,
             SYSTEM_FIXED_FONT, DEFAULT_GUI_FONT,
             SB_TOP, SB_BOTTOM, SB_LINEUP, SB_PAGEUP, SB_LINEDOWN, SB_PAGEDOWN, SB_THUMBPOSITION, };


fn main() {
    let app_name = to_wstr("stokfont");
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
                        to_wstr("This program requires Windows NT!").as_ptr(),
                        app_name.as_ptr(),
                        MB_ICONERROR);
            return; //   premature exit
        }

        let caption = to_wstr("Stock Fonts");
        let hwnd = CreateWindowExW(
            0,                 // dwExStyle:
            atom as LPCWSTR,   // lpClassName: class name or atom
            caption.as_ptr(),  // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW | WS_VSCROLL,  // dwStyle: window style
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
    struct StockFont<'a> {
        index: c_int,
        face_name: &'a str,
    }

    const STOCK_FONTS: &'static [StockFont] = &[
        StockFont { index: OEM_FIXED_FONT, face_name: "OEM_FIXED_FONT" },
        StockFont { index: ANSI_FIXED_FONT, face_name: "ANSI_FIXED_FONT" },
        StockFont { index: ANSI_VAR_FONT, face_name: "ANSI_VAR_FONT" },
        StockFont { index: SYSTEM_FONT, face_name: "SYSTEM_FONT" },
        StockFont { index: DEVICE_DEFAULT_FONT, face_name: "DEVICE_DEFAULT_FONT" },
        StockFont { index: SYSTEM_FIXED_FONT, face_name: "SYSTEM_FIXED_FONT" },
        StockFont { index: DEFAULT_GUI_FONT, face_name: "DEFAULT_GUI_FONT" },
    ];
    static mut STOCK_FONT_IDX: usize = 0;
    let stock_font_count = STOCK_FONTS.len();

    match message {
        WM_CREATE => {
            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_RANGE,
                nMin: 0,
                nMax: stock_font_count as c_int - 1,
                ..mem::zeroed()
            };
            SetScrollInfo(hwnd, SB_VERT, &si, TRUE);
            0 as LRESULT  // message processed
        }

        WM_DISPLAYCHANGE => {
            InvalidateRect(hwnd, null(), TRUE);
            0 as LRESULT
        }

        WM_VSCROLL => {
            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_ALL,
                ..mem::uninitialized()
            };
            GetScrollInfo(hwnd, SB_VERT, &mut si);

            match GET_WM_VSCROLL_CODE(wparam, lparam) {
                SB_TOP => { si.nPos = si.nMin; }
                SB_BOTTOM => { si.nPos = si.nMax; }
                SB_LINEUP |
                SB_PAGEUP => { si.nPos -= 1; }
                SB_LINEDOWN |
                SB_PAGEDOWN => { si.nPos += 1; }
                SB_THUMBPOSITION => { si.nPos = si.nTrackPos; }
                _ => {}
            }

            // Because messages are sent from WM_KEYDOWN which has
            // no knowledge of the current postion.

            si.nPos = cmp::max(si.nMin, cmp::min(si.nPos, si.nMax));

            // Set the position and then retrieve it.  Due to adjustments
            // by Windows it may not be the same as the value set.

            si.fMask = SIF_POS;
            SetScrollInfo(hwnd, SB_VERT, &si, TRUE);
            GetScrollInfo(hwnd, SB_VERT, &mut si);
            STOCK_FONT_IDX = si.nPos as usize;

            InvalidateRect(hwnd, null(), TRUE);
            0 as LRESULT
        }

        WM_KEYDOWN => {
            match wparam as c_int {
                VK_HOME => { SendMessageW(hwnd, WM_VSCROLL, SB_TOP as WPARAM, 0); }
                VK_END => { SendMessageW(hwnd, WM_VSCROLL, SB_BOTTOM as WPARAM, 0); }
                VK_PRIOR |
                VK_LEFT |
                VK_UP => { SendMessageW(hwnd, WM_VSCROLL, SB_LINEUP as WPARAM, 0); }
                VK_NEXT |
                VK_RIGHT |
                VK_DOWN => { SendMessageW(hwnd, WM_VSCROLL, SB_PAGEDOWN as WPARAM, 0); }
                _ => {}
            }

            0 as LRESULT  // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(hdc, GetStockFont(STOCK_FONTS[STOCK_FONT_IDX].index));

            let mut face_name_buffer: Vec<u16> = vec![0; LF_FACESIZE];
            let size = GetTextFaceW(hdc, LF_FACESIZE as c_int, face_name_buffer.as_mut_ptr());
            let face_name = if size > 0 {
                String::from_utf16(&face_name_buffer[..size as usize]).unwrap()
            } else {
                String::from("")
            };

            let mut tm: TEXTMETRICW = mem::uninitialized();
            GetTextMetricsW(hdc, &mut tm);
            let x_grid: c_int = cmp::max(3 * tm.tmAveCharWidth, 2 * tm.tmMaxCharWidth);
            let y_grid: c_int = tm.tmHeight + 3;

            let buffer = to_wstr(&format!(" {}: Face Name = {}, CharSet = {}",
                                          STOCK_FONTS[STOCK_FONT_IDX].face_name,
                                          face_name, tm.tmCharSet));
            TextOutW(hdc, 0, 0, buffer.as_ptr(), lstrlenW(buffer.as_ptr()));

            SetTextAlign(hdc, TA_TOP | TA_CENTER);

            // vertical and horizontal lines

            for i in 0 as c_int..17 {
                MoveToEx(hdc, (i + 2) * x_grid, 2 * y_grid, null_mut());
                LineTo(hdc, (i + 2) * x_grid, 19 * y_grid);

                MoveToEx(hdc, x_grid, (i + 3) * y_grid, null_mut());
                LineTo(hdc, 18 * x_grid, (i + 3) * y_grid);
            }

            // vertical and horizontal headings

            for i in 0 as c_int..16 {
                let buffer1 = to_wstr(&format!("{:X}-", i));
                TextOutW(hdc, (2 * i + 5) * x_grid / 2, 2 * y_grid + 2,
                         buffer1.as_ptr(), lstrlenW(buffer1.as_ptr()));

                let buffer2 = to_wstr(&format!("-{:X}", i));
                TextOutW(hdc, 3 * x_grid / 2, (i + 3) * y_grid + 2,
                         buffer2.as_ptr(), lstrlenW(buffer2.as_ptr()));
            }

            // characters

            for y in 0 as c_int..16 {
                for x in 0 as c_int..16 {
                    let c: u16 = (16 * x + y) as u16;
                    TextOutW(hdc, (2 * x + 5) * x_grid / 2,
                             (y + 3) * y_grid + 2, &c, 1);
                }
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
