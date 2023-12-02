#[derive(Debug)]
pub struct Bag {
    blue: i32,
    red: i32,
    green: i32,
}

impl Bag {
    pub fn new(blue: i32, red: i32, green: i32) -> Bag {
        Bag { blue, red, green }
    }

    pub fn from_string(s: &str) -> Result<Bag, &'static str> {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for part in s.split(',') {
            let parts: Vec<&str> = part.trim().split_whitespace().collect();
            if parts.len() != 2 {
                return Err("Invalid input format");
            }

            let count = parts[0].parse::<i32>().map_err(|_| "Invalid count")?;
            match parts[1] {
                "blue" => blue = count,
                "red" => red = count,
                "green" => green = count,
                _ => return Err("Unknown color"),
            }
        }

        Ok(Bag::new(blue, red, green))
    }

    // used in part 1 of the problem
    pub fn can_contain(&self, other: &Bag) -> bool {
        self.blue >= other.blue && self.red >= other.red && self.green >= other.green
    }

    // used in part 2 of the problem
    pub fn power(&self) -> i32 {
        self.blue * self.green * self.red
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    bags: Vec<Bag>,
}

impl Game {
    pub fn from_string(s: &str) -> Result<Game, &'static str> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid input format");
        }

        let id = parts[0]
            .split_whitespace()
            .last()
            .ok_or("Invalid ID")?
            .parse::<i32>()
            .map_err(|_| "ID is not a number")?;

        let bag_parts = parts[1].split(';');
        let mut bags = Vec::new();

        for part in bag_parts {
            match Bag::from_string(part) {
                Ok(bag) => bags.push(bag),
                Err(e) => return Err(e),
            }
        }

        Ok(Game { id, bags })
    }

    // used in part 1 of the problem
    pub fn check(&self, max_bag: &Bag) -> bool {
        self.bags.iter().all(|bag| max_bag.can_contain(bag))
    }

    // used in part 2 of the problem
    pub fn max_values(&self) -> Bag {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for bag in &self.bags {
            max_blue = max_blue.max(bag.blue);
            max_red = max_red.max(bag.red);
            max_green = max_green.max(bag.green);
        }

        Bag::new(max_blue, max_red, max_green)
    }
}
