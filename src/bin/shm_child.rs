// shm_child.rs
use shared_memory::{ShmemConf, ShmemError};
use std::sync::atomic::Ordering;
use std::time::Duration;
use libc::{pthread_cond_signal, pthread_mutex_lock, pthread_mutex_unlock};

#[repr(C)]
struct SharedMemoryLayout {
    ready: std::sync::atomic::AtomicBool,
    mutex: libc::pthread_mutex_t,
    condition: libc::pthread_cond_t,
}

fn main() {
    // Open the shared memory
    let shmem_flink = "shm_example";
    let shmem = match ShmemConf::new().size(4096).flink(shmem_flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open().unwrap(),
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {shmem_flink} : {e}");
            return;
        }
    };

    let shm_ptr = shmem.as_ptr() as *mut SharedMemoryLayout;

    // Set the `ready` flag to true
    unsafe {
        (*shm_ptr).ready.store(true, Ordering::SeqCst);
    }

    // Loop, signaling the parent periodically
    loop {
        unsafe {
            // Lock the mutex, signal the condition, and unlock the mutex
            pthread_mutex_lock(&mut (*shm_ptr).mutex);
            pthread_cond_signal(&mut (*shm_ptr).condition);
            pthread_mutex_unlock(&mut (*shm_ptr).mutex);
        }

        println!("Child sent signal to parent");
        std::thread::sleep(Duration::from_secs(1));
    }
}