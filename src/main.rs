#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    infinity: bool,  // Flag to indicate if the point is the point at infinity
}

impl Point {
    fn is_at_infinity(&self) -> bool {
        self.infinity
    }

    fn at_infinity() -> Self {
        Point { x: 0, y: 0, infinity: true }
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, m, a % m);
    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        std::mem::swap(&mut t, &mut new_t);
        r = r - quotient * new_r;
        std::mem::swap(&mut r, &mut new_r);
    }
    if t < 0 { t += m; }
    t
}

fn elliptic_add(p: &Point, q: &Point, a: i64, m: i64) -> Point {
    if p.is_at_infinity() {
        return q.clone();
    }
    if q.is_at_infinity() {
        return p.clone();
    }
    if p.x == q.x && (p.y != q.y || p.y == 0) {
        return Point::at_infinity();
    }

    let (x1, y1) = (p.x, p.y);
    let (x2, y2) = (q.x, q.y);

    let lambda = if x1 == x2 && y1 == y2 {
        (3 * x1 * x1 + a) * mod_inv(2 * y1, m) % m
    } else {
        (y2 - y1) * mod_inv(x2 - x1, m) % m
    };

    let x3 = (lambda * lambda - x1 - x2) % m;
    let y3 = (lambda * (x1 - x3) - y1) % m;

    Point {
        x: if x3 < 0 { x3 + m } else { x3 },
        y: if y3 < 0 { y3 + m } else { y3 },
        infinity: false,
    }
}

fn scalar_mult(n: i64, p: &Point, a: i64, m: i64) -> Point {
    let mut result = Point::at_infinity();
    let mut addend = p.clone();
    let mut n = n;

    while n > 0 {
        if n & 1 == 1 {
            result = elliptic_add(&result, &addend, a, m);
        }
        addend = elliptic_add(&addend, &addend, a, m);
        n >>= 1;
    }

    result
}

fn is_point_on_curve(x: i64, y: i64, a: i64, b: i64, m: i64) -> bool {
    (y * y - (x * x * x + a * x + b)).rem_euclid(m) == 0
}

fn find_all_points(a: i64, b: i64, m: i64) -> Vec<Point> {
    let mut points = Vec::new();
    for x in 0..m {
        for y in 0..m {
            if is_point_on_curve(x, y, a, b, m) {
                points.push(Point { x, y, infinity: false });
            }
        }
    }
    points.push(Point::at_infinity());  // Include the point at infinity
    points
}

fn find_generators(a: i64, b: i64, m: i64) -> Vec<Point> {
    let points = find_all_points(a, b, m);
    let total_points = points.len() as i64; // Convert usize to i64
    let mut generators = Vec::new();

    for point in &points {
        let mut current = point.clone();
        let mut order = 1;

        while !current.is_at_infinity() {
            current = scalar_mult(order, point, a, m);
            order += 1;
        }

        if order == total_points + 1 { // Adjust for the last increment
            generators.push(point.clone());
        }

        // Debugging output
        println!("Point ({}, {}): Order = {}", point.x, point.y, order - 1);
    }

    generators
}

fn main() {
    let a = 4;
    let b = 4;
    let m = 7;

    let points = find_all_points(a, b, m);
    println!("All points on the curve:");
    for point in &points {
        if point.is_at_infinity() {
            println!("Point at infinity");
        } else {
            println!("({}, {})", point.x, point.y);
        }
    }

    let generators = find_generators(a, b, m);
    println!("\nGenerators:");
    for gen in generators {
        if gen.is_at_infinity() {
            println!("Point at infinity");
        } else {
            println!("({}, {})", gen.x, gen.y);
        }
    }
}

