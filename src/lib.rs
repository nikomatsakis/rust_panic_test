extern crate libc;
extern crate libcruby_sys as sys;

use std::ffi::CString;

#[no_mangle]
pub extern "C" fn panic() -> sys::VALUE {
    let res = ::std::panic::catch_unwind(|| {
        panic!("Panic!")
    });

    println!("Caught: {:?}", res);

    unsafe { sys::Qnil }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    unsafe {
        sys::rb_define_method(sys::rb_cObject, CString::new("panic").unwrap().as_ptr(), panic as *const libc::c_void, 0);
    }
}
