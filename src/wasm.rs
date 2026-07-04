use std::num::NonZeroUsize;

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    // WebAssembly does not expose OS-level process introspection.
    // Even if the host environment uses Web Workers, there is no API
    // to query the active count, and each worker is technically its own
    // isolated execution context. Therefore, from the perspective of
    // this executing Wasm instance, the active thread count is 1.
    NonZeroUsize::new(1)
}

#[inline]
pub(crate) fn is_single_threaded() -> bool {
    true
}
