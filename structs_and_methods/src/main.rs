use std::f32::consts::PI;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Dimension {
    w: f64,
    h: f64,
}

#[derive(Clone)]
struct Rectangle {
    location: Point,
    dimension: Dimension,
}

#[derive(Clone)]
struct Circle {
    location: Point,
    r: f64,
}

#[derive(Clone)]
enum Figure {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        let mut xx = self.location.x - p.x;
        let mut yy = self.location.y - p.y;
        xx = xx * xx;
        yy = yy * yy;
        (xx + yy).sqrt() < self.r
    }
    fn area(&self) -> f64 {
        (PI as f64) * self.r * self.r
    }
}

impl Rectangle {
    fn contains(&self, p: &Point) -> bool {
        (self.location.x + self.dimension.w >= p.x)
            && (self.location.y + self.dimension.h >= p.y)
            && (self.location.x <= p.x)
            && (self.location.y <= p.y)
    }
    fn area(&self) -> f64 {
        self.dimension.w * self.dimension.h
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Rectangle(rect) => {
                rect.contains(p)
            }
            Figure::Circle(circle) => {
                circle.contains(p)
            }
        }
    }
}

fn main() {
    let c = Circle { location: Point { x: 1.0, y: 2.0 }, r: 3.0 };
    let r = Rectangle { location: Point { x: 1.0, y: 2.0 }, dimension: Dimension { w: 1.0, h: 2.0 } };
    let fr = Figure::Rectangle(r.clone());
    let fc = Figure::Circle(c.clone());
    println!("area for circle = {}", c.area());
    println!("area for rect = {}", r.area());

    let p = Point { x: -10.0, y: -2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };

    println!("{}", r.contains(&p));
    println!("{}", r.contains(&p2));
    println!("{}", c.contains(&p));
    println!("{}", c.contains(&p2));
    println!("{}", fr.contains(&p));
    println!("{}", fc.contains(&p2));
}
