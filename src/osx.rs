use std::num::NonZeroUsize;
use std::ptr;

use mach2::kern_return::KERN_SUCCESS;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut state = [0u32; 1296];
    let mut count: u32 = 0;
    let rc =
        unsafe { task_threads(mask_task_self(), &mut state as *mut *mut _, &mut count as *mut _) };

    if rc == KERN_SUCCESS {
        NonZeroUsize::new(usize::try_from(count).expect("Failed converting from u32 to usize"))
    } else {
        None
    }
}
