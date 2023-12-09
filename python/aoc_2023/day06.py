from dataclasses import dataclass


@dataclass
class Race:
    time: int
    distance_to_beat: int
    answer: int = 0

    def run(self):
        for i in range(self.time):
            run_time = self.time - i
            distance = i * run_time
            if distance > self.distance_to_beat:
                self.answer += 1
