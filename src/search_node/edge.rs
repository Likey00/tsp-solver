pub type Edge = (usize, usize);

pub trait Reverse {
    fn reverse(&self) -> Self;
}

impl Reverse for Edge {
    fn reverse(&self) -> Self {
        (self.1, self.0)
    }
}