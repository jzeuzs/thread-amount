use std::num::NonZeroUsize;
use std::ptr;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_init::mach_thread_self;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut state = [0u32; 1296];
    let mut count: u32 = 0;
    let rc = unsafe {
        let task = mach_task_self();
        let thread = mach_thread_self();

        task_threads(task, ptr::addr_of_mut!(state.as_mut_ptr()), &mut count)
    };

    if rc == KERN_SUCCESS {
        NonZeroUsize::new(usize::try_from(count).expect("Failed converting from u32 to usize"))
    } else {
        None
    }
}
