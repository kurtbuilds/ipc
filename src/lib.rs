use std::{io, thread};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use libc::{pthread_cond_t, pthread_condattr_t, pthread_mutex_t, pthread_mutexattr_t, PTHREAD_PROCESS_SHARED};
use shared_memory::{ShmemConf, ShmemError};

const PIPE_SIZE: usize = 65536;

#[repr(C)]
pub struct SharedMemoryHeader {
    ready: AtomicBool,
    mutex: pthread_mutex_t,
    condition: pthread_cond_t,
}

#[repr(C)]
pub struct SharedMemoryLayout {
    header: SharedMemoryHeader,
    data: [u8; PIPE_SIZE],
}

impl SharedMemoryLayout {
    pub fn new(p: &str) -> io::Result<Self> {
        let s = size_of::<Self>();
        let shmem = match ShmemConf::new().size(s).flink(p).create() {
            Ok(m) => m,
            Err(ShmemError::LinkExists) => ShmemConf::new().flink(p).open().unwrap(),
            Err(e) => {
                eprintln!("Unable to create or open shmem flink {p} : {e}");
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to create or open shared memory"));
            }
        };
        let shm_ptr = shmem.as_ptr() as *mut SharedMemoryLayout;
        unsafe {
            // Initialize the mutex attributes
            let mut mutex_attr: pthread_mutexattr_t = std::mem::zeroed();
            libc::pthread_mutexattr_init(&mut mutex_attr);
            libc::pthread_mutexattr_setpshared(&mut mutex_attr, PTHREAD_PROCESS_SHARED);

            // Initialize the condition attributes
            let mut cond_attr: pthread_condattr_t = std::mem::zeroed();
            libc::pthread_condattr_init(&mut cond_attr);
            libc::pthread_condattr_setpshared(&mut cond_attr, PTHREAD_PROCESS_SHARED);

            // Initialize the mutex and condition variables in shared memory
            libc::pthread_mutex_init(&raw mut (*shm_ptr).header.mutex, &mutex_attr);
            libc::pthread_cond_init(&mut (*shm_ptr).header.condition, &cond_attr);

            // Destroy the attributes
            libc::pthread_mutexattr_destroy(&mut mutex_attr);
            libc::pthread_condattr_destroy(&mut cond_attr);

            // Set `ready` to false initially
            (*shm_ptr).header.ready.store(false, Ordering::SeqCst);
        }
        Ok(unsafe {shm_ptr.read()})
    }

    pub fn wait(&mut self) {
        unsafe {
            libc::pthread_mutex_lock(&raw mut self.header.mutex);
            libc::pthread_cond_wait(&raw mut self.header.condition, &raw mut self.header.mutex);
            libc::pthread_mutex_unlock(&raw mut self.header.mutex);
        }
    }

    pub fn signal(&mut self) {
        unsafe {
            libc::pthread_mutex_lock(&raw mut self.header.mutex);
            libc::pthread_cond_signal(&raw mut self.header.condition);
            libc::pthread_mutex_unlock(&raw mut self.header.mutex);
        }
    }

    pub fn set_true(&mut self) {
        self.header.ready.store(true, Ordering::SeqCst);
    }

    pub fn wait_for_true(&mut self) {
        loop {
            if self.header.ready.load(Ordering::SeqCst) {
                break;
            } else {
                eprintln!("Parent waiting for true");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}