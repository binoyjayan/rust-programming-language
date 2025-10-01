use core::arch::asm;

const STACK_SIZE: isize = 48;
const STACK_ALIGN: usize = 16;

#[repr(C)]
#[derive(Debug, Default)]
struct ThreadContext {
    rsp: u64,
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; STACK_SIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().add(STACK_SIZE as usize);
        let sb_aligned = (stack_bottom as usize & !(STACK_ALIGN - 1)) as *mut u8;
        std::ptr::write(
            sb_aligned.offset(-(STACK_ALIGN as isize)) as *mut u64,
            hello as u64,
        );
        ctx.rsp = (sb_aligned.offset(-(STACK_ALIGN as isize))) as u64;

        // Show stack contents
        for i in 0..STACK_SIZE {
            println!(
                "mem: {}: {}",
                sb_aligned.offset(-i as isize) as usize,
                *sb_aligned.offset(-i as isize) as u8,
            );
        }
        gt_switch(&mut ctx);
    }
}

unsafe fn gt_switch(ctx: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) ctx,
    );
}

fn hello() {
    println!("WAKING UP ON A STACK!");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
