//
#![cfg(windows)]
extern crate winapi;


use std::ptr::{null_mut, null};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LOWORD, HIWORD, MAKELONG, WORD, DWORD, WPARAM, LPARAM, BOOL, HRGN,
                                HMODULE, LRESULT, };
use winapi::shared::windef::{HWND, HDC, HGDIOBJ, HPEN, HBRUSH, HFONT, HPALETTE, HBITMAP, RECT,
                             LPRECT, };
use winapi::shared::ntdef::{LPCWSTR, LPWSTR, };
use winapi::um::winuser::{SendMessageW, EnableWindow, InflateRect, GetWindowLongPtrW, };
use winapi::um::wingdi::{GetStockObject, SelectObject, DeleteObject, CombineRgn,
                         RGN_AND, RGN_COPY, RGN_DIFF, RGN_OR, RGN_XOR, };
use winapi::um::winnls::LCTYPE;
use winapi::shared::minwindef::UINT;
use winapi::shared::basetsd::ULONG_PTR;

use winapi::um::winuser::{LB_ADDSTRING, LB_DELETESTRING, LB_DIR, LB_FINDSTRING, LB_FINDSTRINGEXACT,
                          LB_GETCARETINDEX, LB_GETCOUNT, LB_GETCURSEL, LB_GETHORIZONTALEXTENT,
                          LB_GETITEMDATA, LB_GETITEMHEIGHT, LB_GETITEMRECT, LB_GETSEL,
                          LB_GETSELCOUNT, LB_GETSELITEMS, LB_GETTEXT, LB_GETTEXTLEN, LB_GETTOPINDEX,
                          LB_INSERTSTRING, LB_RESETCONTENT, LB_SELECTSTRING, LB_SELITEMRANGE,
                          LB_SETCARETINDEX, LB_SETCOLUMNWIDTH, LB_SETCURSEL, LB_SETHORIZONTALEXTENT,
                          LB_SETITEMDATA, LB_SETITEMHEIGHT, LB_SETSEL, LB_SETTABSTOPS,
                          LB_SETTOPINDEX};
pub const LB_OKAY: c_int = 0;
pub const LB_ERR: c_int = -1;
pub const LB_ERRSPACE: c_int = -2;

extern "system" {
    // these are incorrect in winapi 0.3.4
    pub fn lstrcpyW(lpString1: LPWSTR, lpString2: LPCWSTR) -> LPWSTR;
    pub fn lstrcatW(lpString1: LPWSTR, lpString2: LPCWSTR) -> LPWSTR;
}



// This should be in winapi::um::winnls
pub const LOCALE_STIMEFORMAT: LCTYPE = 0x00001003;
pub const LOCALE_NAME_USER_DEFAULT: LPCWSTR = null();

// There are some things missing from winapi,
// some are here to prevent the older 32 bit values being used accidentally
// and some that have been given an interesting interpretation are
// reinterpreted here.
pub const GWLP_WNDPROC    : c_int = -4;
pub const GWLP_HINSTANCE  : c_int = -6;  // Use GetWindowInstance()
pub const GWLP_HWNDPARENT : c_int = -8;  // Use SetParent()
pub const GWL_STYLE       : c_int = -16;
pub const GWL_EXSTYLE     : c_int = -20;
pub const GWLP_USERDATA   : c_int = -21;
pub const GWLP_ID         : c_int = -12;

pub const GCLP_MENUNAME      : c_int = -8;
pub const GCLP_HBRBACKGROUND : c_int = -10;
pub const GCLP_HCURSOR       : c_int = -12;
pub const GCLP_HICON         : c_int = -14;
pub const GCLP_HMODULE       : c_int = -16;
pub const GCL_CBWNDEXTRA     : c_int = -18;
pub const GCL_CBCLSEXTRA     : c_int = -20;
pub const GCL_STYLE          : c_int = -26;
pub const GCLP_WNDPROC       : c_int = -24;
pub const GCLP_HICONSM       : c_int = -34;

pub const GCW_ATOM : c_int = -32;

pub const TRANSPARENT : c_int = 1;
pub const OPAQUE      : c_int = 2;
pub const BKMODE_LAST : c_int = 2;

pub const MM_TEXT        : c_int = 1;
pub const MM_LOMETRIC    : c_int = 2;
pub const MM_HIMETRIC    : c_int = 3;
pub const MM_LOENGLISH   : c_int = 4;
pub const MM_HIENGLISH   : c_int = 5;
pub const MM_TWIPS       : c_int = 6;
pub const MM_ISOTROPIC   : c_int = 7;
pub const MM_ANISOTROPIC : c_int = 8;

pub const MM_MIN            : c_int = MM_TEXT;
pub const MM_MAX            : c_int = MM_ANISOTROPIC;
pub const MM_MAX_FIXEDSCALE : c_int = MM_TWIPS;

pub const ABSOLUTE : c_int = 1;
pub const RELATIVE : c_int = 2;

pub const WHITE_BRUSH         : c_int = 0;
pub const LTGRAY_BRUSH        : c_int = 1;
pub const GRAY_BRUSH          : c_int = 2;
pub const DKGRAY_BRUSH        : c_int = 3;
pub const BLACK_BRUSH         : c_int = 4;
pub const NULL_BRUSH          : c_int = 5;
pub const HOLLOW_BRUSH        : c_int = NULL_BRUSH;
pub const WHITE_PEN           : c_int = 6;
pub const BLACK_PEN           : c_int = 7;
pub const NULL_PEN            : c_int = 8;
pub const OEM_FIXED_FONT      : c_int = 10;
pub const ANSI_FIXED_FONT     : c_int = 11;
pub const ANSI_VAR_FONT       : c_int = 12;
pub const SYSTEM_FONT         : c_int = 13;
pub const DEVICE_DEFAULT_FONT : c_int = 14;
pub const DEFAULT_PALETTE     : c_int = 15;
pub const SYSTEM_FIXED_FONT   : c_int = 16;
pub const DEFAULT_GUI_FONT    : c_int = 17;
pub const DC_BRUSH            : c_int = 18;
pub const DC_PEN              : c_int = 19;
pub const STOCK_LAST          : c_int = 19;

pub const PS_SOLID       : c_int = 0;
pub const PS_DASH        : c_int = 1;
pub const PS_DOT         : c_int = 2;
pub const PS_DASHDOT     : c_int = 3;
pub const PS_DASHDOTDOT  : c_int = 4;
pub const PS_NULL        : c_int = 5;
pub const PS_INSIDEFRAME : c_int = 6;
pub const PS_USERSTYLE   : c_int = 7;
pub const PS_ALTERNATE   : c_int = 8;

pub const SB_HORZ : c_int = 0;
pub const SB_VERT : c_int = 1;
pub const SB_CTL  : c_int = 2;
pub const SB_BOTH : c_int = 3;

pub const SB_LINEUP        : c_int = 0;
pub const SB_LINELEFT      : c_int = 0;
pub const SB_LINEDOWN      : c_int = 1;
pub const SB_LINERIGHT     : c_int = 1;
pub const SB_PAGEUP        : c_int = 2;
pub const SB_PAGELEFT      : c_int = 2;
pub const SB_PAGEDOWN      : c_int = 3;
pub const SB_PAGERIGHT     : c_int = 3;
pub const SB_THUMBPOSITION : c_int = 4;
pub const SB_THUMBTRACK    : c_int = 5;
pub const SB_TOP           : c_int = 6;
pub const SB_LEFT          : c_int = 6;
pub const SB_BOTTOM        : c_int = 7;
pub const SB_RIGHT         : c_int = 7;
pub const SB_ENDSCROLL     : c_int = 8;

pub const DDL_READWRITE : UINT = 0x0000;
pub const DDL_READONLY  : UINT = 0x0001;
pub const DDL_HIDDEN    : UINT = 0x0002;
pub const DDL_SYSTEM    : UINT = 0x0004;
pub const DDL_DIRECTORY : UINT = 0x0010;
pub const DDL_ARCHIVE   : UINT = 0x0020;

pub const DDL_POSTMSGS  : UINT = 0x2000;
pub const DDL_DRIVES    : UINT = 0x4000;
pub const DDL_EXCLUSIVE : UINT = 0x8000;


// This performs the conversion from Rust str to Windows WSTR
// Use this function to convert and then use its returned value's .as_ptr()
// method to get the LPWSTR.
pub fn to_wstr(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}

#[allow(non_snake_case)]
#[inline]
pub fn MAKELPARAM(l: WORD, h: WORD) -> LPARAM {
    MAKELONG(l, h) as DWORD as LPARAM
}

// These came from windowx.h and should ideally be in
// winapi::shared::windowsx
//
#[allow(non_snake_case)]
#[inline]
pub fn GetWindowInstance(hwnd: HWND) -> HMODULE {
    unsafe { GetWindowLongPtrW(hwnd, GWLP_HINSTANCE) as HMODULE }
}

#[allow(non_snake_case)]
#[inline]
pub fn GET_WM_HSCROLL_CODE(wp: WPARAM, _lp: LPARAM) -> c_int {
    LOWORD(wp as DWORD) as c_int
}

#[deprecated(note = "Please use GetScrollInfo instead")]
#[allow(non_snake_case)]
#[inline]
pub fn GET_WM_HSCROLL_POS(wp: WPARAM, _lp: LPARAM) -> c_int {
    HIWORD(wp as DWORD) as c_int
}

#[allow(non_snake_case)]
#[inline]
pub fn GET_WM_VSCROLL_CODE(wp: WPARAM, _lp: LPARAM) -> c_int {
    LOWORD(wp as DWORD) as c_int
}

#[deprecated(note = "Please use GetScrollInfo instead")]
#[allow(non_snake_case)]
#[inline]
pub fn GET_WM_VSCROLL_POS(wp: WPARAM, _lp: LPARAM) -> c_int {
    HIWORD(wp as DWORD) as c_int
}

//****** GDI Macro APIs ******************************************************

#[allow(non_snake_case)]
#[inline]
pub fn DeletePen(hpen: HPEN) -> BOOL {
    unsafe { DeleteObject(hpen as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn SelectPen(hdc: HDC, hpen: HPEN) -> HPEN {
    unsafe { SelectObject(hdc, hpen as HGDIOBJ) as HPEN }
}

#[allow(non_snake_case)]
#[inline]
pub fn GetStockPen(i: c_int) -> HPEN {
    unsafe { GetStockObject(i) as HPEN }
}

#[allow(non_snake_case)]
#[inline]
pub fn DeleteBrush(hbrush: HBRUSH) -> BOOL {
    unsafe { DeleteObject(hbrush as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn SelectBrush(hdc: HDC, hbrush: HBRUSH) -> HBRUSH {
    unsafe { SelectObject(hdc, hbrush as HGDIOBJ) as HBRUSH }
}

#[allow(non_snake_case)]
#[inline]
pub fn GetStockBrush(i: c_int) -> HBRUSH {
    unsafe { GetStockObject(i) as HBRUSH }
}


#[allow(non_snake_case)]
#[inline]
pub fn DeleteRgn(hrgn: HRGN) -> BOOL {
    unsafe { DeleteObject(hrgn as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn CopyRgn(hrgn_dst: HRGN, hrgn_src: HRGN) -> c_int {
    unsafe { CombineRgn(hrgn_dst, hrgn_src, null_mut(), RGN_COPY) }
}

#[allow(non_snake_case)]
#[inline]
pub fn IntersectRgn(hrgn_result: HRGN, hrgn_a: HRGN, hrgn_b: HRGN) -> c_int {
    unsafe { CombineRgn(hrgn_result, hrgn_a, hrgn_b, RGN_AND) }
}

#[allow(non_snake_case)]
#[inline]
pub fn SubtractRgn(hrgn_result: HRGN, hrgn_a: HRGN, hrgn_b: HRGN) -> c_int {
    unsafe { CombineRgn(hrgn_result, hrgn_a, hrgn_b, RGN_DIFF) }
}

#[allow(non_snake_case)]
#[inline]
pub fn UnionRgn(hrgn_result: HRGN, hrgn_a: HRGN, hrgn_b: HRGN) -> c_int {
    unsafe { CombineRgn(hrgn_result, hrgn_a, hrgn_b, RGN_OR) }
}

#[allow(non_snake_case)]
#[inline]
pub fn XorRgn(hrgn_result: HRGN, hrgn_a: HRGN, hrgn_b: HRGN) -> c_int {
    unsafe { CombineRgn(hrgn_result, hrgn_a, hrgn_b, RGN_XOR) }
}

#[allow(non_snake_case)]
#[inline]
pub fn DeletePalette(hpal: HPALETTE) -> BOOL {
    unsafe { DeleteObject(hpal as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn DeleteFont(hfont: HFONT) -> BOOL {
    unsafe { DeleteObject(hfont as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn SelectFont(hdc: HDC, hfont: HFONT) -> HFONT {
    unsafe { SelectObject(hdc, hfont as HGDIOBJ) as HFONT }
}

#[allow(non_snake_case)]
#[inline]
pub fn GetStockFont(i: c_int) -> HFONT {
    unsafe { GetStockObject(i) as HFONT }
}

#[allow(non_snake_case)]
#[inline]
pub fn DeleteBitmap(hbitmap: HBITMAP) -> BOOL {
    unsafe { DeleteObject(hbitmap as HGDIOBJ) }
}

#[allow(non_snake_case)]
#[inline]
pub fn SelectBitmap(hdc: HDC, hbitmap: HBITMAP) -> HBITMAP {
    unsafe { SelectObject(hdc, hbitmap as HGDIOBJ) as HBITMAP }
}

#[allow(non_snake_case)]
#[inline]
pub fn InsetRect(lprect: LPRECT, dx: c_int, dy: c_int) -> BOOL {
    unsafe { InflateRect(lprect, -dx, -dy) }
}

//****** Listbox Macro APIs **************************************************

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_Enable(hwndCtl: HWND, fEnable: BOOL) -> BOOL {
    unsafe { EnableWindow(hwndCtl, fEnable) }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetCount(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETCOUNT, 0, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_ResetContent(hwndCtl: HWND) -> BOOL {
    unsafe { SendMessageW(hwndCtl, LB_RESETCONTENT, 0, 0) as DWORD as BOOL }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_AddString(hwndCtl: HWND, lpsz: LPCWSTR) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_ADDSTRING, 0, lpsz as LPARAM) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_InsertString(hwndCtl: HWND, index: c_int, lpsz: LPCWSTR) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_INSERTSTRING, index as WPARAM, lpsz as LPARAM) as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_AddItemData(hwndCtl: HWND, data: LPARAM) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_ADDSTRING, 0, data) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_InsertItemData(hwndCtl: HWND, index: c_int, data: LPARAM) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_INSERTSTRING, index as WPARAM, data) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_DeleteString(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_DELETESTRING, index as WPARAM, 0) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetTextLen(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETTEXTLEN, index as WPARAM, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetText(hwndCtl: HWND, index: c_int, lpszBuffer: LPWSTR) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_GETTEXT, index as WPARAM, lpszBuffer as LPARAM) as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetItemData(hwndCtl: HWND, index: c_int) -> LRESULT {
    unsafe { SendMessageW(hwndCtl, LB_GETITEMDATA, index as WPARAM, 0) as ULONG_PTR as LRESULT }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetItemData(hwndCtl: HWND, index: c_int, data: LPARAM) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_SETITEMDATA, index as WPARAM, data) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_FindString(hwndCtl: HWND, indexStart: c_int, lpszFind: LPCWSTR) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_FINDSTRING, indexStart as WPARAM, lpszFind as LPARAM)
            as DWORD as c_int
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_FindItemDate(hwndCtl: HWND, indexStart: c_int, data: LPARAM) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_FINDSTRING, indexStart as WPARAM, data) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetSel(hwndCtl: HWND, fSelect: BOOL, index: c_int) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_SETSEL, fSelect as WPARAM, index as LPARAM) as DWORD as c_int
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SelItemRange(hwndCtl: HWND, fSelect: BOOL, first: WORD, last: WORD) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_SELITEMRANGE, fSelect as WPARAM, MAKELPARAM(first, last))
            as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetCurSel(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETCURSEL, 0, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetCurSel(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_SETCURSEL, index as WPARAM, 0) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SelectString(hwndCtl: HWND, indexStart: c_int, lpszFind: LPCWSTR) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_SELECTSTRING, indexStart as WPARAM, lpszFind as LPARAM)
            as DWORD as c_int
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SelectItemData(hwndCtl: HWND, indexStart: c_int, data: LPARAM) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_SELECTSTRING, indexStart as WPARAM, data) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetSel(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETSEL, index as WPARAM, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetSelCount(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETSELCOUNT, 0, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetTopIndex(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETTOPINDEX, 0, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetSelItems(hwndCtl: HWND, cItems: c_int, lpItems: *const c_int) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_GETSELITEMS, cItems as WPARAM, lpItems as LPARAM) as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetTopIndex(hwndCtl: HWND, indexTop: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_SETTOPINDEX, indexTop as WPARAM, 0) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetColumnWidth(hwndCtl: HWND, cxColumn: c_int) {
    unsafe { SendMessageW(hwndCtl, LB_SETCOLUMNWIDTH, cxColumn as WPARAM, 0); }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetHorizontalExtent(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETHORIZONTALEXTENT, 0, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetHorizontalExtent(hwndCtl: HWND, cxExtent: c_int) {
    unsafe { SendMessageW(hwndCtl, LB_SETHORIZONTALEXTENT, cxExtent as WPARAM, 0); }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetTabStops(hwndCtl: HWND, cTabs: c_int, lpTabs: *const c_int) -> BOOL {
    unsafe { SendMessageW(hwndCtl, LB_SETTABSTOPS, cTabs as WPARAM, lpTabs as LPARAM) as BOOL }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetItemRect(hwndCtl: HWND, index: c_int, lprc: *const RECT) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_GETITEMRECT, index as WPARAM, lprc as LPARAM) as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetCaretIndex(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_SETCARETINDEX, index as WPARAM, 0) as DWORD as c_int }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetCaretIndex(hwndCtl: HWND) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETCARETINDEX, 0, 0) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_FindStringExact(hwndCtl: HWND, indexStart: c_int, lpszFind: LPCWSTR) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_FINDSTRINGEXACT, indexStart as WPARAM, lpszFind as LPARAM)
            as DWORD as c_int
    }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_SetItemHeight(hwndCtl: HWND, index: c_int, cy: WORD) -> c_int {
    unsafe {
        SendMessageW(hwndCtl, LB_SETITEMHEIGHT, index as WPARAM, MAKELPARAM(cy, 0))
            as DWORD as c_int
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ListBox_GetItemHeight(hwndCtl: HWND, index: c_int) -> c_int {
    unsafe { SendMessageW(hwndCtl, LB_GETITEMHEIGHT, index as WPARAM, 0) as DWORD as c_int }
}


#[allow(non_snake_case)]
#[inline]
pub fn ListBox_Dir(hwndCtl: HWND, attrs: UINT, lpszFileSpec: LPCWSTR) -> c_int {
    // attrs as UINT as WPARAM
    unsafe { SendMessageW(hwndCtl, LB_DIR, attrs as WPARAM, lpszFileSpec as LPARAM) as DWORD as c_int }
}
