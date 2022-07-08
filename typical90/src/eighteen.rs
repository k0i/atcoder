use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
use std::cmp;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io;

pub fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).expect("error");
    let vec: Vec<&str> = buf.trim().split_whitespace().collect();
    let mut t: f64 = vec[0].parse().unwrap();
    // let mut s: Vec<char> = vec[0].chars().collect();

    buf.clear();
    io::stdin().read_line(&mut buf).expect("error");
    let vec: Vec<&str> = buf.trim().split_whitespace().collect();
    let mut l: f64 = vec[0].parse().unwrap();
    let mut x: f64 = vec[1].parse().unwrap();
    let mut y: f64 = vec[2].parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).expect("error");
    let vec: Vec<&str> = buf.trim().split_whitespace().collect();
    let mut q: usize = vec[0].parse().unwrap();

    let mut e: Vec<f64> = vec![];

    for i in 0..q {
        buf.clear();
        io::stdin().read_line(&mut buf).expect("error");
        let vec: Vec<&str> = buf.trim().split_whitespace().collect();
        e.push(vec[0].parse().unwrap());
    }

    // println!("t={} l={} x={} y={}", t, l, x, y);

    let tx: f64 = x;
    let ty: f64 = y;
    let tz: f64 = 0.0;

    for i in 0..q {
        let th = e[i] / t * 2.0 * std::f64::consts::PI;
        let ex: f64 = 0.0;
        let ey: f64 = -th.sin() * l / 2.0;
        let ez: f64 = -th.cos() * l / 2.0 + l / 2.0;

        let a: f64 = ((tx - ex) * (tx - ex) + (ty - ey) * (ty - ey)).sqrt();
        let deg: f64 = ez.atan2(a).to_degrees();
        println!("{}", deg);
    }
}
