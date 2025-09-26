//! A thin abstraction over epoll

use std::{io, net::TcpStream, os::fd::AsRawFd};

use crate::ffi;

type Events = Vec<ffi::Event>;

/// Poll is the main interface to epoll similar to mio's Poll
pub struct Poll {
    registry: Registry,
}

impl Poll {
    /// Create a new event queue
    pub fn new() -> io::Result<Self> {
        let raw_fd = unsafe { ffi::epoll_create(1) };
        if raw_fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            registry: Registry::new(raw_fd),
        })
    }
    /// Returns a reference to the registry that can be
    /// used to register intereset to be notified of events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    /// Blocks the current thread until an event is available
    /// or the timeout is reached, whichever happens first.
    /// If timeout is None, it will block indefinitely.
    /// poll requires exclusive access to the object so
    /// when the caller is waiting for events, no other
    /// thread can register interest in events.
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> io::Result<()> {
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let maxevents = events.capacity() as i32;
        let ret = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), maxevents, timeout) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe { events.set_len(ret as usize) };
        Ok(())
    }
}

/// Registry is a handle that allows users to register interest in events
/// Moving the concern of registering interests to a separate struct
/// allows uses to use Registry::try_clone() to get an owned Registry instance.
/// The allows multiple threads to register interest to the same poll instance.
/// https://docs.rs/mio/0.8.8/mio/struct.Registry.html
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn new(raw_fd: i32) -> Self {
        Registry { raw_fd }
    }
    /// Register interest in events on a TcpStream
    /// Arguments:
    /// - source: The TcpStream to monitor for events
    /// - token: Associates the event with the source that generated the event
    /// - interests: The types of events to monitor (readable, writable, etc.)
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> io::Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };
        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe { ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };
        if res < 0 {
            let err = io::Error::last_os_error();
            eprintln!("Error closing epoll fd {}: {}", self.raw_fd, err);
        }
    }
}
