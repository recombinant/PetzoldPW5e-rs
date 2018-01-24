// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 5 - DevCaps1
//
// The original source code copyright:
//
// DEVCAPS1.C -- Device Capabilities Display Program No. 1
//               (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]

#![cfg(windows)]
extern crate winapi;
extern crate extras;

use std::mem;
use std::ptr::{null_mut, null};
use winapi::ctypes::{c_int};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, PostQuitMessage, RegisterClassExW,
                          ShowWindow, UpdateWindow, GetMessageW, TranslateMessage, DispatchMessageW,
                          BeginPaint, EndPaint, MessageBoxW, LoadIconW, LoadCursorW, GetDC,
                          ReleaseDC,
                          MSG, PAINTSTRUCT, WNDCLASSEXW,
                          WM_CREATE, WM_DESTROY, WM_PAINT,
                          WS_OVERLAPPEDWINDOW, SW_SHOW, CS_HREDRAW,
                          CS_VREDRAW, IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, CW_USEDEFAULT, };
use winapi::um::wingdi::{GetTextMetricsW, TextOutW, SetTextAlign, GetDeviceCaps,
                         TEXTMETRICW,
                         TA_LEFT, TA_RIGHT, TA_TOP,
                         HORZSIZE, VERTSIZE, HORZRES, VERTRES, BITSPIXEL, PLANES, NUMBRUSHES,
                         NUMPENS, NUMMARKERS, NUMFONTS, NUMCOLORS, PDEVICESIZE, ASPECTX, ASPECTY,
                         ASPECTXY, LOGPIXELSX, LOGPIXELSY, SIZEPALETTE, NUMRESERVED, COLORRES, };
use winapi::um::winbase::{lstrlenW};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE};
use winapi::shared::windef::{HWND};
use winapi::shared::ntdef::{LPCWSTR};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{WHITE_BRUSH, to_wstr, GetStockBrush};


struct DevCaps<'a> {
    index: c_int,
    label: &'a str,
    desc: &'a str,
}

//@formatter:off
const DEV_CAPS: &'static [DevCaps] = &[
    DevCaps { index: HORZSIZE,    label: "HORZSIZE",    desc: "Width in millimeters:" },
    DevCaps { index: VERTSIZE,    label: "VERTSIZE",    desc: "Height in millimeters:" },
    DevCaps { index: HORZRES,     label: "HORZRES",     desc: "Width in pixels:" },
    DevCaps { index: VERTRES,     label: "VERTRES",     desc: "Height in raster lines:" },
    DevCaps { index: BITSPIXEL,   label: "BITSPIXEL",   desc: "Color bits per pixel:" },
    DevCaps { index: PLANES,      label: "PLANES",      desc: "Number of color planes:" },
    DevCaps { index: NUMBRUSHES,  label: "NUMBRUSHES",  desc: "Number of device brushes:" },
    DevCaps { index: NUMPENS,     label: "NUMPENS",     desc: "Number of device pens:" },
    DevCaps { index: NUMMARKERS,  label: "NUMMARKERS",  desc: "Number of device markers:" },
    DevCaps { index: NUMFONTS,    label: "NUMFONTS",    desc: "Number of device fonts:" },
    DevCaps { index: NUMCOLORS,   label: "NUMCOLORS",   desc: "Number of device colors:" },
    DevCaps { index: PDEVICESIZE, label: "PDEVICESIZE", desc: "Size of device structure:" },
    DevCaps { index: ASPECTX,     label: "ASPECTX",     desc: "Relative width of pixel:" },
    DevCaps { index: ASPECTY,     label: "ASPECTY",     desc: "Relative height of pixel:" },
    DevCaps { index: ASPECTXY,    label: "ASPECTXY",    desc: "Relative diagonal of pixel:" },
    DevCaps { index: LOGPIXELSX,  label: "LOGPIXELSX",  desc: "Horizontal dots per inch:" },
    DevCaps { index: LOGPIXELSY,  label: "LOGPIXELSY",  desc: "Vertical dots per inch:" },
    DevCaps { index: SIZEPALETTE, label: "SIZEPALETTE", desc: "Number of palette entries:" },
    DevCaps { index: NUMRESERVED, label: "NUMRESERVED", desc: "Reserved palette entries:" },
    DevCaps { index: COLORRES,    label: "COLORRES",    desc: "Actual color resolution:" },
];
//@formatter:on


fn main() {
    let app_name = to_wstr("dev_caps1");
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

        let caption = to_wstr("Device Capabilities No. 1");
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
    static mut CAPS_WIDTH: c_int = 0;
    static mut CHAR_WIDTH: c_int = 0;
    static mut CHAR_HEIGHT: c_int = 0;

    match message {
        WM_CREATE => {
            let hdc = GetDC(hwnd);
            let mut tm: TEXTMETRICW = mem::uninitialized();

            GetTextMetricsW(hdc, &mut tm);
            CHAR_WIDTH = tm.tmAveCharWidth;
            CAPS_WIDTH = (if tm.tmPitchAndFamily & 1 == 1 { 3 } else { 2 }) * CHAR_WIDTH / 2;
            CHAR_HEIGHT = tm.tmHeight + tm.tmExternalLeading;

            ReleaseDC(hwnd, hdc);

            0 as LRESULT  // message processed
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::uninitialized();
            let hdc = BeginPaint(hwnd, &mut ps);

            for (u, dev_cap) in DEV_CAPS.iter().enumerate() {
                let i = u as c_int;

                SetTextAlign(hdc, TA_LEFT | TA_TOP);

                let label = to_wstr(dev_cap.label);
                TextOutW(hdc,
                         0,
                         CHAR_HEIGHT * i,
                         label.as_ptr(),
                         lstrlenW(label.as_ptr()));

                let desc = to_wstr(dev_cap.desc);
                TextOutW(hdc,
                         14 * CAPS_WIDTH,
                         CHAR_HEIGHT * i,
                         desc.as_ptr(),
                         lstrlenW(desc.as_ptr()));

                SetTextAlign(hdc, TA_RIGHT | TA_TOP);

                let cap = to_wstr(&format!("{:5}", GetDeviceCaps(hdc, dev_cap.index)));
                TextOutW(hdc,
                         14 * CAPS_WIDTH + 35 * CHAR_WIDTH,
                         CHAR_HEIGHT * i,
                         cap.as_ptr(),
                         lstrlenW(cap.as_ptr()));
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
