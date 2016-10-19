#![no_std]
#[allow(non_camel_case_types)]
#[macro_use]
extern crate bitflags;
extern crate libc;

#[allow(unused_imports)]
use libc::{c_int, uintptr_t, c_short, c_ushort, c_uint, int64_t, intptr_t, uint32_t, c_void, size_t, timespec};

pub mod constants;

pub use self::constants::*;

#[cfg(not(target_os="netbsd"))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    ident: uintptr_t,
    filter: EventFilter,
    flags: EventFlag,
    fflags: FilterFlag,
    data: int64_t,
    udata: *mut c_void,
}

#[cfg(target_os="netbsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    ident: uintptr_t,
    filter: EventFilter,
    flags: EventFlag,
    fflags: FilterFlag,
    data: int64_t,
    udata: intptr_t,
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn kqueue() -> c_int;

    #[cfg(not(target_os="netbsd"))]
    pub fn kevent(kq: c_int,
                  changelist: *const kevent,
                  nchanges: c_int,
                  eventlist: *mut kevent,
                  nevents: c_int,
                  timeout: *const timespec)
                  -> c_int;

    #[cfg(target_os="netbsd")]
    pub fn kevent(kq: c_int,
                  changelist: *const kevent,
                  nchanges: size_t,
                  eventlist: *mut kevent,
                  nevents: size_t,
                  timeout: *const timespec)
                  -> c_int;

    #[cfg(target_os="netbsd")]
    pub fn kqueue1(flags: c_int) -> c_int;
}

#[cfg(test)]
mod test {
    use super::kqueue;

    #[test]
    fn test_kqueue() {
        unsafe {
            assert!(kqueue() > 0);
        }
    }
}
