use rand::prelude::thread_rng;
use rand::Rng;
use std::{collections::HashMap, env, fs, io};

mod shape;
use shape::Rectangle;

mod satellite;
use satellite::Satellite;

mod location;
use location::Location;

mod utils;

pub fn first_test() {
    println!("1. Test started");
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b as f64 + c as f64) / 3.0;
    assert_eq!(average, 45.1);

    println!("1. Test passed!");
}

pub fn second_test() {
    println!("2. Test started");

    let celsius_temp = 23.0;
    let fahrenheit_temp = utils::celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);

    println!("2. Test passed!");
}

pub fn third_test() {
    println!("3. Test started");

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

    println!("3. Test passed!");
}

pub fn fourth_test() {
    println!("4. Test started");

    let test1 = "We need more space.";
    assert_eq!(utils::trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(utils::trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear.   ");
    assert_eq!(utils::trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "    We're surrounded by space!    ";
    assert_eq!(utils::trim_spaces(test4), "We're surrounded by space!");

    let test5 = "      ";
    assert_eq!(utils::trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(utils::trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(utils::trim_spaces(test7), "🚀");

    println!("4. Test passed!");
}

pub fn fifth_test() {
    println!("5. Test started");

    let rand_number = thread_rng().gen_range(0..100);

    println!("\trand_number {}", rand_number);
    println!("\tHigher or lower?");

    loop {
        println!("\tInsert number: ");
        let mut stdin = String::new();
        let num: u32 = match io::stdin().read_line(&mut stdin) {
            Ok(_) => match stdin.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("\tChoose a valid number");
                    continue;
                }
            },
            Err(_) => {
                println!("\tChoose a valid number");
                continue;
            }
        };

        if rand_number > num {
            println!("\tHigher");
        } else if rand_number < num {
            println!("\tLower");
        } else {
            println!("\tWinner!!");
            break;
        }
    }

    println!("\t---------------------------------------------------");

    println!("5. Test passed!");
}

#[allow(dead_code)]
pub fn sixth_test() {
    println!("6. Test started");

    if env::args().len() < 2 {
        eprintln!("Program require two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let filename: String = env::args().nth(1).unwrap();
    let search_value: String = env::args().nth(2).unwrap();

    for line in fs::read_to_string(&filename).unwrap().lines() {
        if line == search_value {
            println!("\tFound {} on {}", search_value, filename);
            println!("\tTest passed!");
            return;
        }
    }

    println!("\tNot found {} on {}", search_value, filename);

    println!("6. Test passed!");
}

pub fn seventh_test() {
    println!("7. Test started");

    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);

    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);

    println!("7. Test passed!");
}

pub fn eighth_test() {
    println!("8. Test started");

    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*utils::sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*utils::sum_boxes(pi, e), 5.85987);

    println!("8. Test passed!");
}

pub fn nineth_test() {
    println!("9. Test started");

    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("\thubble is {}", hubble);

    println!("9. Test passed!");
}

pub fn tenth_test() {
    println!("10. Test started");

    let address = Location::Unknown;
    address.display();

    let address = Location::Anonymous;
    address.display();

    let address = Location::Known(28.608295, -80.604177);
    address.display();

    println!("10. Test passed!");
}

pub fn eleventh_test() {
    println!("11. Test started");

    if env::args().len() < 2 {
        eprintln!("Program require two arguments: <file path>");
        std::process::exit(1);
    }

    let filename: String = match env::args().nth(1) {
        Some(e) => e,
        None => {
            eprintln!("A problem occur parsing your arguments");
            std::process::exit(1);
        }
    };

    let buf = match fs::read_to_string(&filename) {
        Ok(b) => b.to_lowercase(),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("A problem occur parsing your arguments");
                std::process::exit(1);
            }
            io::ErrorKind::OutOfMemory => {
                eprintln!("A problem occur parsing your arguments");
                std::process::exit(1);
            }
            _ => {
                eprintln!("A problem occur parsing your arguments");
                std::process::exit(1);
            }
        },
    };

    let mut words = HashMap::new();
    let mut top_count: u32 = 32;
    let mut top_word: Vec<&str> = Vec::new();

    for line in buf.lines() {
        for word in line.split_whitespace() {
            let exist = words.entry(word).or_insert(0);
            *exist += 1;
        }
    }

    for (&key, &val) in words.iter() {
        if val > top_count {
            top_count = val;
            top_word.clear();
            top_word.push(key);
        } else if val == top_count {
            top_word.push(key);
        }
    }

    println!("\tTop word(s) ocurred {}:", top_count);
    for top in top_word.iter() {
        println!("\t\t'{}'", top)
    }

    println!("11. Test passed!");
}
