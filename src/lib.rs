#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi::CString;

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct VALUE(void_ptr);

#[cfg_attr(windows, link(name="helix-runtime"))]
extern "C" {
    #[link_name = "HELIX_Qnil"]
    pub static Qnil: VALUE;

    #[link_name = "rb_eRuntimeError"]
    pub static rb_eRuntimeError: VALUE;

    #[link_name = "rb_cObject"]
    pub static rb_cObject: VALUE;

    pub fn rb_raise(exc: VALUE, string: c_string, ...) -> !;
    pub fn rb_define_method(class: VALUE, name: c_string, func: void_ptr, arity: isize);
}

// This will become Object#panic in Ruby
#[no_mangle]
pub extern "C" fn panic() -> VALUE {
    let _ = ::std::panic::catch_unwind(|| {
        panic!("Panic!")
    });
    unsafe { Qnil }
}

// This will become Object#rust_raise in Ruby
#[no_mangle]
pub extern "C" fn rust_raise() -> ! {
    unsafe {
        rb_raise(rb_eRuntimeError, CString::new("Panicked in Rust").unwrap().as_ptr())
    }
}

#[no_mangle]
pub extern "C" fn panic_and_raise() -> ! {
    {
        let _ = panic();
    }
    rust_raise()
}

// This method is called when Ruby loads the native extension
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    unsafe {
        // This is a built-in Ruby method that allows us to define C methods on Ruby object
        rb_define_method(rb_cObject, CString::new("panic").unwrap().as_ptr(), panic as *const libc::c_void, 0);
        rb_define_method(rb_cObject, CString::new("rust_raise").unwrap().as_ptr(), rust_raise as *const libc::c_void, 0);
        rb_define_method(rb_cObject, CString::new("panic_and_raise").unwrap().as_ptr(), panic_and_raise as *const libc::c_void, 0);
    }
}
