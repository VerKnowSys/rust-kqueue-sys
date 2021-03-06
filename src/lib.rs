#![no_std]
#[macro_use]
extern crate bitflags;
extern crate libc;

#[allow(unused_imports)]
use libc::{c_int, uintptr_t, c_short, c_ushort, c_uint, int64_t, intptr_t, uint32_t, c_void,
           size_t, timespec};

pub mod constants;

pub use self::constants::*;

#[cfg(not(target_os="netbsd"))]
pub type EventListSize = c_int;

#[cfg(target_os="netbsd")]
pub type EventListSize = size_t;

#[cfg(not(target_os="netbsd"))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    pub ident: uintptr_t,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: int64_t,
    pub udata: *mut c_void,
}

#[cfg(target_os="netbsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct kevent {
    pub ident: uintptr_t,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: int64_t,
    pub udata: intptr_t,
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn kqueue() -> c_int;

    pub fn kevent(kq: c_int,
                  changelist: *const kevent,
                  nchanges: EventListSize,
                  eventlist: *mut kevent,
                  nevents: EventListSize,
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
