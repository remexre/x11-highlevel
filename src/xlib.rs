use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use x11::xlib;

use xinput::InputDevices;

pub struct Display<'a> {
    ptr: *mut xlib::Display,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Display<'a> {
    /// Creates a new Display based on the `DISPLAY` environment variable.
    pub fn new() -> Option<Display<'a>> {
        use std::env;
        println!("{:?}", env::var("DISPLAY"));
        env::var("DISPLAY").ok().and_then(|s| Display::connect(&s))
    }

    /// Opens a connection to the X server.
    pub fn connect(display: &str) -> Option<Display<'a>> {
        let display = CString::new(display).unwrap();
        let ptr = unsafe { xlib::XOpenDisplay(display.as_ptr()) };
        if ptr == ptr::null_mut() {
            None
        } else {
            Some(unsafe { Display::from_ptr(ptr) })
        }
    }
}

impl<'a> Display<'a> {
    /// Creates a new Display from a raw pointer. This pointer must not be
    /// null.
    pub unsafe fn from_ptr(ptr: *mut xlib::Display) -> Display<'a> {
        Display {
            ptr: ptr,
            phantom: PhantomData,
        }
    }

    /// Returns the pointer underlying this struct.
    pub unsafe fn as_ptr(&self) -> *const xlib::Display {
        self.ptr as *const _
    }

    /// Returns the pointer underlying this struct.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut xlib::Display {
        self.ptr
    }
}

impl<'a> Display<'a> {
    /// Returns an iterator over the input devices connected to the X server.
    pub fn devices(&mut self) -> InputDevices<'a> {
        InputDevices::new(self)
    }
}
