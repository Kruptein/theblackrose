use std::collections::VecDeque;

// todo fix the nano second part :4
pub fn millis_to_chrono(millis: i64) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(millis / 1000, 0)
}

pub struct SlidingWindow {
    window: VecDeque<String>,
    last_len: usize,
}

impl SlidingWindow {
    pub fn new() -> SlidingWindow {
        SlidingWindow {
            window: VecDeque::with_capacity(5),
            last_len: 0,
        }
    }

    pub fn push(&mut self, format: String) -> () {
        let len = self.window.len();
        if len == 5 {
            self.window.pop_front();
        }

        self.window.push_back(format);

        self.print();
    }

    pub fn replace(&mut self, format: String) -> () {
        self.window.pop_back();
        self.window.push_back(format);
        self.print();
    }

    pub fn clear(&mut self) -> () {
        self.window.clear();
        self.last_len = 0;
    }

    fn print(&mut self) -> () {
        if self.last_len > 0 {
            print!("\x1B[{}A\x1B[J", self.last_len);
        }
        println!("=============================");
        for ln in &self.window {
            println!("{}", ln);
        }
        println!("=============================");
        self.last_len = self.window.len() + 2;
    }
}
