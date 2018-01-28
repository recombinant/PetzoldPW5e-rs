//
#![cfg(windows)]
extern crate winapi;


use std::ptr::null_mut;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LOWORD, HIWORD, MAKELONG, WORD, DWORD, WPARAM, LPARAM, BOOL, HRGN,
                                HMODULE};
use winapi::shared::windef::{HWND, HDC, HGDIOBJ, HPEN, HBRUSH, HFONT, HPALETTE, HBITMAP, LPRECT};
use winapi::um::winuser::{InflateRect, GetWindowLongPtrW, GWLP_HINSTANCE, };
use winapi::um::wingdi::{GetStockObject, SelectObject, DeleteObject, CombineRgn,
                         RGN_AND, RGN_COPY, RGN_DIFF, RGN_OR, RGN_XOR, };

// There are some things missing from winapi,
// and some that have been given an interesting interpretation
pub const NULL_BRUSH: c_int = winapi::um::wingdi::NULL_BRUSH as c_int;
pub const WHITE_BRUSH: c_int = winapi::um::wingdi::WHITE_BRUSH as c_int;
pub const BLACK_BRUSH: c_int = winapi::um::wingdi::BLACK_BRUSH as c_int;
pub const GRAY_BRUSH: c_int = winapi::um::wingdi::GRAY_BRUSH as c_int;
pub const WHITE_PEN: c_int = winapi::um::wingdi::WHITE_PEN as c_int;
pub const BLACK_PEN: c_int = winapi::um::wingdi::BLACK_PEN as c_int;

pub const PS_DASH: c_int = winapi::um::wingdi::PS_DASH as c_int;

pub const SB_VERT: c_int = winapi::um::winuser::SB_VERT as c_int;
pub const SB_HORZ: c_int = winapi::um::winuser::SB_HORZ as c_int;
pub const SB_TOP: c_int = winapi::um::winuser::SB_TOP as c_int;
pub const SB_BOTTOM: c_int = winapi::um::winuser::SB_BOTTOM as c_int;
pub const SB_LINEUP: c_int = winapi::um::winuser::SB_LINEUP as c_int;
pub const SB_LINEDOWN: c_int = winapi::um::winuser::SB_LINEDOWN as c_int;
pub const SB_PAGEUP: c_int = winapi::um::winuser::SB_PAGEUP as c_int;
pub const SB_PAGEDOWN: c_int = winapi::um::winuser::SB_PAGEDOWN as c_int;
pub const SB_THUMBPOSITION: c_int = winapi::um::winuser::SB_THUMBPOSITION as c_int;
pub const SB_LINELEFT: c_int = winapi::um::winuser::SB_LINELEFT as c_int;
pub const SB_LINERIGHT: c_int = winapi::um::winuser::SB_LINERIGHT as c_int;
pub const SB_PAGELEFT: c_int = winapi::um::winuser::SB_PAGELEFT as c_int;
pub const SB_PAGERIGHT: c_int = winapi::um::winuser::SB_PAGERIGHT as c_int;
pub const OEM_FIXED_FONT: c_int = winapi::um::wingdi::OEM_FIXED_FONT as c_int;
pub const ANSI_FIXED_FONT: c_int = winapi::um::wingdi::ANSI_FIXED_FONT as c_int;
pub const ANSI_VAR_FONT: c_int = winapi::um::wingdi::ANSI_VAR_FONT as c_int;
pub const SYSTEM_FONT: c_int = winapi::um::wingdi::SYSTEM_FONT as c_int;
pub const DEVICE_DEFAULT_FONT: c_int = winapi::um::wingdi::DEVICE_DEFAULT_FONT as c_int;
pub const DEFAULT_PALETTE: c_int = winapi::um::wingdi::DEFAULT_PALETTE as c_int;
pub const SYSTEM_FIXED_FONT: c_int = winapi::um::wingdi::SYSTEM_FIXED_FONT as c_int;
pub const DEFAULT_GUI_FONT: c_int = winapi::um::wingdi::DEFAULT_GUI_FONT as c_int;
pub const MM_ANISOTROPIC: c_int = winapi::um::wingdi::MM_ANISOTROPIC as c_int;
pub const MM_TEXT: c_int = winapi::um::wingdi::MM_TEXT as c_int;
pub const MM_LOMETRIC: c_int = winapi::um::wingdi::MM_LOMETRIC as c_int;
pub const MM_HIMETRIC: c_int = winapi::um::wingdi::MM_HIMETRIC as c_int;
pub const MM_LOENGLISH: c_int = winapi::um::wingdi::MM_LOENGLISH as c_int;
pub const MM_HIENGLISH: c_int = winapi::um::wingdi::MM_HIENGLISH as c_int;
pub const MM_TWIPS: c_int = winapi::um::wingdi::MM_TWIPS as c_int;
pub const TRANSPARENT: c_int = winapi::um::wingdi::TRANSPARENT as c_int;

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
