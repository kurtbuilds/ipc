// shm_child.rs
use shared_memory::{ShmemConf, ShmemError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use libc::{pthread_cond_signal, pthread_cond_t, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock};
use ipc::SharedMemoryLayout;

fn main() {
    // Open the shared memory
    eprintln!("Child: Opening shared memory");
    let mut shmem = SharedMemoryLayout::new("shm_example2").unwrap();
    shmem.set_true();
    eprintln!("Child: Sending signal");
    shmem.signal();
    eprintln!("Child: Signal sent. Exiting");
}