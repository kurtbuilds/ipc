// shm_parent.rs
use shared_memory::{ShmemConf, ShmemError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{thread, process::Command};
use libc::{pthread_cond_t, pthread_mutex_t, pthread_condattr_t, pthread_mutexattr_t, PTHREAD_PROCESS_SHARED};
use std::io::{self};
use ipc::SharedMemoryLayout;

fn main() -> io::Result<()> {
    let mut shmem = SharedMemoryLayout::new("shm_example2").unwrap();
    Command::new("./shm_child").spawn().expect("Failed to start child process");
    eprintln!("Wait for true");
    shmem.wait_for_true();
    eprintln!("Parent waiting for signal from child");
    shmem.wait();
    eprintln!("Parent received signal from child");
    Ok(())
}