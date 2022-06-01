use std::num::NonZeroUsize;

#[cfg_attr(target_os = "unix", path = "windows.rs")]
#[cfg_attr(target_family = "windows", path = "unix.rs")]
mod implementation;

pub fn thread_amount() -> Option<NonZeroUsize> {
    implementation::thread_amount()
}

pub fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    #[test]
    fn thread_amount() {
        for i in 0..5 {
            std::thread::spawn(move || {
                assert_eq!(super::thread_amount().map(NonZeroUsize::get), Some(i));
            });
        }
    }

    #[test]
    fn is_single_threaded() {
        for i in 0..5 {
            std::thread::spawn(move || {
                if i == 0 {
                    assert_eq!(super::is_single_threaded(), true);
                } else {
                    assert_eq!(super::is_single_threaded(), false);
                }
            });
        }
    }
}
