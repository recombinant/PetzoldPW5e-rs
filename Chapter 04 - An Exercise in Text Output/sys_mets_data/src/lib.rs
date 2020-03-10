//
// System metrics display structure
//
#![windows_subsystem = "windows"]
#![cfg(windows)]
extern crate winapi;

use winapi::ctypes::c_int;
use winapi::um::winuser::{
    SM_ARRANGE, SM_CLEANBOOT, SM_CMONITORS, SM_CMOUSEBUTTONS, SM_CXBORDER, SM_CXCURSOR,
    SM_CXDOUBLECLK, SM_CXDRAG, SM_CXEDGE, SM_CXFIXEDFRAME, SM_CXFULLSCREEN, SM_CXHSCROLL,
    SM_CXHTHUMB, SM_CXICON, SM_CXICONSPACING, SM_CXMAXIMIZED, SM_CXMAXTRACK, SM_CXMENUCHECK,
    SM_CXMENUSIZE, SM_CXMIN, SM_CXMINIMIZED, SM_CXMINSPACING, SM_CXMINTRACK, SM_CXSCREEN,
    SM_CXSIZE, SM_CXSIZEFRAME, SM_CXSMICON, SM_CXSMSIZE, SM_CXVIRTUALSCREEN, SM_CXVSCROLL,
    SM_CYBORDER, SM_CYCAPTION, SM_CYCURSOR, SM_CYDOUBLECLK, SM_CYDRAG, SM_CYEDGE, SM_CYFIXEDFRAME,
    SM_CYFULLSCREEN, SM_CYHSCROLL, SM_CYICON, SM_CYICONSPACING, SM_CYKANJIWINDOW, SM_CYMAXIMIZED,
    SM_CYMAXTRACK, SM_CYMENU, SM_CYMENUCHECK, SM_CYMENUSIZE, SM_CYMIN, SM_CYMINIMIZED,
    SM_CYMINSPACING, SM_CYMINTRACK, SM_CYSCREEN, SM_CYSIZE, SM_CYSIZEFRAME, SM_CYSMCAPTION,
    SM_CYSMICON, SM_CYSMSIZE, SM_CYVIRTUALSCREEN, SM_CYVSCROLL, SM_CYVTHUMB, SM_DBCSENABLED,
    SM_DEBUG, SM_MENUDROPALIGNMENT, SM_MIDEASTENABLED, SM_MOUSEPRESENT, SM_MOUSEWHEELPRESENT,
    SM_NETWORK, SM_PENWINDOWS, SM_SAMEDISPLAYFORMAT, SM_SECURE, SM_SHOWSOUNDS, SM_SLOWMACHINE,
    SM_SWAPBUTTON, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};

pub struct SysMetrics<'a> {
    pub index: c_int,
    pub label: &'a str,
    pub desc: &'a str,
}

pub const SYS_METRICS: &[SysMetrics] = &[
    SysMetrics {
        index: SM_CYSCREEN,
        label: "SM_CYSCREEN",
        desc: "Screen height in pixels",
    },
    SysMetrics {
        index: SM_CXVSCROLL,
        label: "SM_CXVSCROLL",
        desc: "Vertical scroll width",
    },
    SysMetrics {
        index: SM_CYHSCROLL,
        label: "SM_CYHSCROLL",
        desc: "Horizontal scroll height",
    },
    SysMetrics {
        index: SM_CXSCREEN,
        label: "SM_CXSCREEN",
        desc: "Screen width in pixels",
    },
    SysMetrics {
        index: SM_CYCAPTION,
        label: "SM_CYCAPTION",
        desc: "Caption bar height",
    },
    SysMetrics {
        index: SM_CXBORDER,
        label: "SM_CXBORDER",
        desc: "Window border width",
    },
    SysMetrics {
        index: SM_CYBORDER,
        label: "SM_CYBORDER",
        desc: "Window border height",
    },
    SysMetrics {
        index: SM_CXFIXEDFRAME,
        label: "SM_CXFIXEDFRAME",
        desc: "Dialog window frame width",
    },
    SysMetrics {
        index: SM_CYFIXEDFRAME,
        label: "SM_CYFIXEDFRAME",
        desc: "Dialog window frame height",
    },
    SysMetrics {
        index: SM_CYVTHUMB,
        label: "SM_CYVTHUMB",
        desc: "Vertical scroll thumb height",
    },
    SysMetrics {
        index: SM_CXHTHUMB,
        label: "SM_CXHTHUMB",
        desc: "Horizontal scroll thumb width",
    },
    SysMetrics {
        index: SM_CXICON,
        label: "SM_CXICON",
        desc: "Icon width",
    },
    SysMetrics {
        index: SM_CYICON,
        label: "SM_CYICON",
        desc: "Icon height",
    },
    SysMetrics {
        index: SM_CXCURSOR,
        label: "SM_CXCURSOR",
        desc: "Cursor width",
    },
    SysMetrics {
        index: SM_CYCURSOR,
        label: "SM_CYCURSOR",
        desc: "Cursor height",
    },
    SysMetrics {
        index: SM_CYMENU,
        label: "SM_CYMENU",
        desc: "Menu bar height",
    },
    SysMetrics {
        index: SM_CXFULLSCREEN,
        label: "SM_CXFULLSCREEN",
        desc: "Full screen client area width",
    },
    SysMetrics {
        index: SM_CYFULLSCREEN,
        label: "SM_CYFULLSCREEN",
        desc: "Full screen client area height",
    },
    SysMetrics {
        index: SM_CYKANJIWINDOW,
        label: "SM_CYKANJIWINDOW",
        desc: "Kanji window height",
    },
    SysMetrics {
        index: SM_MOUSEPRESENT,
        label: "SM_MOUSEPRESENT",
        desc: "Mouse present flag",
    },
    SysMetrics {
        index: SM_CYVSCROLL,
        label: "SM_CYVSCROLL",
        desc: "Vertical scroll arrow height",
    },
    SysMetrics {
        index: SM_CXHSCROLL,
        label: "SM_CXHSCROLL",
        desc: "Horizontal scroll arrow width",
    },
    SysMetrics {
        index: SM_DEBUG,
        label: "SM_DEBUG",
        desc: "Debug version flag",
    },
    SysMetrics {
        index: SM_SWAPBUTTON,
        label: "SM_SWAPBUTTON",
        desc: "Mouse buttons swapped flag",
    },
    SysMetrics {
        index: SM_CXMIN,
        label: "SM_CXMIN",
        desc: "Minimum window width",
    },
    SysMetrics {
        index: SM_CYMIN,
        label: "SM_CYMIN",
        desc: "Minimum window height",
    },
    SysMetrics {
        index: SM_CXSIZE,
        label: "SM_CXSIZE",
        desc: "Min/Max/Close button width",
    },
    SysMetrics {
        index: SM_CYSIZE,
        label: "SM_CYSIZE",
        desc: "Min/Max/Close button height",
    },
    SysMetrics {
        index: SM_CXSIZEFRAME,
        label: "SM_CXSIZEFRAME",
        desc: "Window sizing frame width",
    },
    SysMetrics {
        index: SM_CYSIZEFRAME,
        label: "SM_CYSIZEFRAME",
        desc: "Window sizing frame height",
    },
    SysMetrics {
        index: SM_CXMINTRACK,
        label: "SM_CXMINTRACK",
        desc: "Minimum window tracking width",
    },
    SysMetrics {
        index: SM_CYMINTRACK,
        label: "SM_CYMINTRACK",
        desc: "Minimum window tracking height",
    },
    SysMetrics {
        index: SM_CXDOUBLECLK,
        label: "SM_CXDOUBLECLK",
        desc: "Double click x tolerance",
    },
    SysMetrics {
        index: SM_CYDOUBLECLK,
        label: "SM_CYDOUBLECLK",
        desc: "Double click y tolerance",
    },
    SysMetrics {
        index: SM_CXICONSPACING,
        label: "SM_CXICONSPACING",
        desc: "Horizontal icon spacing",
    },
    SysMetrics {
        index: SM_CYICONSPACING,
        label: "SM_CYICONSPACING",
        desc: "Vertical icon spacing",
    },
    SysMetrics {
        index: SM_MENUDROPALIGNMENT,
        label: "SM_MENUDROPALIGNMENT",
        desc: "Left or right menu drop",
    },
    SysMetrics {
        index: SM_PENWINDOWS,
        label: "SM_PENWINDOWS",
        desc: "Pen extensions installed",
    },
    SysMetrics {
        index: SM_DBCSENABLED,
        label: "SM_DBCSENABLED",
        desc: "Double-Byte Char Set enabled",
    },
    SysMetrics {
        index: SM_CMOUSEBUTTONS,
        label: "SM_CMOUSEBUTTONS",
        desc: "Number of mouse buttons",
    },
    SysMetrics {
        index: SM_SECURE,
        label: "SM_SECURE",
        desc: "Security present flag",
    },
    SysMetrics {
        index: SM_CXEDGE,
        label: "SM_CXEDGE",
        desc: "3-D border width",
    },
    SysMetrics {
        index: SM_CYEDGE,
        label: "SM_CYEDGE",
        desc: "3-D border height",
    },
    SysMetrics {
        index: SM_CXMINSPACING,
        label: "SM_CXMINSPACING",
        desc: "Minimized window spacing width",
    },
    SysMetrics {
        index: SM_CYMINSPACING,
        label: "SM_CYMINSPACING",
        desc: "Minimized window spacing height",
    },
    SysMetrics {
        index: SM_CXSMICON,
        label: "SM_CXSMICON",
        desc: "Small icon width",
    },
    SysMetrics {
        index: SM_CYSMICON,
        label: "SM_CYSMICON",
        desc: "Small icon height",
    },
    SysMetrics {
        index: SM_CYSMCAPTION,
        label: "SM_CYSMCAPTION",
        desc: "Small caption height",
    },
    SysMetrics {
        index: SM_CXSMSIZE,
        label: "SM_CXSMSIZE",
        desc: "Small caption button width",
    },
    SysMetrics {
        index: SM_CYSMSIZE,
        label: "SM_CYSMSIZE",
        desc: "Small caption button height",
    },
    SysMetrics {
        index: SM_CXMENUSIZE,
        label: "SM_CXMENUSIZE",
        desc: "Menu bar button width",
    },
    SysMetrics {
        index: SM_CYMENUSIZE,
        label: "SM_CYMENUSIZE",
        desc: "Menu bar button height",
    },
    SysMetrics {
        index: SM_ARRANGE,
        label: "SM_ARRANGE",
        desc: "How minimized windows arranged",
    },
    SysMetrics {
        index: SM_CXMINIMIZED,
        label: "SM_CXMINIMIZED",
        desc: "Minimized window width",
    },
    SysMetrics {
        index: SM_CYMINIMIZED,
        label: "SM_CYMINIMIZED",
        desc: "Minimized window height",
    },
    SysMetrics {
        index: SM_CXMAXTRACK,
        label: "SM_CXMAXTRACK",
        desc: "Maximum draggable width",
    },
    SysMetrics {
        index: SM_CYMAXTRACK,
        label: "SM_CYMAXTRACK",
        desc: "Maximum draggable height",
    },
    SysMetrics {
        index: SM_CXMAXIMIZED,
        label: "SM_CXMAXIMIZED",
        desc: "Width of maximized window",
    },
    SysMetrics {
        index: SM_CYMAXIMIZED,
        label: "SM_CYMAXIMIZED",
        desc: "Height of maximized window",
    },
    SysMetrics {
        index: SM_NETWORK,
        label: "SM_NETWORK",
        desc: "Network present flag",
    },
    SysMetrics {
        index: SM_CLEANBOOT,
        label: "SM_CLEANBOOT",
        desc: "How system was booted",
    },
    SysMetrics {
        index: SM_CXDRAG,
        label: "SM_CXDRAG",
        desc: "Avoid drag x tolerance",
    },
    SysMetrics {
        index: SM_CYDRAG,
        label: "SM_CYDRAG",
        desc: "Avoid drag y tolerance",
    },
    SysMetrics {
        index: SM_SHOWSOUNDS,
        label: "SM_SHOWSOUNDS",
        desc: "Present sounds visually",
    },
    SysMetrics {
        index: SM_CXMENUCHECK,
        label: "SM_CXMENUCHECK",
        desc: "Menu check-mark width",
    },
    SysMetrics {
        index: SM_CYMENUCHECK,
        label: "SM_CYMENUCHECK",
        desc: "Menu check-mark height",
    },
    SysMetrics {
        index: SM_SLOWMACHINE,
        label: "SM_SLOWMACHINE",
        desc: "Slow processor flag",
    },
    SysMetrics {
        index: SM_MIDEASTENABLED,
        label: "SM_MIDEASTENABLED",
        desc: "Hebrew and Arabic enabled flag",
    },
    SysMetrics {
        index: SM_MOUSEWHEELPRESENT,
        label: "SM_MOUSEWHEELPRESENT",
        desc: "Mouse wheel present flag",
    },
    SysMetrics {
        index: SM_XVIRTUALSCREEN,
        label: "SM_XVIRTUALSCREEN",
        desc: "Virtual screen x origin",
    },
    SysMetrics {
        index: SM_YVIRTUALSCREEN,
        label: "SM_YVIRTUALSCREEN",
        desc: "Virtual screen y origin",
    },
    SysMetrics {
        index: SM_CXVIRTUALSCREEN,
        label: "SM_CXVIRTUALSCREEN",
        desc: "Virtual screen width",
    },
    SysMetrics {
        index: SM_CYVIRTUALSCREEN,
        label: "SM_CYVIRTUALSCREEN",
        desc: "Virtual screen height",
    },
    SysMetrics {
        index: SM_CMONITORS,
        label: "SM_CMONITORS",
        desc: "Number of monitors",
    },
    SysMetrics {
        index: SM_SAMEDISPLAYFORMAT,
        label: "SM_SAMEDISPLAYFORMAT",
        desc: "Same color format flag",
    },
];
