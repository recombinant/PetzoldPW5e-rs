// Transliterated from Charles Petzold's Programming Windows 5e
// http://www.charlespetzold.com/pw5/index.html
//
// Chapter 9 - Environ
//
// The original source code copyright:
//
// HEAD.C -- Displays beginning (head) of file
//           (c) Charles Petzold, 1998
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate extras;
extern crate winapi;

use std::mem;
use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::basetsd::{DWORD_PTR, UINT_PTR};
use winapi::shared::minwindef::{
    DWORD, FALSE, HIWORD, LOWORD, LPARAM, LPVOID, LRESULT, MAKELONG, MAX_PATH, TRUE, UINT, WPARAM,
};
use winapi::shared::ntdef::{CHAR, LPCWSTR};
use winapi::shared::windef::{HBRUSH, HMENU, HWND, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::commctrl::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
use winapi::um::fileapi::{CreateFileW, ReadFile, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processenv::{GetCurrentDirectoryW, SetCurrentDirectoryW};
use winapi::um::winbase::lstrlenW;
use winapi::um::wingdi::{SetBkColor, SetTextColor};
use winapi::um::winnt::{FILE_SHARE_READ, GENERIC_READ};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawTextA, EndPaint,
    GetDialogBaseUnits, GetMessageW, GetParent, GetSysColor, GetSystemMetrics, InvalidateRect,
    LoadCursorW, LoadIconW, MessageBoxW, PostQuitMessage, RegisterClassExW, SendMessageW, SetFocus,
    SetWindowTextW, ShowWindow, TranslateMessage, UpdateWindow, COLOR_BTNFACE, COLOR_BTNTEXT,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, DT_EXPANDTABS, DT_NOCLIP, DT_NOPREFIX, DT_WORDBREAK,
    IDC_ARROW, IDI_APPLICATION, LBN_DBLCLK, LBS_STANDARD, MB_ICONERROR, MSG, PAINTSTRUCT,
    SM_CXVSCROLL, SS_LEFT, SW_SHOW, VK_RETURN, WM_COMMAND, WM_CREATE, WM_DESTROY, WM_KEYDOWN,
    WM_NCDESTROY, WM_PAINT, WM_SETFOCUS, WM_SIZE, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
};

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
use extras::{
    lstrcatW,
    lstrcpyW, // TODO: remove these when fixed in winbase
    to_wstr,
    GetStockFont,
    GetWindowInstance,
    ListBox_Dir,
    ListBox_GetCurSel,
    ListBox_GetText,
    ListBox_ResetContent,
    SelectFont,
    DDL_ARCHIVE,
    DDL_DIRECTORY,
    DDL_DRIVES,
    DDL_HIDDEN,
    DDL_READONLY,
    DDL_READWRITE,
    DDL_SYSTEM,
    LB_ERR,
    SYSTEM_FIXED_FONT,
};

const ID_LIST: u16 = 1;
const ID_TEXT: u16 = 2;

const ID_LISTPROC: usize = 1; // SetWindowSubclass/RemoveWindowSubclass

const MAXREAD: usize = 8192;
const DIRATTR: UINT = DDL_READWRITE
    | DDL_READONLY
    | DDL_HIDDEN
    | DDL_SYSTEM
    | DDL_DIRECTORY
    | DDL_ARCHIVE
    | DDL_DRIVES;
const DTFLAGS: UINT = DT_WORDBREAK | DT_EXPANDTABS | DT_NOCLIP | DT_NOPREFIX;

fn main() {
    let app_name = to_wstr("head");

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
            hbrBackground: (COLOR_BTNFACE + 1) as HBRUSH,
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

        let caption = to_wstr("Head");
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
    static mut VALID_FILE: bool = false;
    static mut RECT: RECT = RECT {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    static mut HWND_LIST: HWND = null_mut();
    static mut HWND_TEXT: HWND = null_mut();
    static mut FILE_NAME: [u16; MAX_PATH + 1] = [0; MAX_PATH + 1];
    static mut WSTR_BUFFER: [u16; MAX_PATH + 1] = [0; MAX_PATH + 1];

    match message {
        WM_CREATE => {
            let char_x = LOWORD(GetDialogBaseUnits() as DWORD) as c_int;
            let char_y = HIWORD(GetDialogBaseUnits() as DWORD) as c_int;

            RECT.left = 20 * char_x;
            RECT.top = 3 * char_y;

            // Create listbox and static text windows.

            let text_listbox = to_wstr("listbox");
            HWND_LIST = CreateWindowExW(
                0,
                text_listbox.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE | LBS_STANDARD,
                char_x,
                char_y * 3,
                char_x * 13 + GetSystemMetrics(SM_CXVSCROLL),
                char_y * 10,
                hwnd,
                ID_LIST as HMENU,
                GetWindowInstance(hwnd),
                null_mut(),
            );

            GetCurrentDirectoryW(MAX_PATH as DWORD + 1, WSTR_BUFFER.as_mut_ptr());

            let text_static = to_wstr("static");
            HWND_TEXT = CreateWindowExW(
                0,
                text_static.as_ptr(),
                WSTR_BUFFER.as_ptr(),
                WS_CHILD | WS_VISIBLE | SS_LEFT,
                char_x,
                char_y,
                char_x * MAX_PATH as c_int,
                char_y,
                hwnd,
                ID_TEXT as HMENU,
                GetWindowInstance(hwnd),
                null_mut(),
            );

            SetWindowSubclass(HWND_LIST, Some(list_proc), ID_LISTPROC, 0);

            let all = to_wstr("*.*");
            ListBox_Dir(HWND_LIST, DIRATTR, all.as_ptr());

            0 // message processed
        }

        WM_SIZE => {
            RECT.right = GET_X_LPARAM(lparam);
            RECT.bottom = GET_Y_LPARAM(lparam);
            0 // message processed
        }

        WM_SETFOCUS => {
            SetFocus(HWND_LIST);
            0 // message processed
        }

        WM_COMMAND => {
            // Here lies evil. lstrcpyW and lstrcatW are used for pedagogical
            // purposes. There is potential for disaster as these two functions
            // are notorious for buffer overruns.

            if LOWORD(wparam as DWORD) == ID_LIST && HIWORD(wparam as DWORD) == LBN_DBLCLK {
                let i = ListBox_GetCurSel(HWND_LIST);
                if LB_ERR == i {
                    return 0; // message processed
                }

                ListBox_GetText(HWND_LIST, i, WSTR_BUFFER.as_mut_ptr());

                let hfile = CreateFileW(
                    WSTR_BUFFER.as_ptr(),
                    GENERIC_READ,
                    FILE_SHARE_READ,
                    null_mut(),
                    OPEN_EXISTING,
                    0,
                    null_mut(),
                );

                if INVALID_HANDLE_VALUE != hfile {
                    CloseHandle(hfile);
                    VALID_FILE = true;

                    //                    for (dst, src) in FILE_NAME.iter_mut().zip(WSTR_BUFFER)

                    // equivalent of lstrlenW
                    let len = WSTR_BUFFER.iter().take_while(|&c| *c != 0).count();
                    //                    FILE_NAME[..len+1].copy_from_slice(WSTR_BUFFER)
                    println!("{} {}", len, lstrlenW(WSTR_BUFFER.as_ptr()));

                    lstrcpyW(FILE_NAME.as_mut_ptr(), WSTR_BUFFER.as_ptr());
                    GetCurrentDirectoryW(MAX_PATH as DWORD + 1, WSTR_BUFFER.as_mut_ptr());

                    // Append backslash character to null terminated string if not present.
                    let len = lstrlenW(WSTR_BUFFER.as_ptr()) as usize;
                    if WSTR_BUFFER[len - 1] != 0x5c {
                        WSTR_BUFFER[len] = 0x5c; // backslash
                        WSTR_BUFFER[len + 1] = 0x00;
                    }
                    SetWindowTextW(
                        HWND_TEXT,
                        lstrcatW(WSTR_BUFFER.as_mut_ptr(), FILE_NAME.as_ptr()),
                    );
                } else {
                    VALID_FILE = false;

                    // Directories are displayed with square brackets in
                    // the listbox. The shenanigans in this scope involves
                    // the removal of said square brackets.

                    WSTR_BUFFER[lstrlenW(WSTR_BUFFER.as_ptr()) as usize - 1] = 0x00;

                    // If setting the directory doesn't work, maybe it's
                    // a drive change, so try that.

                    if SetCurrentDirectoryW(WSTR_BUFFER[1..].as_ptr()) == FALSE {
                        WSTR_BUFFER[3] = 0x3a;
                        WSTR_BUFFER[4] = 0x00;
                        SetCurrentDirectoryW(WSTR_BUFFER[2..].as_ptr());
                    }

                    // Get the new directory name and fill the list box.

                    GetCurrentDirectoryW(MAX_PATH as DWORD + 1, WSTR_BUFFER.as_mut_ptr());
                    SetWindowTextW(HWND_TEXT, WSTR_BUFFER.as_ptr());
                    ListBox_ResetContent(HWND_LIST);
                    let all = to_wstr("*.*");
                    ListBox_Dir(HWND_LIST, DIRATTR, all.as_ptr());
                }

                InvalidateRect(hwnd, null(), TRUE);
            }

            0 // message processed
        }

        WM_PAINT => {
            if !VALID_FILE {
                return DefWindowProcW(hwnd, message, wparam, lparam);
            } else {
                let hfile = CreateFileW(
                    FILE_NAME.as_ptr(),
                    GENERIC_READ,
                    FILE_SHARE_READ,
                    null_mut(),
                    OPEN_EXISTING,
                    0,
                    null_mut(),
                );
                if INVALID_HANDLE_VALUE == hfile {
                    VALID_FILE = false;
                    return DefWindowProcW(hwnd, message, wparam, lparam);
                } else {
                    static mut BUFFER: [CHAR; MAXREAD] = [0; MAXREAD];
                    let mut i: DWORD = 0;
                    ReadFile(
                        hfile,
                        BUFFER.as_mut_ptr() as LPVOID,
                        MAXREAD as DWORD,
                        &mut i,
                        null_mut(),
                    );
                    CloseHandle(hfile);

                    // i now equals the number of bytes in buffer.
                    // Commence getting a device context for displaying text.

                    let mut ps: PAINTSTRUCT = mem::MaybeUninit::uninit().assume_init();
                    let hdc = BeginPaint(hwnd, &mut ps);

                    SelectFont(hdc, GetStockFont(SYSTEM_FIXED_FONT));
                    SetTextColor(hdc, GetSysColor(COLOR_BTNTEXT));
                    SetBkColor(hdc, GetSysColor(COLOR_BTNFACE));

                    // Assume the file is ASCII

                    DrawTextA(hdc, BUFFER.as_ptr(), i as c_int, &mut RECT, DTFLAGS);

                    EndPaint(hwnd, &ps);
                }
            }
            0 // message processed
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            0 // message processed
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

unsafe extern "system" fn list_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    _id_subclass: UINT_PTR,
    _ref_data: DWORD_PTR,
) -> LRESULT {
    match message {
        WM_KEYDOWN => {
            if wparam == VK_RETURN as WPARAM {
                SendMessageW(
                    GetParent(hwnd),
                    WM_COMMAND,
                    MAKELONG(ID_LIST, LBN_DBLCLK) as WPARAM,
                    hwnd as LPARAM,
                );
            }
        }

        WM_NCDESTROY => {
            RemoveWindowSubclass(hwnd, Some(list_proc), ID_LISTPROC);
        }

        _ => {}
    }
    DefSubclassProc(hwnd, message, wparam, lparam)
}
