use std::num::NonZeroUsize;
use std::{mem, process};

use field_offset::offset_of;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot,
    Thread32First,
    Thread32Next,
    TH32CS_SNAPTHREAD,
    THREADENTRY32,
};

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let mut amount: usize = 0;
    let pid = process::id();

    // https://devblogs.microsoft.com/oldnewthing/20060223-14/?p=32173
    unsafe {
        let handle =
            CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0).expect("Failed creating snapshot");

        if !handle.is_invalid() {
            let mut te = THREADENTRY32 {
                dwSize: mem::size_of::<THREADENTRY32>() as u32,
                ..Default::default()
            };

            if Thread32First(handle, &mut te as *mut THREADENTRY32).as_bool() {
                while Thread32Next(handle, &mut te as *mut THREADENTRY32).as_bool() {
                    if te.dwSize as usize
                        >= (offset_of!(THREADENTRY32 => th32OwnerProcessID).get_byte_offset()
                            + mem::size_of::<u32>())
                        && te.th32OwnerProcessID == pid
                    {
                        amount += 1
                    }
                }
            }
        }
    }

    match amount {
        0 => None,
        num => NonZeroUsize::new(num),
    }
}
