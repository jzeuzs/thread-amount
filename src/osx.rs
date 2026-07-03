use std::num::NonZeroUsize;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_types::thread_act_array_t;
use mach2::message::mach_msg_type_number_t;
use mach2::port::{mach_port_deallocate, mach_port_t};
use mach2::task::task_threads;
use mach2::traps::mach_task_self;
use mach2::vm::mach_vm_deallocate;
use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t};

struct MachThreadList {
    threads: thread_act_array_t,
    count: mach_msg_type_number_t,
}

impl Drop for MachThreadList {
    fn drop(&mut self) {
        if self.threads.is_null() || self.count == 0 {
            return;
        }

        unsafe {
            let threads_slice = std::slice::from_raw_parts(self.threads, self.count as usize);
            for &thread_port in threads_slice {
                mach_port_deallocate(mach_task_self(), thread_port);
            }

            let size = (self.count as usize) * std::mem::size_of::<mach_port_t>();

            mach_vm_deallocate(
                mach_task_self(),
                self.threads as mach2::vm_types::mach_vm_address_t,
                size as mach2::vm_types::mach_vm_size_t,
            );
        }
    }
}

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut thread_list: thread_act_array_t = std::ptr::null_mut();
    let mut thread_count: mach_msg_type_number_t = 0;

    unsafe {
        let result = task_threads(mach_task_self(), &mut thread_list, &mut thread_count);

        if result != KERN_SUCCESS {
            return None;
        }

        let _guard = MachThreadList {
            threads: thread_list,
            count: thread_count,
        };

        NonZeroUsize::new(thread_count as usize)
    }
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
