use std::num::NonZeroUsize;
use std::{fs, process};

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut amount: Option<NonZeroUsize> = None;
    let status = fs::read_to_string(format!("/proc/{}/status", process::id()))
        .expect("Failed reading status");

    for line in status.split('\n') {
        if line.to_lowercase().starts_with("threads:") {
            let new_line = line.replace(' ', "").to_lowercase().replace("threads:", "");

            amount =
                NonZeroUsize::new(new_line.parse::<usize>().expect("Failed parsing into usize"));
        }
    }

    amount
}
