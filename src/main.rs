extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rand::prelude::*;
use std::fs::{File};
use std::io::{BufWriter, Write};
use std::ops::Add;

#[derive(Clone, Copy, Serialize)]
struct Point (u32, u32, u32);

#[derive(Clone, Copy, Serialize)]
struct FPoint (f64, f64, f64);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs : Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

struct RandomPoint(SmallRng);

impl RandomPoint {
    pub fn new() -> Self {
        let rng_seed = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        RandomPoint(SmallRng::from_seed(rng_seed))
    }

    pub fn next(&mut self) -> Point {
        // return point that is never larger than (7, 7, 7)
        Point(self.0.next_u32() & 7, self.0.next_u32() & 7, self.0.next_u32() & 7)
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let f = File::create("data.txt")?;
    let mut writer = BufWriter::with_capacity(32 * 2usize.pow(20), f); // 32MB buffer
    let origin = FPoint(0f64, 0f64, 0f64); // starting point for every point
    let n = 10; // number of point in each record
    let mut points = vec![origin; 10];
    let len = 10usize.pow(5); // need 100,000 records
    // let mut rdm = RandomPoint::new();

    // total size is 10^6 * (10 * 32^3) = 31,250MB
    (0..len).for_each(|r| {
        // each record
        (0..n).for_each(|i| {
            // each point

            // randomly move point
            // points[i] = points[i] + rdm.next();

            // calculate sin wave
            let coor = f64::sin((r + i) as f64 / std::f64::consts::PI);
            points[i] = FPoint(coor, coor, coor)
        });
        let p_str : &str = &serde_json::to_string(&points).unwrap();
        writer.write(p_str.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
    });

    Ok(())
}
