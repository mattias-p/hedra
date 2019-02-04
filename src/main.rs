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

struct Pentagon {
    points: [Point; 5],
}

impl Pentagon {
    fn flip_rotate(&self) -> Pentagon {
        let [p0, p1, p2, p3, p4] = self.points;
        let p2_rot = p1 - (p2 - p0);
        let p3_rot = p1 - (p3 - p0);
        let p4_rot = p1 - (p4 - p0);
        Pentagon {
            points: [p0, p1, p2_rot, p3_rot, p4_rot],
        }
    }
    fn flip_reflect(&self) -> Pentagon {
        let [p0, p1, p2, p3, p4] = self.points;
        let axis = p1 - p0;
        let v2 = p2 - p0;
        let v3 = p3 - p0;
        let v4 = p4 - p0;
        let p2_compl = p0 - 2.0 * (v2 - v2.onto(axis));
        let p3_compl = p0 - 2.0 * (v3 - v3.onto(axis));
        let p4_compl = p0 - 2.0 * (v4 - v4.onto(axis));
        Pentagon {
            points: [p0, p1, p2_compl, p3_compl, p4_compl],
        }
    }
    fn rotate(&self) -> Pentagon {
        let [p0, p1, p2, p3, p4] = self.points;
        let t1 = Matrix::translate(Point::default() - p1);
        let r = Matrix::rotate_scale((-(p1 - p0) * (p2 - p1)).unit());
        let t0 = Matrix::translate(p0 - Point::default());
        let t = t0 * r * t1;
        let t = &t;
        Pentagon {
            points: [t * p0, t * p1, t * p2, t * p3, t * p4],
        }
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
