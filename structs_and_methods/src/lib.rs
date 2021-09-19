pub mod lib {
    use std::f64::consts::PI;
    use num_traits::Num;

    #[derive(Clone, Debug, Hash, Eq)]
    pub struct Point<T: Num> {
        pub x: T,
        pub y: T,
    }

    impl<T: Num> PartialEq for Point<T> {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl<T: Num> Default for Point<T> {
        fn default() -> Self {
            Point { x: T::zero(), y: T::zero() }
        }
    }

    #[derive(Clone, Debug, Hash, Eq)]
    pub struct Dimension<T: Num> {
        pub w: T,
        pub h: T,
    }

    impl<T: Num> PartialEq for Dimension<T> {
        fn eq(&self, other: &Self) -> bool {
            self.w == other.w && self.h == other.h
        }
    }

    #[derive(Clone, Debug, Hash, Eq)]
    pub struct Rectangle<T: Num> {
        pub location: Point<T>,
        pub dimension: Dimension<T>,
    }

    #[derive(Clone, Debug, Hash, Eq)]
    pub struct Circle<T: Num> {
        pub location: Point<T>,
        pub r: T,
    }

    #[derive(Clone, Debug, Hash, Eq)]
    pub enum Figure<T: Num> {
        Rectangle(Rectangle<T>),
        Circle(Circle<T>),
    }

    impl<T: Num + Copy + PartialOrd + Into<f64>> Circle<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            let mut xx = self.location.x - p.x;
            let mut yy = self.location.y - p.y;
            xx = xx * xx;
            yy = yy * yy;
            (xx + yy) < self.r * self.r
        }
        pub fn area(&self) -> f64 {
            PI * (self.r * self.r).into()
        }
    }

    impl<T: Num> PartialEq for Circle<T> {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location && self.r == other.r
        }
    }

    impl<T: Num> Default for Circle<T> {
        fn default() -> Self {
            Circle { location: Point::default(), r: T::one() }
        }
    }

    impl<T: Num + Copy + PartialOrd + Into<f64>> Rectangle<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            (self.location.x + self.dimension.w >= p.x)
                && (self.location.y + self.dimension.h >= p.y)
                && (self.location.x <= p.x)
                && (self.location.y <= p.y)
        }
        pub fn area(&self) -> f64 {
            (self.dimension.w * self.dimension.h).into()
        }
    }

    impl<T: Num> PartialEq for Rectangle<T> {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location && self.dimension == other.dimension
        }
    }

    impl<T: Num> Default for Rectangle<T> {
        fn default() -> Self {
            Rectangle { location: Point::default(), dimension: Dimension { w: T::one(), h: T::one() } }
        }
    }

    impl<T: Num + Copy + PartialOrd + Into<f64>> Figure<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
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

    impl<T: Num> PartialEq for Figure<T> {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Figure::Rectangle(rect) => {
                    match other {
                        Figure::Rectangle(r) => { rect == r }
                        Figure::Circle(_) => { false }
                    }
                }
                Figure::Circle(circle) => {
                    match other {
                        Figure::Rectangle(_) => { false }
                        Figure::Circle(c) => { circle == c }
                    }
                }
            }
        }
    }

    impl<T: Num> Default for Figure<T> {
        fn default() -> Self {
            Figure::Circle(Circle::default())
        }
    }
}

