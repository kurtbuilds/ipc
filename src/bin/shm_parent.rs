// shm_parent.rs
use shared_memory::{ShmemConf, ShmemError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{thread, process::Command};
use libc::{pthread_cond_t, pthread_mutex_t, pthread_condattr_t, pthread_mutexattr_t, PTHREAD_PROCESS_SHARED};
use std::io::{self};

#[repr(C)]
struct SharedMemoryLayout {
    ready: AtomicBool,
    mutex: pthread_mutex_t,
    condition: pthread_cond_t,
}

fn main() -> io::Result<()> {

    // Create or open the shared memory
    let shmem_flink = "shm_example";
    let shmem = match ShmemConf::new().size(4096).flink(shmem_flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open().unwrap(),
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {shmem_flink} : {e}");
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to create or open shared memory"));
        }
    };
    let shm_ptr = shmem.as_ptr() as *mut SharedMemoryLayout;

    // Initialize shared memory layout
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
        libc::pthread_mutex_init(&mut (*shm_ptr).mutex, &mutex_attr);
        libc::pthread_cond_init(&mut (*shm_ptr).condition, &cond_attr);

        // Destroy the attributes
        libc::pthread_mutexattr_destroy(&mut mutex_attr);
        libc::pthread_condattr_destroy(&mut cond_attr);

        // Set `ready` to false initially
        (*shm_ptr).ready.store(false, Ordering::SeqCst);
    }

    // Spawn the child program
    Command::new("./shm_child").spawn().expect("Failed to start child process");

    // Wait for the child to set `ready` to true
    loop {
        if unsafe { (*shm_ptr).ready.load(Ordering::SeqCst) } {
            break;
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }

    // Wait on the condition variable signaled by the child
    loop {
        unsafe {
            libc::pthread_mutex_lock(&mut (*shm_ptr).mutex);
            libc::pthread_cond_wait(&mut (*shm_ptr).condition, &mut (*shm_ptr).mutex);
            libc::pthread_mutex_unlock(&mut (*shm_ptr).mutex);

            // Perform some action after being notified
            println!("Parent received signal from child");
        }
    }
    Ok(())
}