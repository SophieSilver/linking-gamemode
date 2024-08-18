use libc::pid_t;
use std::ffi::c_int;

#[link(name = "ffi_stub", kind = "static")]
extern "C" {
    fn gamemode_request_start_for_wrapper(pid: pid_t) -> c_int;
}

fn main() {
    println!("Hello, world!");
    let result = unsafe { gamemode_request_start_for_wrapper(libc::getpid()) };
    println!("{result}");
}
