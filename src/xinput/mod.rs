use libc::{c_int, c_ulong};
use std::ffi::CString;
use std::marker::PhantomData;
use x11::{xinput, xlib};

use Display;

/// An input device.
pub struct InputDevice<'a> {
    display: *mut xlib::Display,
    info: &'a xinput::XDeviceInfo,
    ptr: Option<*mut xinput::XDevice>,
}

impl<'a> InputDevice<'a> {
    fn new(display: *mut xlib::Display, info: &'a xinput::XDeviceInfo) -> InputDevice<'a> {
        let ptr = if info.use_ == 3 /* IsXExtensionKeyboard */ || info.use_ == 4 /* IsXExtensionPointer */ {
            Some(unsafe { xinput::XOpenDevice(display, info.id) })
        } else {
            None
        };
        InputDevice { display, info, ptr }
    }

    /// Returns the ID number of the device.
    pub fn id(&self) -> xlib::XID {
        self.info.id
    }

    /// Returns the name of the device as a Rust-owned string.
    pub fn name(&self) -> String {
        let cstr = unsafe { CString::from_raw(self.info.name) };
        cstr.into_string().unwrap()
    }
}

impl<'a> Drop for InputDevice<'a> {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr {
            unsafe { xinput::XCloseDevice(self.display, ptr); }
        }
    }
}

/// An iterator over the input devices present on the system.
pub struct InputDevices<'a> {
    count: c_int,
    display: *mut xlib::Display,
    idx: c_int,
    ptr: *mut xinput::XDeviceInfo,
    phantom: PhantomData<&'a ()>,
}

impl<'a> InputDevices<'a> {
    pub fn new(display: &mut Display<'a>) -> InputDevices<'a> {
        let display = unsafe { display.as_mut_ptr() };
        let mut count: c_int = 0;
        let ptr = unsafe { xinput::XListInputDevices(display, &mut count) };
        InputDevices {
            count: count,
            display: display,
            idx: 0,
            ptr: ptr,
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for InputDevices<'a> {
    fn drop(&mut self) {
        unsafe { xinput::XFreeDeviceList(self.ptr) }
    }
}

impl<'a> Iterator for InputDevices<'a> {
    type Item = InputDevice<'a>;
    
    fn next(&mut self) -> Option<InputDevice<'a>> {
        if self.idx >= self.count {
            return None;
        }
        let dev = unsafe { self.ptr.offset(self.idx as isize).as_ref().unwrap() };
        self.idx += 1;
        Some(InputDevice::new(self.display, dev))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.idx - self.count) as usize;
        (len, Some(len))
    }
}

/// A Rust-owned property for an input device.
pub enum InputDeviceProperty {
    // TODO
}

/// An iterator over the properties of an input device.
pub struct InputDevicePropertyIter<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> InputDevicePropertyIter<'a> {
    fn new() -> InputDevicePropertyIter<'a> {
        unimplemented!()
    }
}

impl<'a> Iterator for InputDevicePropertyIter<'a> {
    type Item = InputDeviceProperty;

    fn next(&mut self) -> Option<InputDeviceProperty> {
        unimplemented!()
    }
}
