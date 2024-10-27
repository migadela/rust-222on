use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    a: Point, // Ліва верхня точка
    b: Point, // Права нижня точка
}

impl Rectangle {
    // Метод для отримання всіх точок, які покриває прямокутник
    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for x in self.a.x..self.b.x {
            for y in self.a.y..self.b.y {
                points.insert(Point { x, y });
            }
        }
        points
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut all_points = HashSet::new();
    for rect in xs {
        all_points.extend(rect.points());
    }
    all_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
