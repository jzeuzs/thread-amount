use std::num::NonZeroUsize;
use std::ptr;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_types::{mach_port_t, thread_act_array_t, vm_address_t, vm_size_t};
use mach2::message::mach_msg_type_number_t;
use mach2::task::task_threads;
use mach2::traps::mach_task_self;
use mach2::vm::mach_vm_deallocate;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        let task = mach_task_self();
        let mut thread_list: thread_act_array_t = ptr::null_mut();
        let mut count: mach_msg_type_number_t = 0;
        let rc = task_threads(task, &mut thread_list, &mut count);

        if rc != KERN_SUCCESS {
            return None;
        }

        let result = NonZeroUsize::new(count as usize);
        let size = (count as usize * mem::size_of::<mach_port_t>()) as vm_size_t;
        let _ = mach_vm_deallocate(task, thread_list as vm_address_t, size);

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
