#[derive(Default, Debug, Clone, Copy)]
pub enum Error {
    #[default]
    Any,
}

impl Error {
    pub fn new() -> Self {
        Self::Any
    }
}
