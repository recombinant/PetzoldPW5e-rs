(2020-09-09 successfully compiled and run with Rust 1.46.0 on Windows 7 and Windows 10)

# PetzoldPW5e-rs
64 bit Rust transliteration of Charles Petzold's excellent book **Programming Windows 
5th Edition ISBN-10 157231995X**

The C source code has been manually converted into Rust and tested. 
Only nine of the twenty three chapters have been transliterated (March 2019).

If you cannot understand what the Rust code does then consider buying Charles Petzold's 
Programming Windows book - the 5th edition is a classic and the last using C.
It weighs 2.4kg (that's over 5lb).

---

Microsoft Windows 10 is to some extent backwards compatible with Windows 1.0 release 
circa 1983. Today this gives us a legacy of some interesting choices of types used 
in functions and constants. There was probably good reasoning behind it all,
but this does cause a lot of potentially unnecessary casting when using the Rust
*winapi* crate. Search for "The Old New Thing" by Raymond Chen. His blog contains
anecdotes relating to various design choices in Windows.
___

There is some code in the *extras* directory that is transliterated from 
*windowsx.h* file that was not available in the Rust *winapi* crate in January 2018.
There are also variations on some constants that have no explicit type in the 
Windows header files but have been given different types in the Rust *winapi* crate.

And don't forget the Windows functions and structures that have been superseded:

- RegisterClass, WNDCLASS
- GetLocaleInfo, LOCALE_SDATE, LOCALE_STIME, LOCALE_IDATE, LOCALE_ILDATE, LOCALE_ITIME, 
  LOCALE_ITIMEMARKPOSN, LOCALE_ICENTURY, LOCALE_ITLZERO, LOCALE_IDAYLZERO, LOCALE_IMONLZERO
- ScrollWindow
- SetClassLong, GetClassLong, SetWindowLong, GetWindowLong, GWL_WNDPROC, GWL_HINSTANCE, 
  GWL_HWNDPARENT, GWL_USERDATA, GWL_ID, GCL_MENUNAME, GCL_HBRBACKGROUND, GCL_HCURSOR, GCL_HICON,
  GCL_HMODULE, GCL_WNDPROC, GCL_HICONSM  
- GetScrollPos, SetScrollPos, GetScrollRange, SetScrollRange
- KP_SALT
- midiInGetID
- ... and probably a few more.
