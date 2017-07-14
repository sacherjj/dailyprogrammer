use std::str::FromStr;
use std::io::Read;
use std::f32;

const BOX_MIN: f32 = 0.0;
const BOX_MAX: f32 = 1.0;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    r: f32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(' ');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        match coords.next() {
            None    => Ok(Point { x, y }),
            Some(_) => Err(()),
        }
    }
}


pub fn nearly_equal(a: f32, b: f32) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // Handle infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
		// One of a or b is zero (or both are extremely close to it,) use absolute error.
		diff < (f32::EPSILON * f32::MIN_POSITIVE)
	} else { // Use relative error.  Had to added *2. on EPSILON, as it didn't match.
		(diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON*2.
	}
}

pub fn float_less(a: f32, b: f32) -> bool {
    if nearly_equal(a, b) {
        false
    } else {
        a < b
    }
}

impl Circle {

    pub fn new(center: Point, radius: f32) -> Circle {
        Circle{center: center, r: radius}
    }

    pub fn update(&mut self, cir: &Circle) {
        self.center.x = cir.center.x;
        self.center.y = cir.center.y;
        self.r = cir.r;
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

        let x = - b / (2. * a);
        let y = - c / (2. * a);
        let r = ((b*b + c*c - 4.*a*d) / (4.*a*a)).sqrt();

        Circle{center: Point{x: x, y: y}, r: r}
    }

    /// Returns a Circle based on two points (segment is diameter)
    pub fn from_two_points(j: &Point, k: &Point) -> Circle {
        let c_x = (j.x - k.x)/2. + k.x;
        let c_y = (j.y - k.y)/2. + k.y;
        let r = ((j.x - k.x)*(j.x - k.x) + (j.y - k.y)*(j.y - k.y)).sqrt() / 2.;
        Circle::new(Point{x: c_x, y: c_y}, r)
    }

    /// Check if the circle is contained in a square from 0.0 to 1.0 (x and y)
    pub fn in_square(&self) -> bool {
        let r = self.r;
        if float_less((BOX_MAX - BOX_MIN)/2., r) {
            return false;
        }
        let Point { x, y } = self.center;
        if float_less(x - r, BOX_MIN) ||
            float_less(BOX_MAX, x + r) {
            return false;
        }
        if float_less(y - r, BOX_MIN) ||
            float_less(BOX_MAX, y + r) {
            return false;
        }
        true
    }

    /// Check if point is inside circle
    /// Using pythagorean, but squaring radius instead of sqrt.
    pub fn is_point_inside(&self, p: &Point) -> bool {
        let ref c = self.center;
        let d_sq = (p.x - c.x)*(p.x - c.x) + (p.y - c.y)*(p.y - c.y);
        let r_sq = self.r*self.r;
        if nearly_equal(d_sq, r_sq) {
            true
        } else {
            d_sq < r_sq
        }
    }

    /// Counts how many points are inside circle
    pub fn points_inside_count(&self, points: &Vec<Point>) -> i32 {
        let mut count: i32 = 0;
        for point in points {
            if self.is_point_inside(point) {
                count += 1;
            }
        }
        count
    }
}

/// If new_cir is smaller than cur_cir, update cur_cir to values of new_cir
fn set_smallest_circle(cur_cir: &mut Circle, new_cir: &Circle, pts: &Vec<Point>) {
    let half_len = (pts.len() as i32) / 2;
    if new_cir.in_square() {
        let points_in = new_cir.points_inside_count(&pts);
        if points_in == half_len {
            if new_cir.r < cur_cir.r {
                cur_cir.update(&new_cir);
            }
        }
    }
}

fn calc_smallest_circle(pts: &Vec<Point>) -> Option<Circle> {
    let length = pts.len();
    let mut smallest = Circle::new(Point{x: 0.5, y:0.5}, 1.0);

    // Test solution for 3 points
    for pta_ind in 0 .. length - 2 {
        for ptb_ind in pta_ind + 1 .. length - 1 {
            for ptc_ind in ptb_ind + 1 .. length {
                let cir = Circle::from_three_points(&pts[pta_ind],
                                                    &pts[ptb_ind],
                                                    &pts[ptc_ind]);
                set_smallest_circle(&mut smallest, &cir, &pts);
            }
        }
    }

    // Test for solution with 2 points
    for pta_ind in 0..length - 1 {
        for ptb_ind in pta_ind + 1..length {
            let cir = Circle::from_two_points(&pts[pta_ind],
                                              &pts[ptb_ind]);
            set_smallest_circle(&mut smallest, &cir, &pts);
        }
    }

    if nearly_equal(smallest.r, 1.) {
        None
    } else {
        Some(smallest)
    }
}


fn get_test_data(filename: &str) -> Vec<Vec<Point>> {
    let buf = {
        let mut buf = String::new();
        std::fs::File::open(filename).unwrap()
            .read_to_string(&mut buf).unwrap();
        buf
    };
    let mut lines = buf.lines();
    let mut point_sets: Vec<Vec<Point>> = Vec::new();
    while let Some(line) = lines.next() {
        let n = line.parse::<u32>().unwrap();
        let points = (0..n).map(|_| lines.next().expect("Point is missing").parse().unwrap()).collect();
        point_sets.push(points);
    }
    point_sets
}


fn main() {
    let test_sets = get_test_data("../test1.txt");
    for test_set in test_sets {
        let result = calc_smallest_circle(&test_set);
        match result {
            // Solution Found for circle splitter
            Some(c) => println!("Solution: {:?}", c),
            // No solution exists
            None => println!("No Solution")
        }
    }
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
