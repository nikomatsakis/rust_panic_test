extern crate libc;
extern crate libcruby_sys as sys;

use std::ffi::CString;

// This will become Object#panic in Ruby
#[no_mangle]
pub extern "C" fn panic() -> sys::VALUE {
    let res = ::std::panic::catch_unwind(|| {
        panic!("Panic!")
    });

    println!("Caught: {:?}", res);

    if let Err(_) = res {
        let msg = format!("Panicked in Rust");
        let ptr = msg.as_ptr();
        let len = msg.len();
        let msg = unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) };
        // WARNING: This will immediately exit out of Rust skipping all destructors
        unsafe { sys::rb_raise(sys::rb_eRuntimeError, sys::PRINT_VALUE_STR, msg); }
    }

    unsafe { sys::Qnil }
}

// This method is called when Ruby loads the native extension
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    unsafe {
        // This is a built-in Ruby method that allows us to define C methods on Ruby object
        sys::rb_define_method(sys::rb_cObject, CString::new("panic").unwrap().as_ptr(), panic as *const libc::c_void, 0);
    }
}
