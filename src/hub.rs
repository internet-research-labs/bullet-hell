// use std::convert::TryInto;

#[derive(Default)]
pub struct Hub {
    curr: i64,
}

impl Hub {
    pub fn new() -> Hub {
        Hub{
            // curr: AtomicUsize::new(1),
            curr: 1,
        }
    }

    pub fn uuid(&mut self) -> i64 {
        self.curr += 1;
        self.curr
        // self.curr.fetch_add(1, Ordering::Relaxed)
        //    .try_into().unwrap()
    }
}
