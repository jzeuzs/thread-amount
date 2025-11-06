#![warn(clippy::pedantic)]

use std::num::NonZeroUsize;

#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "osx.rs")]
#[cfg_attr(target_os = "linux", path = "unix.rs")]
#[cfg_attr(target_family = "windows", path = "windows.rs")]
mod implementation;

/// Gets the amount of threads for the current process.
/// Returns `None` if there are no threads.
#[must_use]
pub fn thread_amount() -> Option<NonZeroUsize> {
    implementation::thread_amount()
}

/// Check if the current process is single-threaded.
#[must_use]
pub fn is_single_threaded() -> bool {
    match thread_amount() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[track_caller]
    fn wait_for_count_to_stabilize(expected: usize) {
        let mut current = 0;

        // Poll for up to 2.5 seconds
        for _ in 0..50 {
            current = thread_amount().map_or(0, NonZeroUsize::get);
            if current == expected {
                return;
            }

            thread::sleep(Duration::from_millis(50));
        }

        panic!(
            "Timed out waiting for thread count to stabilize at {expected}. Last count: {current}"
        );
    }

    mod thread_amount_tests {
        use super::*;

        #[test]
        fn spawn_increases_count() {
            let initial = thread_amount().unwrap().get();
            let barrier = Arc::new(Barrier::new(2));
            let c_barrier = barrier.clone();

            let handle = thread::spawn(move || {
                c_barrier.wait(); // Wait for main thread to check
                c_barrier.wait(); // Wait for main thread to release
            });

            barrier.wait(); // Wait for spawned thread to be active
            let new_count = thread_amount().unwrap().get();
            assert_eq!(new_count, initial + 1);

            barrier.wait();
            handle.join().unwrap();

            // Ensure count returns to baseline
            wait_for_count_to_stabilize(initial);
        }

        #[test]
        fn many_threads_simultaneously() {
            let initial = thread_amount().unwrap().get();
            let num_threads = 5;
            let barrier = Arc::new(Barrier::new(num_threads + 1));
            let mut handles = Vec::new();

            for _ in 0..num_threads {
                let c_barrier = barrier.clone();
                handles.push(thread::spawn(move || {
                    c_barrier.wait(); // Sync start
                    c_barrier.wait(); // Sync end
                }));
            }

            barrier.wait(); // All threads are now running
            wait_for_count_to_stabilize(initial + num_threads);

            barrier.wait(); // Release all threads
            for handle in handles {
                handle.join().unwrap();
            }

            wait_for_count_to_stabilize(initial);
        }

        #[test]
        fn nested_spawning() {
            let initial = thread_amount().unwrap().get();
            let barrier = Arc::new(Barrier::new(2));
            let b_clone = barrier.clone();

            let h1 = thread::spawn(move || {
                // Thread 1 spawns Thread 2
                let nested_h = thread::spawn(move || {
                    b_clone.wait(); // Wait A: active
                    b_clone.wait(); // Wait B: exit
                });

                nested_h.join().unwrap();
            });

            barrier.wait(); // Wait A: Both threads should be active
            let expected = initial + 2; // Main + h1 + nested_h
            wait_for_count_to_stabilize(expected);

            barrier.wait(); // Release and cleanup
            h1.join().unwrap();

            wait_for_count_to_stabilize(initial);
        }

        #[test]

        fn count_decreases_after_join() {
            let initial = thread_amount().unwrap().get();
            let h = thread::spawn(|| thread::sleep(Duration::from_millis(50)));

            // Wait for it to be running
            wait_for_count_to_stabilize(initial + 1);

            h.join().unwrap();

            // Ensure it goes back down
            wait_for_count_to_stabilize(initial);
        }

        #[test]

        fn rapid_churn() {
            let initial = thread_amount().unwrap().get();
            for _ in 0..50 {
                thread::spawn(|| {}).join().unwrap();
            }
            wait_for_count_to_stabilize(initial);
        }

        #[test]
        fn named_threads_are_counted() {
            let initial = thread_amount().unwrap().get();
            let barrier = Arc::new(Barrier::new(2));
            let c_barrier = barrier.clone();

            let h = thread::Builder::new()
                .name("test-worker".into())
                .spawn(move || {
                    c_barrier.wait();
                    c_barrier.wait();
                })
                .unwrap();

            barrier.wait();
            wait_for_count_to_stabilize(initial + 1);

            barrier.wait();
            h.join().unwrap();
            wait_for_count_to_stabilize(initial);
        }

        #[test]
        fn panicking_thread_decrements_count() {
            let initial = thread_amount().unwrap().get();
            let h = thread::spawn(|| panic!("Intentional panic for testing"));
            let _ = h.join();

            wait_for_count_to_stabilize(initial);
        }
    }

    mod is_single_threaded_tests {
        use super::*;

        #[test]
        fn lifecycle_is_relative() {
            // Establish baseline for THIS test run
            let initial = thread_amount().unwrap().get();

            // Only test the `true` case if we start at 1
            if initial == 1 {
                assert!(is_single_threaded(), "Should be true when count is 1");
            }

            let barrier = Arc::new(Barrier::new(2));
            let c_barrier = barrier.clone();

            let h = thread::spawn(move || {
                c_barrier.wait(); // Sync 1: Alive
                c_barrier.wait(); // Sync 2: Ready to exit
            });

            barrier.wait(); // Wait for new thread to be definitely active

            // Count MUST be higher now
            wait_for_count_to_stabilize(initial + 1);
            assert!(!is_single_threaded(), "Cannot be single-threaded with active child");

            // Finish child thread
            barrier.wait();
            h.join().unwrap();

            // Wait for count to return to original baseline
            wait_for_count_to_stabilize(initial);

            if initial == 1 {
                assert!(is_single_threaded(), "Should return to true");
            }
        }
    }
}
