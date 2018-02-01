//
#![cfg(windows)]
extern crate winapi;


use std::ptr::{null_mut, null};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LOWORD, HIWORD, MAKELONG, WORD, DWORD, WPARAM, LPARAM, BOOL, HRGN,
                                HMODULE};
use winapi::shared::windef::{HWND, HDC, HGDIOBJ, HPEN, HBRUSH, HFONT, HPALETTE, HBITMAP, LPRECT};
use winapi::shared::ntdef::{LPCWSTR, };
use winapi::um::winuser::{InflateRect, GetWindowLongPtrW };
use winapi::um::wingdi::{GetStockObject, SelectObject, DeleteObject, CombineRgn,
                         RGN_AND, RGN_COPY, RGN_DIFF, RGN_OR, RGN_XOR, };
use winapi::um::winnls::LCTYPE;

// This should be in winapi::um::winnls
pub const LOCALE_STIMEFORMAT: LCTYPE = 0x00001003;
pub const LOCALE_NAME_USER_DEFAULT: LPCWSTR = null();

// There are some things missing from winapi,
// some are here to prevent the older 32 bit values being used accidentally
// and some that have been given an interesting interpretation are
// reinterpreted here.
pub const GWLP_WNDPROC        : c_int = -4;
pub const GWLP_HINSTANCE      : c_int = -6;  // Use GetWindowInstance()
pub const GWLP_HWNDPARENT     : c_int = -8;  // Use SetParent()
pub const GWL_STYLE           : c_int = -16;
pub const GWL_EXSTYLE         : c_int = -20;
pub const GWLP_USERDATA       : c_int = -21;
pub const GWLP_ID             : c_int = -12;

pub const GCLP_MENUNAME       : c_int = -8;
pub const GCLP_HBRBACKGROUND  : c_int = -10;
pub const GCLP_HCURSOR        : c_int = -12;
pub const GCLP_HICON          : c_int = -14;
pub const GCLP_HMODULE        : c_int = -16;
pub const GCL_CBWNDEXTRA      : c_int = -18;
pub const GCL_CBCLSEXTRA      : c_int = -20;
pub const GCL_STYLE           : c_int = -26;
pub const GCLP_WNDPROC        : c_int = -24;
pub const GCLP_HICONSM        : c_int = -34;

pub const GCW_ATOM            : c_int = -32;

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

pub const SB_HORZ          : c_int = 0;
pub const SB_VERT          : c_int = 1;
pub const SB_CTL           : c_int = 2;
pub const SB_BOTH          : c_int = 3;

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
