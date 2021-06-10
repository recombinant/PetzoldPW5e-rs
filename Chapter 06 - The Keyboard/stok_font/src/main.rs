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
extern crate extras;
extern crate winapi;

use std::cmp;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{
    GetTextFaceW, GetTextMetricsW, LineTo, MoveToEx, SetTextAlign, TextOutW, LF_FACESIZE,
    TA_CENTER, TA_TOP, TEXTMETRICW,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetMessageW,
    GetScrollInfo, InvalidateRect, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage,
    RegisterClassExW, SendMessageW, SetScrollInfo, ShowWindow, TranslateMessage, UpdateWindow,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG,
    PAINTSTRUCT, SCROLLINFO, SIF_ALL, SIF_POS, SIF_RANGE, SW_SHOW, VK_DOWN, VK_END, VK_HOME,
    VK_LEFT, VK_NEXT, VK_PRIOR, VK_RIGHT, VK_UP, WM_CREATE, WM_DESTROY, WM_DISPLAYCHANGE,
    WM_KEYDOWN, WM_PAINT, WM_VSCROLL, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VSCROLL,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetStockBrush, GetStockFont, SelectFont, ANSI_FIXED_FONT, ANSI_VAR_FONT,
    DEFAULT_GUI_FONT, DEVICE_DEFAULT_FONT, GET_WM_VSCROLL_CODE, OEM_FIXED_FONT, SB_BOTTOM,
    SB_LINEDOWN, SB_LINEUP, SB_PAGEDOWN, SB_PAGEUP, SB_THUMBPOSITION, SB_TOP, SB_VERT,
    SYSTEM_FIXED_FONT, SYSTEM_FONT, WHITE_BRUSH,
};

fn main() {
    let app_name = to_wstr("stokfont");

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

        let caption = to_wstr("Stock Fonts");
        let hwnd = CreateWindowExW(
            0,                                // dwExStyle:
            atom as LPCWSTR,                  // lpClassName: class name or atom
            caption.as_ptr(),                 // lpWindowName: window caption
            WS_OVERLAPPEDWINDOW | WS_VSCROLL, // dwStyle: window style
            CW_USEDEFAULT,                    // x: initial x position
            CW_USEDEFAULT,                    // y: initial y position
            CW_USEDEFAULT,                    // nWidth: initial x size
            CW_USEDEFAULT,                    // nHeight: initial y size
            null_mut(),                       // hWndParent: parent window handle
            null_mut(),                       // hMenu: window menu handle
            hinstance,                        // hInstance: program instance handle
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
    struct StockFont<'a> {
        index: c_int,
        face_name: &'a str,
    }

    const STOCK_FONTS: &[StockFont] = &[
        StockFont {
            index: OEM_FIXED_FONT,
            face_name: "OEM_FIXED_FONT",
        },
        StockFont {
            index: ANSI_FIXED_FONT,
            face_name: "ANSI_FIXED_FONT",
        },
        StockFont {
            index: ANSI_VAR_FONT,
            face_name: "ANSI_VAR_FONT",
        },
        StockFont {
            index: SYSTEM_FONT,
            face_name: "SYSTEM_FONT",
        },
        StockFont {
            index: DEVICE_DEFAULT_FONT,
            face_name: "DEVICE_DEFAULT_FONT",
        },
        StockFont {
            index: SYSTEM_FIXED_FONT,
            face_name: "SYSTEM_FIXED_FONT",
        },
        StockFont {
            index: DEFAULT_GUI_FONT,
            face_name: "DEFAULT_GUI_FONT",
        },
    ];
    static mut STOCK_FONT_IDX: usize = 0;
    let stock_font_count = STOCK_FONTS.len();

    match message {
        WM_CREATE => {
            let si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_RANGE,
                nMin: 0,
                nMax: stock_font_count as c_int - 1,
                ..mem::zeroed()
            };
            SetScrollInfo(hwnd, SB_VERT, &si, TRUE);
            0 // message processed
        }

        WM_DISPLAYCHANGE => {
            InvalidateRect(hwnd, null(), TRUE);
            0 // message processed
        }

        WM_VSCROLL => {
            let mut si: SCROLLINFO = SCROLLINFO {
                cbSize: mem::size_of::<SCROLLINFO>() as UINT,
                fMask: SIF_ALL,
                ..mem::MaybeUninit::uninit().assume_init()
            };
            GetScrollInfo(hwnd, SB_VERT, &mut si);

            match GET_WM_VSCROLL_CODE(wparam, lparam) {
                SB_TOP => {
                    si.nPos = si.nMin;
                }
                SB_BOTTOM => {
                    si.nPos = si.nMax;
                }
                SB_LINEUP | SB_PAGEUP => {
                    si.nPos -= 1;
                }
                SB_LINEDOWN | SB_PAGEDOWN => {
                    si.nPos += 1;
                }
                SB_THUMBPOSITION => {
                    si.nPos = si.nTrackPos;
                }
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
            0 // message processed
        }

        WM_KEYDOWN => {
            match wparam as c_int {
                VK_HOME => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_TOP as WPARAM, 0);
                }
                VK_END => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_BOTTOM as WPARAM, 0);
                }
                VK_PRIOR | VK_LEFT | VK_UP => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_LINEUP as WPARAM, 0);
                }
                VK_NEXT | VK_RIGHT | VK_DOWN => {
                    SendMessageW(hwnd, WM_VSCROLL, SB_PAGEDOWN as WPARAM, 0);
                }
                _ => {}
            }

            0 // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            SelectFont(hdc, GetStockFont(STOCK_FONTS[STOCK_FONT_IDX].index));

            let mut face_name_buffer: Vec<u16> = vec![0; LF_FACESIZE];
            let size = GetTextFaceW(hdc, LF_FACESIZE as c_int, face_name_buffer.as_mut_ptr());
            let face_name = if size > 0 {
                OsString::from_wide(&face_name_buffer[..size as usize])
                    .into_string()
                    .unwrap()
            } else {
                String::from("")
            };

            let mut tm: TEXTMETRICW = mem::MaybeUninit::uninit().assume_init();
            GetTextMetricsW(hdc, &mut tm);
            let x_grid: c_int = cmp::max(3 * tm.tmAveCharWidth, 2 * tm.tmMaxCharWidth);
            let y_grid: c_int = tm.tmHeight + 3;

            let buffer = to_wstr(&format!(
                " {}: Face Name = {}, CharSet = {}",
                STOCK_FONTS[STOCK_FONT_IDX].face_name, face_name, tm.tmCharSet
            ));
            TextOutW(hdc, 0, 0, buffer.as_ptr(), lstrlenW(buffer.as_ptr()));

            SetTextAlign(hdc, TA_TOP | TA_CENTER);

            // vertical and horizontal lines

            for i in 0..17 {
                MoveToEx(hdc, (i + 2) * x_grid, 2 * y_grid, null_mut());
                LineTo(hdc, (i + 2) * x_grid, 19 * y_grid);

                MoveToEx(hdc, x_grid, (i + 3) * y_grid, null_mut());
                LineTo(hdc, 18 * x_grid, (i + 3) * y_grid);
            }

            // vertical and horizontal headings

            for i in 0..16 {
                let buffer1 = to_wstr(&format!("{:X}-", i));
                TextOutW(
                    hdc,
                    (2 * i + 5) * x_grid / 2,
                    2 * y_grid + 2,
                    buffer1.as_ptr(),
                    lstrlenW(buffer1.as_ptr()),
                );

                let buffer2 = to_wstr(&format!("-{:X}", i));
                TextOutW(
                    hdc,
                    3 * x_grid / 2,
                    (i + 3) * y_grid + 2,
                    buffer2.as_ptr(),
                    lstrlenW(buffer2.as_ptr()),
                );
            }

            // characters

            for y in 0..16 {
                for x in 0..16 {
                    let c: u16 = (16 * x + y) as u16;
                    TextOutW(hdc, (2 * x + 5) * x_grid / 2, (y + 3) * y_grid + 2, &c, 1);
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
