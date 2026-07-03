use std::num::NonZeroUsize;
use std::ptr;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        // In OpenBSD, KERN_PROC_SHOW_THREADS is 0x40000000.
        let mib_proc = libc::KERN_PROC_PID | 0x40000000;

        let mut mib = [
            libc::CTL_KERN,
            libc::KERN_PROC,
            mib_proc,
            libc::getpid(),
            std::mem::size_of::<libc::kinfo_proc>() as libc::c_int,
            0,
        ];

        let mut size: libc::size_t = 0;

        if libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            ptr::null_mut(),
            &mut size,
            ptr::null_mut(),
            0,
        ) < 0
        {
            return None;
        }

        if size == 0 {
            return None;
        }

        let count = (size / std::mem::size_of::<libc::kinfo_proc>()) + 4;
        mib[5] = count as libc::c_int;

        let mut kinfo: Vec<libc::kinfo_proc> = Vec::with_capacity(count);

        size = count * std::mem::size_of::<libc::kinfo_proc>();

        if libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            kinfo.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            ptr::null_mut(),
            0,
        ) < 0
        {
            return None;
        }

        kinfo.set_len(size / std::mem::size_of::<libc::kinfo_proc>());

        let mut thread_count = 0;
        for kp in kinfo.iter() {
            if kp.p_tid != -1 {
                thread_count += 1;
            }
        }

        if thread_count == 0 && !kinfo.is_empty() {
            thread_count = 1;
        }

        NonZeroUsize::new(thread_count)
    }
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
