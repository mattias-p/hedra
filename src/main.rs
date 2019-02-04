use std::borrow::Cow;

#[derive(Clone, Copy)]
struct Vect {
    x: f64,
    y: f64,
}

impl Vect {
    fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
    fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
    fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    fn unit(self) -> Self {
        if self.is_zero() {
            panic!("unit undefined for the zero vector");
        }
        self / self.norm()
    }
    fn onto(self, other: Self) -> Self {
        self.dot(other.unit()) * other
    }
}

impl std::ops::Neg for Vect {
    type Output = Vect;
    fn neg(self) -> Self::Output {
        Vect {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Mul<Vect> for f64 {
    type Output = Vect;
    fn mul(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul for Vect {
    type Output = Vect;
    fn mul(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.y,
        }
    }
}

impl std::ops::Div<f64> for Vect {
    type Output = Vect;
    fn div(self, rhs: f64) -> Self::Output {
        Vect {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Add for Vect {
    type Output = Vect;
    fn add(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vect {
    type Output = Vect;
    fn sub(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl std::ops::Add<Vect> for Point {
    type Output = Point;
    fn add(self, rhs: Vect) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Vect;
    fn sub(self, rhs: Self) -> Self::Output {
        Vect {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<Vect> for Point {
    type Output = Point;
    fn sub(self, rhs: Vect) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct PointIter<'a> {
    first: &'a [Point],
    second: &'a [Point],
}

impl<'a> Iterator for PointIter<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((point, rest)) = self.first.split_first() {
            self.first = rest;
            Some(*point)
        } else if let Some((point, rest)) = self.second.split_first() {
            self.second = rest;
            Some(*point)
        } else {
            None
        }
    }
}

struct Polygon<'a> {
    orientation: usize,
    points: Cow<'a, Vec<Point>>,
}

impl<'a> Polygon<'a> {
    fn new(points: Cow<'a, Vec<Point>>) -> Self {
        Polygon {
            orientation: 0,
            points,
        }
    }
    fn len(&self) -> usize {
        self.points.as_ref().len()
    }
    fn align(&self, change: isize) -> Self {
        Polygon {
            orientation: (self.orientation as isize + change) as usize,
            points: self.points.clone(),
        }
    }
    fn points(&self) -> PointIter {
        let (first, second) = self.points.as_ref().as_slice().split_at(self.orientation);
        PointIter {
            first, second
        }
    }
    fn flip_rotate(&self) -> Polygon {
        let mut points = self.points();
        let p0 = points.next().unwrap();
        let p1 = points.next().unwrap();

        let mut buf = Vec::with_capacity(self.len());
        buf.push(p0);
        buf.push(p1);
        for p in points {
            buf.push(p1 - (p - p0));
        }

        Polygon::new(Cow::Owned(buf))
    }
    fn flip_reflect(&self) -> Polygon {
        let mut points = self.points();
        let p0 = points.next().unwrap();
        let p1 = points.next().unwrap();
        let axis = p1 - p0;

        let mut buf = Vec::with_capacity(self.len());
        buf.push(p0);
        buf.push(p1);
        for p in points {
            let v = p - p0;
            let p_compl = p0 - 2.0 * (v - v.onto(axis));
            buf.push(p_compl);
        }

        Polygon::new(Cow::Owned(buf))
    }
    fn reorient(&self) -> Polygon {
        let mut points = self.points();
        let p0 = points.next().unwrap();
        let p1 = points.next().unwrap();
        let p2 = points.next().unwrap();
        let t1 = Matrix::translate(Point::default() - p1);
        let r = Matrix::rotate_scale((-(p1 - p0) * (p2 - p1)).unit());
        let t0 = Matrix::translate(p0 - Point::default());
        let t = t0 * r * t1;
        let t = &t;
        let mut buf = Vec::with_capacity(self.len());
        buf.push(t * p0);
        buf.push(t * p1);
        buf.push(t * p2);
        for p in points {
            buf.push(t * p);
        }

        Polygon::new(Cow::Owned(buf))
    }
}

struct Matrix {
    coords: [f64; 6],
}

impl Matrix {
    fn identity() -> Self {
        Matrix {
            coords: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }
    fn translate(v: Vect) -> Self {
        Matrix {
            coords: [1.0, 0.0, 0.0, 1.0, v.x, v.y],
        }
    }
    fn rotate_scale(v: Vect) -> Self {
        Matrix {
            coords: [v.x, v.y, -v.y, v.x, 0.0, 0.0],
        }
    }
    fn v11(&self) -> f64 {
        self.coords[0]
    }
    fn v21(&self) -> f64 {
        self.coords[1]
    }
    fn v31(&self) -> f64 {
        0.0
    }
    fn v12(&self) -> f64 {
        self.coords[2]
    }
    fn v22(&self) -> f64 {
        self.coords[3]
    }
    fn v32(&self) -> f64 {
        0.0
    }
    fn v13(&self) -> f64 {
        self.coords[4]
    }
    fn v23(&self) -> f64 {
        self.coords[6]
    }
    fn v33(&self) -> f64 {
        1.0
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let v11 = self.v11() * rhs.v11() + self.v21() * rhs.v12();
        let v21 = self.v11() * rhs.v21() + self.v21() * rhs.v22();
        let v12 = self.v12() * rhs.v11() + self.v22() * rhs.v12();
        let v22 = self.v12() * rhs.v21() + self.v22() * rhs.v22();
        let v13 = self.v13() + rhs.v13();
        let v23 = self.v23() + rhs.v23();
        Matrix {
            coords: [v11, v21, v12, v22, v13, v23],
        }
    }
}

impl<'a> std::ops::Mul<Point> for &'a Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let x = self.v11() * rhs.x + self.v12() * rhs.y + self.v13();
        let y = self.v21() * rhs.x + self.v22() * rhs.y + self.v23();
        Point { x, y }
    }
}

fn main() {
    println!("Hello, world!");
}
