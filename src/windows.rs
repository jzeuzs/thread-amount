use std::mem;
use std::num::NonZeroUsize;

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot,
    TH32CS_SNAPTHREAD,
    THREADENTRY32,
    Thread32First,
    Thread32Next,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

struct SnapshotHandle(HANDLE);

impl Drop for SnapshotHandle {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE && self.0 != 0 {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if handle == INVALID_HANDLE_VALUE || handle == 0 {
            return None;
        }

        let guard = SnapshotHandle(handle);

        let mut te32: THREADENTRY32 = std::mem::zeroed();
        te32.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        if Thread32First(guard.0, &mut te32) == 0 {
            return None;
        }

        let pid = GetCurrentProcessId();
        let mut count = 0;

        loop {
            if te32.th32OwnerProcessID == pid {
                count += 1;
            }

            if Thread32Next(guard.0, &mut te32) == 0 {
                break;
            }
        }

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
