// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 8 - Clock
//
// The original source code copyright:
//
// CLOCK.C −− Analog Clock Program
//            (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::f64::consts::PI;
use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::{LONG, LPCWSTR};
use winapi::shared::windef::{HDC, HWND, POINT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::sysinfoapi::GetLocalTime;
use winapi::um::wingdi::{
    Ellipse, Polyline, SetMapMode, SetViewportExtEx, SetViewportOrgEx, SetWindowExtEx,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetDC, GetMessageW,
    KillTimer, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, ReleaseDC,
    SetTimer, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    IDC_ARROW, IDI_APPLICATION, MB_ICONERROR, MSG, PAINTSTRUCT, SW_SHOW, WM_CREATE, WM_DESTROY,
    WM_PAINT, WM_SIZE, WM_TIMER, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetStockBrush, GetStockPen, SelectBrush, SelectPen, BLACK_BRUSH, BLACK_PEN,
    MM_ISOTROPIC, WHITE_BRUSH, WHITE_PEN,
};

const ID_TIMER: usize = 1;

fn main() {
    let app_name = to_wstr("clock");

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

        let caption = to_wstr("Analog Clock");
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
    static mut ST_PREVIOUS: SYSTEMTIME = SYSTEMTIME {
        wYear: 0,
        wMonth: 0,
        wDayOfWeek: 0,
        wDay: 0,
        wHour: 0,
        wMinute: 0,
        wSecond: 0,
        wMilliseconds: 0,
    };

    match message {
        WM_CREATE => {
            SetTimer(hwnd, ID_TIMER, 1000, None);

            let mut st: SYSTEMTIME = mem::MaybeUninit::uninit().assume_init();
            GetLocalTime(&mut st);
            ST_PREVIOUS = st;
            0 // message processed
        }

        WM_SIZE => {
            CLIENT_WIDTH = GET_X_LPARAM(lparam);
            CLIENT_HEIGHT = GET_Y_LPARAM(lparam);
            0 // message processed
        }

        WM_TIMER => {
            let mut st: SYSTEMTIME = mem::MaybeUninit::uninit().assume_init();
            GetLocalTime(&mut st);

            let changed = st.wHour != ST_PREVIOUS.wHour || st.wMinute != ST_PREVIOUS.wMinute;

            let hdc: HDC = GetDC(hwnd);

            set_isotropic(hdc, CLIENT_WIDTH, CLIENT_HEIGHT);

            SelectPen(hdc, GetStockPen(WHITE_PEN));
            draw_hands(hdc, &ST_PREVIOUS, changed);

            SelectPen(hdc, GetStockPen(BLACK_PEN));
            draw_hands(hdc, &st, true);

            ReleaseDC(hwnd, hdc);

            ST_PREVIOUS = st;
            0 // message processed
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            set_isotropic(hdc, CLIENT_WIDTH, CLIENT_HEIGHT);
            draw_clock(hdc);
            draw_hands(hdc, &ST_PREVIOUS, true);

            EndPaint(hwnd, &ps);
            0 // message processed
        }

        WM_DESTROY => {
            KillTimer(hwnd, ID_TIMER);
            PostQuitMessage(0);
            0 // message processed
        }

        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe fn set_isotropic(hdc: HDC, client_width: c_int, client_height: c_int) {
    SetMapMode(hdc, MM_ISOTROPIC);
    SetWindowExtEx(hdc, 1000, 1000, null_mut());
    SetViewportExtEx(hdc, client_width / 2, -client_height / 2, null_mut());
    SetViewportOrgEx(hdc, client_width / 2, client_height / 2, null_mut());
}

unsafe fn draw_clock(hdc: HDC) {
    let mut pt: [POINT; 3] = mem::MaybeUninit::uninit().assume_init();

    for angle in (0..360).step_by(6) {
        pt[0].x = 0;
        pt[0].y = 900;

        rotate_point(&mut pt[..1], angle as f64);

        let tmp = if (angle % 5) != 0 { 33 } else { 100 };
        pt[2].x = tmp;
        pt[2].y = tmp;

        pt[0].x -= pt[2].x / 2;
        pt[0].y -= pt[2].y / 2;

        pt[1].x = pt[0].x + pt[2].x;
        pt[1].y = pt[0].y + pt[2].y;

        SelectBrush(hdc, GetStockBrush(BLACK_BRUSH));

        Ellipse(hdc, pt[0].x, pt[0].y, pt[1].x, pt[1].y);
    }
}

unsafe fn draw_hands(hdc: HDC, pst: &SYSTEMTIME, change: bool) {
    const PT: [[POINT; 5]; 3] = [
        [
            POINT { x: 0, y: -150 },
            POINT { x: 100, y: 0 },
            POINT { x: 0, y: 600 },
            POINT { x: -100, y: 0 },
            POINT { x: 0, y: -150 },
        ],
        [
            POINT { x: 0, y: -200 },
            POINT { x: 50, y: 0 },
            POINT { x: 0, y: 800 },
            POINT { x: -50, y: 0 },
            POINT { x: 0, y: -200 },
        ],
        [
            POINT { x: 0, y: 0 },
            POINT { x: 0, y: 0 },
            POINT { x: 0, y: 0 },
            POINT { x: 0, y: 0 },
            POINT { x: 0, y: 800 },
        ],
    ];

    let angle: [c_int; 3] = [
        (pst.wHour as c_int * 30) % 360 + pst.wMinute as c_int / 2,
        pst.wMinute as c_int * 6,
        pst.wSecond as c_int * 6,
    ];

    let mut temp_pt: [[POINT; 5]; 3] = PT;

    for i in if change { 0 } else { 2 }..3 {
        rotate_point(&mut temp_pt[i], angle[i] as f64);
        Polyline(hdc, &temp_pt[i][0], 5);
    }
}

unsafe fn rotate_point(pt_array: &mut [POINT], angle: f64) {
    let c = (2.0 * PI * angle / 360.0).cos();
    let s = (2.0 * PI * angle / 360.0).sin();
    for pt in pt_array.iter_mut() {
        let x = pt.x as f64;
        let y = pt.y as f64;
        pt.x = (x * c + y * s) as LONG;
        pt.y = (y * c - x * s) as LONG;
    }
}
