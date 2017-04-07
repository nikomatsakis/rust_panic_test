extern crate libc;
extern crate libcruby_sys as sys;

use std::ffi::CString;

// This will become Object#panic in Ruby
#[no_mangle]
pub extern "C" fn panic() -> sys::VALUE {
    let _ = ::std::panic::catch_unwind(|| {
        panic!("Panic!")
    });
    unsafe { sys::Qnil }
}

// This will become Object#rust_raise in Ruby
#[no_mangle]
pub extern "C" fn rust_raise() -> ! {
    unsafe {
        sys::rb_raise(sys::rb_eRuntimeError, CString::new("Panicked in Rust").unwrap().as_ptr())
    }
}

// This method is called when Ruby loads the native extension
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    unsafe {
        // This is a built-in Ruby method that allows us to define C methods on Ruby object
        sys::rb_define_method(sys::rb_cObject, CString::new("panic").unwrap().as_ptr(), panic as *const libc::c_void, 0);
        sys::rb_define_method(sys::rb_cObject, CString::new("rust_raise").unwrap().as_ptr(), rust_raise as *const libc::c_void, 0);
    }
}
