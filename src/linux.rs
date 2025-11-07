use std::num::NonZeroUsize;
use std::{fs, io, process};

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut amount: Option<NonZeroUsize> = None;
    let status = fs::read_to_string(format!("/proc/{}/status", process::id()))
        .expect("Failed reading status");

    for line in status.split('\n') {
        if line.to_lowercase().starts_with("threads:") {
            let new_line = line.to_lowercase().replace("threads:", "");

            amount = NonZeroUsize::new(
                new_line.trim().parse::<usize>().expect("Failed parsing into usize"),
            );
        }
    }

    amount
}

pub(crate) fn is_single_threaded() -> bool {
    unsafe {
        let ret = libc::unshare(libc::CLONE_THREAD);

        if ret == 0 {
            // Success: The kernel confirms we are single-threaded.
            return true;
        }

        // Failure: Check the error
        let err = io::Error::last_os_error().raw_os_error();
        if err == Some(libc::EINVAL) {
            // EINVAL: This is the documented error for being multithreaded.
            return false;
        }
    }

    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
