use std::num::NonZeroUsize;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_init::mach_thread_self;
use mach2::mach_types::thread_act_array_t;
use mach2::message::mach_msg_type_number_t;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut state = [0u32; 1296];
    let mut count: u32 = 0;
    let rc = unsafe {
        let task = mach_task_self();
        let thread = mach_thread_self();

        task_threads(task, &mut state as *mut thread_act_array_t, &mut count)
    };

    if rc == KERN_SUCCESS {
        NonZeroUsize::new(usize::try_from(count).expect("Failed converting from u32 to usize"))
    } else {
        None
    }
}
