use structs_and_methods::lib::{Point, Circle, Rectangle, Figure, Dimension};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

fn test_float() {
    let c = Circle { location: Point { x: 1.0, y: 2.0 }, r: 3.0 };
    let r = Rectangle { location: Point { x: 1.0, y: 2.0 }, dimension: Dimension { w: 1.0, h: 2.0 } };
    let fr = Figure::Rectangle(r.clone());
    let fc = Figure::Circle(c.clone());

    println!("area for circle = {}", c.area());
    println!("area for rect = {}", r.area());

    let p = Point { x: -10.0, y: -2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };

    println!("{:?} {}", r, r.contains(&p));
    println!("{:?} {}", r, r.contains(&p2));
    println!("{:?} {}", r, c.contains(&p));
    println!("{:?} {}", r, c.contains(&p2));
    println!("{:?} {}", r, fr.contains(&p));
    println!("{:?} {}", r, fc.contains(&p2));
}

fn test_int() {
    let c = Circle { location: Point { x: 1, y: 2 }, r: 3 };
    let r = Rectangle { location: Point { x: 1, y: 2 }, dimension: Dimension { w: 1, h: 2 } };
    let fr = Figure::Rectangle(r.clone());
    let fc = Figure::Circle(c.clone());
    println!("area for circle = {}", c.area());
    println!("area for rect = {}", r.area());

    let p = Point { x: -10, y: -2 };
    let p2 = Point { x: 1, y: 2 };

    println!("{:?} {}", r, r.contains(&p));
    println!("{:?} {}", r, r.contains(&p2));
    println!("{:?} {}", r, c.contains(&p));
    println!("{:?} {}", r, c.contains(&p2));
    println!("{:?} {}", r, fr.contains(&p));
    println!("{:?} {}", r, fc.contains(&p2));

    let mut hash_map = HashMap::new();
    hash_map.insert(p, "kek");
    hash_map.insert(p2, "kek2");
    println!("{:?}", hash_map)
}

fn main() {
    test_float();
    test_int();
}
