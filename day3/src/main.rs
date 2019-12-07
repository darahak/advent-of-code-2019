use input_utils::aoc;

fn main() {
    let input = aoc::get_input("src/input.txt");
    let paths: Vec<&str> = aoc::get_lines(&input);

    let segments_first = get_segments(paths[0]);
    let segments_second = get_segments(paths[1]);

    let mut closest: i32 = std::i32::MAX;

    for i in 0..segments_first.len() {
        for j in 0..segments_second.len() {
            let (intersects, intersection) =
                get_intersection(&segments_first[i], &segments_second[j]);

            if intersects {
                let distance = get_distance(&intersection);

                if distance < closest {
                    closest = distance;
                }
            }
        }
    }

    println!("Manhattan distance for closest intersection: {}", closest);
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn get_segments(path: &str) -> Vec<Segment> {
    let mut current_pos = Point { x: 0, y: 0 };
    let mut segments: Vec<Segment> = Vec::new();

    let commands: Vec<&str> = path.split(',').collect();

    for command in commands {
        let (direction, distance_str) = command.split_at(1);
        let distance: i32 = distance_str.parse().unwrap();
        let previous_pos = current_pos;

        match direction {
            "U" => current_pos.y += distance,
            "R" => current_pos.x += distance,
            "D" => current_pos.y -= distance,
            "L" => current_pos.x -= distance,
            _ => println!("Unknown direction: {} {}", direction, distance),
        }

        segments.push(Segment {
            start: previous_pos,
            end: current_pos,
        });
    }

    return segments;
}

fn get_intersection(a: &Segment, b: &Segment) -> (bool, Point) {
    use std::cmp;

    if a.is_vertical() && b.is_horizontal() {
        let ax = a.start.x;
        let by = b.start.y;

        let x_correct = cmp::min(b.start.x, b.end.x) < ax && ax < cmp::max(b.start.x, b.end.x);
        let y_correct = cmp::min(a.start.y, a.end.y) < by && by < cmp::max(a.start.y, a.end.y);

        if x_correct && y_correct {
            return (true, Point { x: ax, y: by });
        }
    } else {
        if b.is_vertical() {
            let bx = b.start.x;
            let ay = a.start.y;

            let x_correct = cmp::min(a.start.x, a.end.x) < bx && bx < cmp::max(a.start.x, a.end.x);
            let y_correct = cmp::min(b.start.y, b.end.y) < ay && ay < cmp::max(b.start.y, b.end.y);

            if x_correct && y_correct {
                return (true, Point { x: bx, y: ay });
            }
        }
    }

    return (false, Point { x: 0, y: 0 });
}

fn get_distance(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}
