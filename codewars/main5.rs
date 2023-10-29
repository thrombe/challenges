
fn main() {
    println!("{:?}", rectangle_rotation(6, 4));
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> 
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::MulAssign,
    T: Copy + Clone,
{
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

fn rectangle_rotation(mut a: i32, mut b: i32) -> i32 {
    // simplyfing calculation logic by considering a > b
    if a < b {
        let t = a;
        a = b;
        b = t;
    }
    let mut count = 0;

    // c1 and c2 are the rotated points on opp sides of origin ( (+, +) and (-, -) before rotation )
    let a = a as f64/2.0;
    let b = b as f64/2.0;
    let ir2 = std::f64::consts::FRAC_1_SQRT_2;
    let c1 = Point::new(ir2*(a-b),  ir2*(a+b));
    // let c2 = Point::new(-c1.x, -c1.y);

    // x-y is constant for points along side of rect with +ve slope
    // similar for x+y, -ve slpoe
    let sum = c1.x+c1.y;
    let sub = c1.x-c1.y;

    let sum = sum.abs().floor() as i64;
    let sub = sub.abs().floor() as i64;

    // points on the x axis
    count += sub*2 +1;

    // consider points from left to right on the x axis -> p1
    // then p2 -> point with the max possible y (or x) coordinate while having a msum of m
    for m in -sub..sub+1 {
        let p1 = Point::new(m, 0);
        // p2.x+p2.y = sum, p2.x-p2.y = m
        let p2 = Point::new((m+sum)/2, (m-sum)/2);
        // there are p2.x-p1.x (or y) no. of points in the rect where y > 0
        count += 2*(p2.x-p1.x); // twice cuz symmetry
    }
    count as i32
}