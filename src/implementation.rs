//! Fallback if no OS is matched.
use std::num::NonZeroUsize;

#[inline]
pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    None
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}
