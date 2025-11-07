use std::num::NonZeroUsize;
use std::ptr;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        let kd = libc::kvm_open(
            ptr::null(),    // ufile
            ptr::null(),    // nfile
            ptr::null(),    // sfile
            libc::O_RDONLY, // flags
            ptr::null(),    // errstr
        );

        if kd.is_null() {
            return None;
        }

        let mut result: Option<NonZeroUsize> = None;
        let pid = libc::getpid();
        let mut count: libc::c_int = 0;
        let kinfo = libc::kvm_getprocs(kd, libc::KERN_PROC_PID, pid, &mut count);

        if !kinfo.is_null() && count == 1 {
            let thread_count = (*kinfo).ki_numthreads;

            if thread_count > 0 {
                result = NonZeroUsize::new(thread_count as usize);
            }
        }

        libc::kvm_close(kd);

        result
    }
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
