pub use std::error::Error;

pub mod linked_list;
pub mod queue;
pub mod queue2;
pub mod stack;

// Should be implemented for <u32>.
pub trait Algorithm {
    // Commonly, should not panic.
    fn showcase() -> Result<(), Box<dyn Error>>;
}
