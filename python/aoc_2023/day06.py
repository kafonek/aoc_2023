from dataclasses import dataclass


@dataclass
class Race:
    time: int
    distance_to_beat: int

    def run(self) -> int:
        answer = 0
        for i in range(self.time):
            run_time = self.time - i
            distance = i * run_time
            if distance > self.distance_to_beat:
                answer += 1
        return answer
