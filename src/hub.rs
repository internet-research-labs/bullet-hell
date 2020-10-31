use std::convert::TryInto;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
};

pub struct Hub {
    curr: AtomicUsize
}

impl Hub {
    pub fn new() -> Hub {
        Hub{
            curr: AtomicUsize::new(1),
        }
    }

    pub fn uuid(&self) -> i64 {
        self.curr.fetch_add(1, Ordering::Relaxed)
            .try_into().unwrap()
    }
}

impl Clone for Hub {
    fn clone(&self) -> Self {
        let ret = Hub::new();
        let i = self.curr.load(Ordering::Relaxed);
        ret.curr.store(i, Ordering::Relaxed);
        ret
    }
}
