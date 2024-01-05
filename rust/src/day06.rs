#[derive(Debug)]
pub struct Race {
    time: usize,
    distance_to_beat: usize,
}

impl Race {
    pub fn new(time: usize, distance_to_beat: usize) -> Self {
        Self {
            time,
            distance_to_beat,
        }
    }

    pub fn run(&self) -> usize {
        let mut answer = 0;
        for i in 1..=self.time {
            let run_time = self.time - i;
            let distance = i * run_time;
            if distance > self.distance_to_beat {
                answer += 1;
            }
        }
        answer
    }
}
