#[allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, uintptr_t, c_short, c_ushort, c_uint, int64_t, c_void, timespec};

#[repr(C)]
pub struct kevent {
    ident: uintptr_t,
    filter: c_short,
    flags: c_ushort,
    fflags: c_uint,
    data: int64_t,
    udata: *mut c_void,
}

extern "C" {
    pub fn kqueue() -> c_int;
    pub fn kevent(kq: c_int,
                  changelist: *const kevent,
                  nchanges: c_int,
                  eventlist: *mut kevent,
                  nevents: c_int,
                  timeout: *const timespec)
                  -> c_int;
}
