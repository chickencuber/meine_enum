pub use meine_enum_proc::*;
mod bitmask;
pub use bitmask::BitMask;


pub trait ToFlagIter: Copy {
    fn next(&mut self) -> Option<Self>;
}

pub struct FlagIter<T: ToFlagIter> {
    remaining: T,
}

impl<T: ToFlagIter> FlagIter<T> {
    pub fn new(remaining: T) -> Self {
        Self { remaining }
    }
}

impl<T: ToFlagIter> Iterator for FlagIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.remaining.next()      
    }
}


