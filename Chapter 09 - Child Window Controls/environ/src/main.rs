// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - Environ
//
// The original source code copyright:
//
// ENVIRON.C -- Environment List Box
//              (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
use std::slice;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HMENU, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processenv::{
    FreeEnvironmentStringsW, GetEnvironmentStringsW, GetEnvironmentVariableW,
};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDialogBaseUnits, GetMessageW,
    GetSystemMetrics, LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW,
    SetFocus, SetWindowTextW, ShowWindow, TranslateMessage, UpdateWindow, COLOR_WINDOW, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, LBN_SELCHANGE, LBS_STANDARD,
    MB_ICONERROR, MSG, SM_CXSCREEN, SM_CXVSCROLL, SS_LEFT, SW_SHOW, WM_COMMAND, WM_CREATE,
    WM_DESTROY, WM_SETFOCUS, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    to_wstr, GetWindowInstance, ListBox_AddString, ListBox_GetCurSel, ListBox_GetText,
    ListBox_GetTextLen,
};

const ID_LIST: u16 = 1;
const ID_TEXT: u16 = 2;

fn main() {
    let app_name = to_wstr("environ");

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
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
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

        let caption = to_wstr("Environment List Box");
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
    static mut HWND_LIST: HWND = null_mut();
    static mut HWND_TEXT: HWND = null_mut();

    match message {
        WM_CREATE => {
            let char_x = LOWORD(GetDialogBaseUnits() as DWORD) as c_int;
            let char_y = HIWORD(GetDialogBaseUnits() as DWORD) as c_int;

            // Create listbox and static text windows.

            let text_listbox = to_wstr("listbox");
            HWND_LIST = CreateWindowExW(
                0,
                text_listbox.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE | LBS_STANDARD,
                char_x,
                char_y * 3,
                char_x * 16 + GetSystemMetrics(SM_CXVSCROLL),
                char_y * 5,
                hwnd,
                ID_LIST as HMENU,
                GetWindowInstance(hwnd),
                null_mut(),
            );

            let text_static = to_wstr("static");
            HWND_TEXT = CreateWindowExW(
                0,
                text_static.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE | SS_LEFT,
                char_x,
                char_y,
                GetSystemMetrics(SM_CXSCREEN),
                char_y,
                hwnd,
                ID_TEXT as HMENU,
                GetWindowInstance(hwnd),
                null_mut(),
            );

            fill_listbox(HWND_LIST);

            0 as LRESULT // message processed
        }

        WM_SETFOCUS => {
            SetFocus(HWND_LIST);
            0 as LRESULT // message processed
        }

        WM_COMMAND => {
            if LOWORD(wparam as DWORD) == ID_LIST && HIWORD(wparam as DWORD) == LBN_SELCHANGE {
                // Get current selection.

                let var_name: Vec<u16> = {
                    let index = ListBox_GetCurSel(HWND_LIST);
                    let length = ListBox_GetTextLen(HWND_LIST, index) as usize;

                    let mut buffer: Vec<u16> = vec![0; length + 1];
                    ListBox_GetText(HWND_LIST, index, buffer.as_mut_ptr());
                    // let s = OsString::from_wide(&buffer[..length]).into_string().unwrap();
                    // println!("{}", s);
                    buffer
                };

                // Get environment string.

                let value: Vec<u16> = {
                    let length = GetEnvironmentVariableW(var_name.as_ptr(), null_mut(), 0) as usize;

                    let mut buffer: Vec<u16> = vec![0; length + 1];
                    GetEnvironmentVariableW(
                        var_name.as_ptr(),
                        buffer.as_mut_ptr(),
                        length as DWORD + 1,
                    );
                    // let s = OsString::from_wide(&buffer[..length]).into_string().unwrap();
                    // println!("{}", s);
                    buffer
                };

                // Show it in window.

                SetWindowTextW(HWND_TEXT, value.as_ptr());
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

unsafe fn fill_listbox(hwnd_list: HWND) {
    /* It it possible to work entirely with the GetEnvironmentStringsW()
       block using pointers or a slice and searching/splitting on
       0x00 and 0x3d.
    */
    // GetEnvironmentStringsW/FreeEnvironmentStringsW contained within
    // scope of braces.
    let s: String = {
        let p_var_block: *mut u16 = GetEnvironmentStringsW(); // Get pointer to environment block

        // Find the length of the the environment block.
        // Double \0\0 terminates block.
        // Single \0 is used a a delimiter within the block.

        let mut i: isize = 0;
        while *p_var_block.offset(i) != 0 || *p_var_block.offset(i + 1) != 0 {
            i += 1;
        }
        // Terminating \0 discarded. Splitting on intermediate \0 chars -
        // leaving the last one would cause a split there giving a zero length
        // string.
        let len = i as usize;

        // Now get a slice the Windows UTF-16 string and convert it to a Rust String.
        // (with \0 delimiters within the string)
        let slice = slice::from_raw_parts(p_var_block, len);
        let delimited: String = OsString::from_wide(&slice[..len]).into_string().unwrap();

        FreeEnvironmentStringsW(p_var_block);

        delimited // everything but 'delimited' goes nicely out of scope.
    };

    // s = "V1=foo\0V2=bar\0V3=foobar"
    // String 's' can now be split on \0.
    for line in s.split('\0').collect::<Vec<&str>>() {
        if line.starts_with('=') {
            continue; // Skip lines names beginning with '='
        };
        // just want value before the '='
        let var = line.split('=').take(1).next().unwrap();
        let var_w = to_wstr(var);
        ListBox_AddString(hwnd_list, var_w.as_ptr());
    }
}
