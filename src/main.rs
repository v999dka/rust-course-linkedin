use std::{fs, io, env};
use rand::prelude::thread_rng;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height
        }
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scale: f64) -> () {
        self.width *= scale;
        self.height *= scale;
    }
}

fn main() {
    first_test();
    second_test();
    third_test();
    fourth_test();
    fifth_test();
    sixth_test();
    seventh_test();
}

fn first_test() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b as f64 + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");
}

fn second_test() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn third_test() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean: f64 = 0.0;
    
    for &number in numbers.iter() {
        if number < min {
            min = number;
        } else if number > max {
            max = number;
        }
        
        mean += number as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed!");
}

fn fourth_test() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front."); 
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear.   ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "    We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "      ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = ""; 
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");

    println!("Test passed!");
}

fn fifth_test() {
    println!("---------------------------------------------------");
    let rand_number = thread_rng().gen_range(0..100); 

    println!("rand_number {}", rand_number);
    println!("Higher or lower?");
    loop {
        println!("Insert number: ");
        let mut stdin = String::new();
        let _ = io::stdin().read_line(&mut stdin);
        let num: i32 = stdin.trim().parse().unwrap();

        if rand_number > num {
            println!("Higher");
        } else if rand_number < num {
            println!("Lower");
        } else {
            println!("Winner!!");
            break;
        }
    }

    println!("---------------------------------------------------");
    println!("Test passed!");
}

fn sixth_test() {
    // Added from the tutorial
    if env::args().len() < 2 {
        eprintln!("Program require two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let filename: String = env::args().nth(1).unwrap(); 
    let search_value: String = env::args().nth(2).unwrap(); 

    for line in fs::read_to_string(&filename).unwrap().lines() {
        if line == search_value {
            println!("Found {} on {}", search_value, filename);
            println!("Test passed!");
            return;
        }
    }
    
    println!("Not found {} on {}", search_value, filename);
    println!("Test passed!");
}

fn seventh_test() {
    let mut rect = Rectangle::new(1.2, 3.4);
    println!("{:?}", rect);
    assert_eq!(rect.get_area(), 4.08);

    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);

    println!("Test passed!");
}

fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    (1.8 * degrees) + 32.0 
}

fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

   &s[start..end]
}
