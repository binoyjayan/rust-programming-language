//! Module that deals with syscalls that are required to
//! communicate with the host operating system.

/// Operation value to add interest
pub const EPOLL_CTL_ADD: i32 = 1;
/// bitflag to indicate interest in read operations
pub const EPOLLIN: i32 = 0x1;
/// bitflag to indicate getting notified with epoll set to an edge-triggered mode
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
unsafe extern "C" {
    // syscall to make an epoll queue
    pub fn epoll_create(size: i32) -> i32;
    // syscall to close a file descriptor
    pub fn close(fd: i32) -> i32;
    // syscall to perform control operations (add, modify, delete) on the epoll queue
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    // Blocks until an event is available or the timeout is reached
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

/// struct used to communicate to the OS in epoll_ctl and epoll_wait
/// events is a bitmask that describes the type of event
#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    /// What kind of event to be notified about
    pub(crate) events: u32,
    /// User data that is associated with the event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
