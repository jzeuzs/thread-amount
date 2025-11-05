use std::num::NonZeroUsize;

use mach2::kern_return::KERN_SUCCESS;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut state = [0u32; 1296];
    let mut count: u32 = 0;
    let rc = unsafe {
        task_threads(
            mach_task_self(),
            &mut state.as_mut_ptr() as *mut *mut u32,
            &mut count as *mut _,
        )
    };

    if rc == KERN_SUCCESS {
        NonZeroUsize::new(usize::try_from(count).expect("Failed converting from u32 to usize"))
    } else {
        None
    }
}
