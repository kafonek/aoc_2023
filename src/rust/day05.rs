#[derive(Debug)]
pub struct Range {
    src_start: usize,
    src_end: usize,
    dst_start: usize,
}

impl Range {
    pub fn new(src_start: usize, src_end: usize, dst_start: usize) -> Range {
        Range {
            src_start,
            src_end,
            dst_start,
        }
    }

    pub fn get(&self, i: usize) -> Option<usize> {
        if i >= self.src_start && i < self.src_end {
            let relative_offset = i - self.src_start;
            Some(self.dst_start + relative_offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Mapping {
    name: String,
    ranges: Vec<Range>,
}

impl Mapping {
    pub fn new(name: String) -> Mapping {
        Mapping {
            name,
            ranges: Vec::new(),
        }
    }

    pub fn update(&mut self, s: &str) {
        let parts: Vec<usize> = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let (dst, src, rng) = (parts[0], parts[1], parts[2]);
        let r = Range::new(src, src + rng, dst);
        self.ranges.push(r);
    }

    pub fn get(&self, i: usize) -> usize {
        for r in &self.ranges {
            if let Some(new) = r.get(i) {
                return new;
            }
        }
        i
    }
}

#[derive(Debug)]
pub struct Pipeline {
    maps: Vec<Mapping>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline { maps: Vec::new() }
    }

    pub fn from_lines(lines: Vec<String>) -> Pipeline {
        let mut pipeline = Pipeline::new();
        let mut current_map: Option<Mapping> = None;

        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line.ends_with("map:") {
                if let Some(map) = current_map {
                    pipeline.maps.push(map);
                }
                let name = trimmed_line.split_whitespace().next().unwrap().to_string();
                current_map = Some(Mapping::new(name));
            } else if trimmed_line.is_empty() {
                if let Some(map) = current_map.take() {
                    pipeline.maps.push(map);
                }
            } else {
                if let Some(ref mut map) = current_map {
                    map.update(trimmed_line);
                } else {
                    panic!(
                        "Got a parseable line but no Mapping obj to update: {}",
                        trimmed_line
                    );
                }
            }
        }

        if let Some(map) = current_map {
            pipeline.maps.push(map);
        }

        pipeline
    }

    pub fn get(&self, i: usize, debug: bool) -> usize {
        let mut new_i = i;
        for map in &self.maps {
            new_i = map.get(new_i);
            if debug {
                println!("{} -> {} -> {}", i, map.name, new_i);
            }
        }
        new_i
    }
}
