use std::collections::VecDeque;

struct RecentCounter {
    times: VecDeque<i32>
}

impl RecentCounter {
    fn new() -> Self {
        Self { times: VecDeque::new() }
    }

    fn ping(&mut self, time: i32) -> i32 {
        self.times.push_back(time);

        loop {
            if let Some(prev) = self.times.front() {
                if time - 3000 > *prev {
                    self.times.pop_front();
                } else {
                    break;
                }
            }
        }

        self.times.len() as i32
    }
}
