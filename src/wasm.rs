use std::num::NonZeroUsize;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    // WebAssembly is single-threaded in standard contexts.
    // Even when using Web Workers, there's no native OS-level API to query
    // the process thread count from inside the Wasm module alone.
    NonZeroUsize::new(1)
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    true
}
