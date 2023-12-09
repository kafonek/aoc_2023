#[derive(Debug)]
pub struct Race {
    duration: usize,
    distance_to_beat: usize,
    pub answer: u32,
}

impl Race {
    pub fn new(duration: usize, distance_to_beat: usize) -> Self {
        Self {
            duration,
            distance_to_beat,
            answer: 0,
        }
    }

    pub fn run(&mut self) {
        for i in 1..=self.duration {
            let run_time = self.duration - i;
            let distance = i * run_time;
            if distance > self.distance_to_beat {
                self.answer += 1;
            }
        }
    }
}
