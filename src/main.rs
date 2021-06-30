
// use std::array;
// use std::{iter, time::{Duration, Instant}};
// use plotters::prelude::*;

// use std::time::Instant;

use core::f64;
// use std::iter::empty;
// use std::cmp::max;
use rand::Rng;

fn main() {

    let iters = 100_000;
    // let iters = 100;
    let dos = 100;

    let digits_of_interest: usize = 5;

    // let mut x_vec = Vec::new();
    // let mut y_vec = Vec::new();
    // let mut r_vec = Vec::new();

    let mut pi_vec = Vec::new();
    let mut steps_vec = Vec::new();
    
    let pi_data: (Vec<usize>, usize, usize) = start(digits_of_interest);

    println!("decimal places = {}", pi_data.2);

    // let pi_final = looper(pi_data.1, pi_data.2 as isize, digits_of_interest, pi_data.0, iters);

    // println!("Pi ~ {}", pi_final.0)

    for _ in 0..=dos {
        let pi_output = looper(pi_data.1, pi_data.2 as isize, digits_of_interest, pi_data.0.to_owned(), iters);

        pi_vec.push(pi_output.0);
        steps_vec.push(pi_output.2);

    }

    for (index, value) in pi_vec.iter().enumerate() {
        println!("Pi ~ {0}, Steps = {1}", value, steps_vec[index])
    }

}

struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }
        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl  Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n /self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
    
}

fn start(input_digits: usize) -> (Vec<usize>, usize, usize) {
        let pi_engineering: f64 = 3.141592653589793;

        let pi_abs: usize = 3141592653589793;

        let piabs_digits: Vec<_> = Digits::new(pi_abs).collect();
    
        // let digitsofint: usize = input_digits;
        let digitsofint: usize;
        let mut pi_vect: Vec<usize> = Vec::new();    
        let mut zeros: usize = 1;
        let mut pi_tar: usize = 0;

        if input_digits > 15 {
            digitsofint = 15;
            println!("Truncating to {}, can only work with pi up to 15 decimal places, sorry.", pi_engineering)
        } else {
            digitsofint = input_digits
        }

        for index in 0..=digitsofint {
            pi_vect.push(piabs_digits[index]);
            zeros *= 10;
        }
    
        let decimal = zeros;

        for index in 0..pi_vect.len() {
            pi_tar = pi_tar + (pi_vect[index] * zeros);
            zeros /= 10;
        }
    
        // let pi_tar: usize = pi_vect.iter().sum();
        println!("{}", pi_tar);
        // println!("{}", pi_tar / (decimal / 100));
        
        return (pi_vect, pi_tar, decimal)
}

fn point(decimal_size: isize) -> (f64, f64) {

    let x_ran = rand::thread_rng().gen_range(-decimal_size..decimal_size) as f64;
    let y_ran = rand::thread_rng().gen_range(-decimal_size..decimal_size) as f64;

    let x = x_ran / (decimal_size as f64);
    let y = y_ran / (decimal_size as f64);

    return (x, y)

}

fn circuitbreaker(mut circuit: Vec<bool>, length: usize) -> Vec<bool> {
    for index in 0..=length {
        circuit[index] = false;
    }

    return circuit
}

fn too_many_steps(steps: usize, max_steps: usize) -> bool {
    if steps > max_steps {
        println!("ending script, steps = {}, max steps = {}", steps, max_steps);
        return true
    } else {
        false
    }
}

fn percent_errdiff(theoretical: f64, measured: f64) -> (f64, f64) {
    let diff = (theoretical - measured) / (0.5 * (theoretical + measured));
    let err = (theoretical - measured) / theoretical;

    return (diff, err)
}

fn looper(pi_target: usize, decimal_size: isize, digits_of_interest: usize, pi_digits: Vec<usize>, max_steps: usize) -> (f64, f64, f64) {

    let pi_tar = pi_target as f64 / decimal_size as f64;

    // println!("Looking for pi = {}", pi_tar);

    let mut inside: f64 = 0.0;
    let mut total: f64 = 0.0;

    let radius =|x: f64, y: f64| -> f64 {(x.powf(2.0) + y.powf(2.0)).sqrt()};
    let pi_approx = |i: f64, t: f64| -> f64 {4.0 * (i / t)};

    // let mut pi_bool = Vec::new();
    let mut pi_bool = empty_vec(digits_of_interest);
    let pi_val: f64;

    loop {
        let (x,y) = point(decimal_size as isize);
        // x_vec.push(x);
        // y_vec.push(y);
    
        let r = radius(x, y);
        // let r_size = (r * pi_data.2 as f64) as usize;
    
        if r <= 1.0 {
            inside += 1.0;
        }
    
        total += 1.0;

        // let pi_nodec = (pi_approx(inside, total) * (decimal_size / 10) as f64) as usize;
        let pi_nodec = (pi_approx(inside, total) * decimal_size as f64) as usize;

        let pinew_digits: Vec<_> = Digits::new(pi_nodec).collect();
        
        // r_vec.push(radius(x, y, pi_data.2));

        // println!("#### Starting comparison ####");
        for (index, _) in pi_digits.iter().enumerate() {
            // println!("{0}, {1}", pi_target[index], pinew_digits[index]);
            if pinew_digits.len() >= pi_digits.len() {
                if pi_digits[index] == pinew_digits[index] {
                    pi_bool[index] = true;
                }    
            // } else {
            //     println!("pinew_digits is to small")
            }
        }

        if pi_bool.iter().all(|&x| x==true) {
            pi_val = pi_approx(inside, total);

            let per_errdiff = percent_errdiff(pi_tar, pi_val as f64);
            // println!("Found PI to be {}", pi_val);
            break (pi_val, per_errdiff.1, total)
        } else {
            pi_bool = circuitbreaker(pi_bool, digits_of_interest)
        }

        if too_many_steps(total as usize, max_steps) {
            pi_val = pi_approx(inside, total);
            // println!("{:?}", pi_nodec);
            // println!("{:?}", pi_bool);
            let per_errdiff = percent_errdiff(pi_tar, pi_val as f64);
            break (pi_val, per_errdiff.1, total)
            // break pi_val
        }

    }
}

fn empty_vec(length: usize) -> Vec<bool> {
    let mut output_vec = Vec::new();
    for _ in 0..=length {
        output_vec.push(false);
    }

    return output_vec
}

// fn def_loop(decimal_size: isize, max_steps: usize) {
//     // let mut steps: usize;
//     let mut inside: f64 = 0.0;
//     let mut total: f64 = 0.0;

//     let radius =|x: f64, y: f64| -> f64 {(x.powf(2.0) + y.powf(2.0)).sqrt()};

//     for _ in 0..=max_steps{
//         let (x,y) = point(decimal_size);
//         // x_vec.push(x);
//         // y_vec.push(y);
    
//         let r = radius(x, y);
//         // let r_size = (r * pi_data.2 as f64) as usize;
    
//         if r <= 1.0 {
//             // println!("hit!");
//             inside += 1.0;
//         }
    
//         total += 1.0;
    
//         // r_vec.push(radius(x, y, pi_data.2));
//     }

//     println!("Pi ~ {}", 3.14)

// }