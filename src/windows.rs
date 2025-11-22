use std::mem;
use std::num::NonZeroUsize;

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot,
    TH32CS_SNAPTHREAD,
    THREADENTRY32,
    Thread32First,
    Thread32Next,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        let pid = GetCurrentProcessId();
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if handle == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut count = 0;

        let mut te: THREADENTRY32 = mem::zeroed();
        te.dwSize = mem::size_of::<THREADENTRY32>() as u32;

        if Thread32First(handle, &mut te) != 0 {
            loop {
                if te.th32OwnerProcessID == pid {
                    count += 1;
                }

                if Thread32Next(handle, &mut te) == 0 {
                    break;
                }
            }
        }

        CloseHandle(handle);

        NonZeroUsize::new(count)
    }
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
