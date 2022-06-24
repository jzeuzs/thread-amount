use std::mem;
use std::num::NonZeroUsize;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_init::mach_thread_self;
use mach2::mach_port::mach_port_deallocate;
use mach2::mach_types::{thread_act_array_t, thread_t};
use mach2::message::mach_msg_type_number_t;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;
use mach2::vm::mach_vm_deallocate;
use mach2::vm_types::mach_vm_address_t;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut threads: Vec<u32> = vec![];
    let mut count: u32 = 0;

    unsafe {
        let task = mach_task_self();
        let thread = mach_thread_self();
        let rc = task_threads(
            task,
            &mut threads as *mut thread_act_array_t,
            &mut count as *mut mach_msg_type_number_t,
        );

        assert_eq!(rc, KERN_SUCCESS, "Failed getting threads");

        mach_port_deallocate(task, thread);
        mach_vm_deallocate(
            task,
            threads as *const mach_vm_address_t,
            mem::size_of::<thread_t>()
                * usize::try_from(count).expect("Failed converting from u32 to usize"),
        );
    }

    NonZeroUsize::new(usize::try_from(count).expect("Failed converting from u32 to usize"))
}
