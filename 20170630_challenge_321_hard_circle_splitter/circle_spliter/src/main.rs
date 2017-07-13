use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const BOX_MIN: f64 = 0.0;
const BOX_MAX: f64 = 1.0;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    r: f64,
}

impl Circle {

    pub fn new(center: Point, radius: f64) -> Circle {
        Circle{center: center, r: radius}
    }

    /// Returns a Circle that passes through the three points.
    pub fn from_three_points(j: &Point, k: &Point, l: &Point) -> Circle {
        let a = j.x * (k.y - l.y) -
                j.y * (k.x - l.x) +
                k.x * l.y -
                l.x * k.y;

        let b = (j.x*j.x + j.y*j.y) * (l.y - k.y) +
                (k.x*k.x + k.y*k.y) * (j.y - l.y) +
                (l.x*l.x + l.y*l.y) * (k.y - j.y);

        let c = (j.x*j.x + j.y*j.y) * (k.x - l.x) +
                (k.x*k.x + k.y*k.y) * (l.x - j.x) +
                (l.x*l.x + l.y*l.y) * (j.x - k.x);

        let d = (j.x*j.x + j.y*j.y) * (l.x*k.y - k.x*l.y) +
                (k.x*k.x + k.y*k.y) * (j.x*l.y - l.x*j.y) +
                (l.x*l.x + l.y*l.y) * (k.x*j.y - j.x*k.y);

        let x = - b / (2f64 * a);
        let y = - c / (2f64 * a);
        let r = ((b*b + c*c - 4f64*a*d) / (4f64*a*a)).sqrt();

        Circle{center: Point{x: x, y: y}, r: r}
    }

    /// Returns a Circle based on two points (segment is diameter)
    pub fn from_two_points(j: &Point, k: &Point) -> Circle {
        let c_x = (j.x - k.x)/2f64 + k.x;
        let c_y = (j.y - k.y)/2f64 + k.y;
        let r = ((j.x - k.x)*(j.x - k.x) + (j.y - k.y)*(j.y - k.y)).sqrt() / 2f64;
        Circle{center: Point{x: c_x, y: c_y}, r: r}
    }

    /// Check if the circle is contained in a square from 0.0 to 1.0 (x and y)
    pub fn in_square(&self) -> bool {
        let r = self.r;
        if r > (BOX_MAX - BOX_MIN)/2f64 {
            false
        }
        let Point { x, y } = self.center;
        if (x - r) < BOX_MIN || (x + r) > BOX_MAX {
            false
        }
        if (y - r) < BOX_MIN || (y + r) > BOX_MAX {
            false
        }
        true
    }

    /// Check if point is inside circle
    /// Using pythagorean, but squaring radius instead of sqrt.
    pub fn point_inside(&self, p: &Point) -> bool {
        let ref c = self.center;
        let d_sq = (p.x - c.x)*(p.x - c.x) + (p.y - c.y)*(p.y - c.y);
        d_sq < self.r*self.r
    }
}


fn get_test_data(filename: &str) -> Vec<Vec<f64>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let line_sets: Vec<Vec<f64>> = reader.lines()
        .filter_map(
            |l| l.ok().map(
                |s| s.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()))
        .collect();
 /*   let mut point_set: Vec<Point> = Vec::new();
    let mut test_sets: Vec<Vec<Point>> = Vec::new();
    for set in &line_sets {
        if set.len() == 1 {
            if point_set.len() > 0 {
                test_sets.push(point_set);
                let mut point_set: Vec<Point> = Vec::new();
            }
        } else {
            point_set.push(Point{x: set[0], y: set[1]})
        }
    }*/
    line_sets
}


fn main() {
    let test_sets = get_test_data("../test1.txt");
    println!("{:?}", test_sets);
/*    let a = Point {x: 0.1, y: 0.1};
    let b = Point {x: 0.2, y: 0.2};
    let c = Point {x: 0.3, y: 0.3};
    let cira = Circle::from_three_points(&a, &b, &c);
    let cirb = Circle::from_two_points(&b, &c);

    println!("{:?}", cira);
    println!("In Square: {}", cira.in_square());
    let test = Point {x: 0.35, y: 0.35};
    println!("{:?} in Circle: {}", test, cira.point_inside(&test));
*/
}
