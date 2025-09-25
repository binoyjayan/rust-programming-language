//! Normal system call demonstration in Rust
//!
//! This program demonstrates how to make syscalls using libc.

#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
}

fn write_message(message: String) -> std::io::Result<isize> {
    let msg_ptr = message.as_ptr();
    let msg_len = message.len();

    let res = unsafe { write(1, msg_ptr, msg_len) };

    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn main() {
    let message = String::from("Hello, world from syscall via libc!\n");
    let res = write_message(message).expect("syscall failed");
    println!("Wrote {} bytes", res);
}
