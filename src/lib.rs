mod auto;
pub use auto::*;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub(crate) use assert_initialized_main_thread;
pub(crate) use skip_assert_initialized;