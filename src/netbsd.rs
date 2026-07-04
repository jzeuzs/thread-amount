use std::num::NonZeroUsize;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        let mut mib = [
            libc::CTL_KERN,
            libc::KERN_PROC2,
            libc::KERN_PROC_PID,
            libc::getpid(),
            std::mem::size_of::<libc::kinfo_proc2>() as libc::c_int,
            1,
        ];

        let mut kp: libc::kinfo_proc2 = std::mem::zeroed();
        let mut size = std::mem::size_of::<libc::kinfo_proc2>();

        let result = libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            &mut kp as *mut _ as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        );

        if result == 0 && size == std::mem::size_of::<libc::kinfo_proc2>() {
            // NetBSD tracks threads as Lightweight Processes (LWPs)
            NonZeroUsize::new(kp.p_nlwps as usize)
        } else {
            None
        }
    }
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
