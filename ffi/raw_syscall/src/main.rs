//! Raw system call demonstration in Rust
//!
//! This program demonstrates how to make raw system calls using inline assembly
//! to write directly to stdout without using standard library functions.

use std::arch::asm;

#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let msg_len = message.len();

    unsafe {
        asm!(
            "mov rax, 1",          // Set syscall number (1 = sys_write)
            "mov rdi, 1",          // Set file descriptor (1 = stdout)
            "syscall",             // Invoke the system call
            // Input/output constraints:
            in("rsi") msg_ptr,     // RSI = pointer to message data
            in("rdx") msg_len,     // RDX = message length in bytes
            out("rax") _,          // Mark RAX as output (return value)
            out("rdi") _,          // Mark RDI as clobbered
            lateout("rsi") _,      // Mark RSI as clobbered after syscall
            lateout("rdx") _,      // Mark RDX as clobbered after syscall
        );
    }
}

fn main() {
    let message = String::from("Hello, world from raw syscall!\n");
    syscall(message);
}
