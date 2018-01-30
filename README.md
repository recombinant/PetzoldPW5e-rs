# PetzoldPW5e-rs
Rust transliteration of Charles Petzold's excellent book **Programming Windows 
5th Edition ISBN-10 157231995X**

The C source code has been manually converted into Rust and tested. There are 
twenty three chapters in total. Five have been completed so far (Jan 2018). My 
knowledge of Rust is slowly improving.

If you cannot understand what the code does then consider buying the book - 
the 5th edition is a classic and the last using C. It weighs 2.4kg (that's over 
5lb).

___


There is some code in the extras directory that is transliterated from 
*windowsx.h* file but is not available in the Rust *winapi* crate. And also 
some variations on some constants that have no explicit type in the 
Windows header files but have been given various types Rust *winapi* crate.

Windows is to some extent backwards compatible with Windows 1.0 release 
circa 1983. Today this gives us a legacy of some interesting choices of types used 
in functions and constants. There was probably good reasoning behind it all,
but this does cause
a lot of potentially unnecessary casting when using the Rust *winapi* crate. Search for
"The Old New Thing" by Raymond Chen. His blog contains anecdotes relating to various
design choices in Windows.

And don't forget the functions and structures that have been superseded:

- RegisterClass, WNDCLASS
- GetLocaleInfo, LOCALE_SDATE, LOCALE_STIME, LOCALE_IDATE, LOCALE_ILDATE, LOCALE_ITIME, LOCALE_ITIMEMARKPOSN, LOCALE_ICENTURY, LOCALE_ITLZERO, LOCALE_IDAYLZERO, LOCALE_IMONLZERO
- ScrollWindow
- GetScrollPos, SetScrollPos, GetScrollRange, SetScrollRange
- KP_SALT
- midiInGetID
- ... and probably a few more.
