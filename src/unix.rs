use std::num::NonZeroUsize;

use procfs::process::Process;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let process = Process::myself().expect("Failed getting process info");
    let status = process.status().expect("Failed getting status");

    NonZeroUsize::new(status.threads as usize)
}
