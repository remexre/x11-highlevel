use libc::{c_int, c_ulong};
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::uninitialized;
use std::os::raw::c_void;
use std::ptr::null_mut;
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

    /// Returns the property with the given name, if it exists.
    pub fn get_prop(&self, name: &str) -> Option<InputDeviceProperty> {
        self.ptr.and_then(|ptr| {
            let name = CString::new(name).unwrap();
            let atom = unsafe { xlib::XInternAtom(self.display, name.as_ptr(), xlib::True) };
            if atom == 0 {
                None
            } else {
                Some((ptr, atom))
            }
        }).and_then(|(ptr, atom)| {
            let mut type_: xlib::Atom = unsafe { uninitialized() };
            let mut format: c_int = 0;
            let mut items: c_ulong = 0;
            let mut bytes: c_ulong = 0;
            let ret = unsafe {
                xinput::XGetDeviceProperty(self.display, // display
                           ptr,                          // device
                           atom,                         // property
                           0,                            // offset
                           0,                            // length
                           xlib::False,                  // delete
                           xlib::AnyPropertyType as u64, // req_type
                           &mut type_,                   // actual_type_return
                           &mut format,                  // actual_format_return
                           &mut items,                   // nitems_return
                           &mut bytes,                   // bytes_after_return
                           null_mut())                   // prop_return
            };
            println!("{:?}", (ptr, atom));
            unimplemented!()
        })
    }

    /// Returns the ID number of the device.
    pub fn id(&self) -> xlib::XID {
        self.info.id
    }

    /// Returns an iterator over the properties of the input device.
    pub fn list_props(&self) -> InputDevicePropertyIter<'a> {
        if let Some(dev) = self.ptr {
            InputDevicePropertyIter::new(self.display, dev)
        } else {
            InputDevicePropertyIter::empty()
        }
    }

    /// Returns the name of the device as a Rust-owned string.
    pub fn name(&self) -> String {
        let cstr = unsafe { CString::from_raw(self.info.name) };
        cstr.into_string().unwrap()
    }

    /// Returns the product ID of the input device.
    pub fn product_id(&self) -> Option<(u16, u16)> {
        self.get_prop("Device Product ID").and_then(|p| match p {
            _ => panic!("Invalid type for Device Product ID"),
        })
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
    count: c_int,
    device: *mut xinput::XDevice,
    display: *mut xlib::Display,
    idx: c_int,
    ptr: *mut xlib::Atom,
    phantom: PhantomData<&'a ()>,
}

impl<'a> InputDevicePropertyIter<'a> {
    fn empty() -> InputDevicePropertyIter<'a> {
        InputDevicePropertyIter {
            count: 0,
            device: null_mut(),
            display: null_mut(),
            idx: 0,
            ptr: null_mut(),
            phantom: PhantomData,
        }
    }

    fn new(display: *mut xlib::Display, device: *mut xinput::XDevice) -> InputDevicePropertyIter<'a> {
        let mut count: c_int = 0;
        let ptr = unsafe { xinput::XListDeviceProperties(display, device, &mut count) };
        InputDevicePropertyIter {
            count: count,
            device: device,
            display: display,
            idx: 0,
            ptr: ptr,
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for InputDevicePropertyIter<'a> {
    fn drop(&mut self) {
        unsafe { xlib::XFree(self.ptr as *mut c_void); }
    }
}

impl<'a> Iterator for InputDevicePropertyIter<'a> {
    type Item = InputDeviceProperty;
    
    fn next(&mut self) -> Option<InputDeviceProperty> {
        if self.idx >= self.count {
            return None;
        }
        let atom = unsafe { self.ptr.offset(self.idx as isize).as_ref().unwrap() };
        self.idx += 1;

        println!("atom == {}", atom);
        unimplemented!();
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.idx - self.count) as usize;
        (len, Some(len))
    }
}
