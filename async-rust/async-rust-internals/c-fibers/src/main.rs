// Disable prologue/epilogue generation
use core::arch::{asm, naked_asm};

const DEFAULT_STACK_SIZE: isize = 1024 * 1024 * 2; // 2MB
const STACK_ALIGN: usize = 16;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;

/// Main runtime structure
pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

/// Possible states for a thread
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    /// Thread is available and ready to be assigned a task
    Available,
    /// Thread is currently running a task
    Running,
    Ready,
}

/// structure holds data for each thread
struct Thread {
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

/// The thread context holds data for the registers that
/// the CPU needs to resume execution on a stack
/// These are registers that are callee-saved in the x86-64 ABI
#[repr(C)]
#[derive(Debug, Default)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

impl Thread {
    /// New thread always starts in the available state
    /// A stack must not move once allocated.
    fn new() -> Self {
        Thread {
            stack: vec![0u8; DEFAULT_STACK_SIZE as usize],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }

    fn running() -> Self {
        Thread {
            stack: vec![0u8; DEFAULT_STACK_SIZE as usize],
            ctx: ThreadContext::default(),
            state: State::Running,
        }
    }
}

impl Runtime {
    /// Create a new runtime with a fixed number of threads
    fn new() -> Self {
        // Thread that runs until all tasks finishes executing
        let base_thread = Thread::running();
        let mut threads = vec![base_thread];
        // Remaining threads are available to run tasks
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|_| Thread::new()).collect();
        threads.append(&mut available_threads);
        // Set current thread to the base thread
        Runtime {
            threads,
            current: 0,
        }
    }

    /// Initialize the global runtime pointer
    /// Make sure not to do anything that invalidates
    /// the pointer to self once it is initialized
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    /// Start running the runtime
    /// Continously call the yield function until it returns false
    /// which means that there is no more work to do
    pub fn run(&mut self) {
        while self.t_yield() {}
        // exit the process when done
        std::process::exit(0);
    }

    /// Return function that is called when a thread finishes
    /// The user does not call this function directly
    /// The stack is set up in such a way that this function
    /// is called when the task is done
    fn t_return(&mut self) {
        // Do not do anything if the calling thread is the base thread
        if self.current != 0 {
            // Mark the thread as available to be assigned a new task
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    /// The yield function.
    /// The first part of the function is a simple RR scheduler
    /// Go through all threads and see any are ready to make progress
    /// If no thread is ready, we are done (return false)
    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos = (pos + 1) % self.threads.len();
            if pos == self.current {
                // No threads are ready to run
                return false;
            }
        }
        // If a thread is ready to be run, change the state of
        // the current thread to ready.
        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }
        // Also change the state of the new thread to running
        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;
        // Switch to the new thread by saving the context of the
        // old thread and restoring the context of the new thread
        // The new context can either be a new task or a context
        // representing an existing task that needs to be resumed
        // The switch function is a naked function and they do not
        // accept formal arguments. So they cannot be called directly
        // using regular function call such as switch(old, new).
        // This is to avoid compiler placing arguments in the registers
        // and to avoid saving/restoring caller saved registers.
        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            asm!(
                "call switch",
                in("rdi") old, // first argument
                in("rsi") new, // second argument
                clobber_abi("C"),
            );
        }
        // Prevent compiler from optimizing away the context switch code
        self.threads.len() > 0
    }

    /// Spawn takes a function pointer which is a task to be
    // run concurrently
    fn spawn(&mut self, f: fn()) {
        // Find an available thread to run the task
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("No available thread");
        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            // Align the memory segment for the stack to be 16 byte aligned
            let s_ptr = (s_ptr as usize & !(STACK_ALIGN - 1)) as *mut u8;
            // Write the address to the guard function that will be called
            // when the task finishes and the function returns
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            // Write the address to the skip function which is there to
            // handle the gap when returned from function 'f' and is simply
            // one instruction 'ret'.
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            // Write the address of 'f' which is the actual task
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            // Address of the function is now at the top of the stack and
            // called when 'ret' is executed
            available.ctx.rsp = (s_ptr.offset(-32)) as u64;
        }
        // Mark the thread as ready to be run
        available.state = State::Ready;
    }
} // impl Runtime

/// The guard function is called when the function 'f' that is passed
/// has returned. When f returns, the task is finished, so dereference
/// the Runtime and and call return function to return to the scheduler
fn guard() {
    unsafe {
        let r_ptr = RUNTIME as *mut Runtime;
        (*r_ptr).t_return();
    }
}

/// The skip function is there to handle the gap when returned from
/// It compiles down to just the ret instruction. It will pop the value
/// off the stack and jumps to the address it points to, in this case
/// the guard function.
#[unsafe(naked)]
unsafe extern "C" fn skip() {
    naked_asm!("ret");
}

/// Helper function to yield
fn yield_thread() {
    unsafe {
        let r_ptr = RUNTIME as *mut Runtime;
        (*r_ptr).t_yield();
    }
}

#[unsafe(naked)]
#[no_mangle]
unsafe extern "C" fn switch() {
    naked_asm!(
        "mov [rdi + 0x00], rsp",
        "mov [rdi + 0x08], r15",
        "mov [rdi + 0x10], r14",
        "mov [rdi + 0x18], r13",
        "mov [rdi + 0x20], r12",
        "mov [rdi + 0x28], rbx",
        "mov [rdi + 0x30], rbp",
        "mov rsp, [rsi + 0x00]",
        "mov r15, [rsi + 0x08]",
        "mov r14, [rsi + 0x10]",
        "mov r13, [rsi + 0x18]",
        "mov r12, [rsi + 0x20]",
        "mov rbx, [rsi + 0x28]",
        "mov rbp, [rsi + 0x30]",
        "ret",
    );
}

fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("THREAD 1 - STARTING");
        let id = 1;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 1 - FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 2 - STARTING");
        let id = 2;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 2 - FINISHED");
    });
    runtime.run();
}
