
use std::io;

fn two_letters() -> Option<(char, char)> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();
    if input.len() != 2 {
        return None;
    }

    return Some((input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap()));
}

fn letter_and_number() -> Option<(char, i32)> {
    let (character, number) = match two_letters() {
        None => return None,
        Some(val) => val,
    };

    let number = (number as i32) - ('0' as i32); // this fucking sucks

    return Some((character, number));
}

fn letter_bound__number_bound(letter_from: char, letter_to: char, number_from: i32, number_to: i32) -> Option<(char, i32)> {
    let (letter, number) = match letter_and_number() {
        None => return None,
        Some(v) => v,
    };

    if (number < number_from) || (number > number_to) {
        return None;
    }

    if (letter < letter_from) || (letter > letter_to) {
        return None;
    }

    Some((letter, number))
}

pub fn letter_bound_normalised__number_bound_normalised(letter_from: char, letter_to: char, number_from: i32, number_to: i32) -> Option<(i32, i32)> {
    let (letter, number) = match letter_bound__number_bound(letter_from, letter_to, number_from, number_to) {
        None => return None,
        Some(v) => v,
    };

    let number = number - number_from;
    let letter = (letter as i32) - (letter_from as i32); // this fucking sucks

    Some((letter, number))
}

// pub fn u8(from: u8, to: u8) -> Option<u8> {
//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .unwrap();

//     let number = input.trim().parse::<u8>();

//     let number = match number {
//         Err(err) => return None,
//         Ok(val) => val,
//     };

//     if number < from {
//         return None;
//     }

//     if number > to {
//         return None;
//     }

//     Some(number)
// }

// pub fn get_num(prompt: &str, from:usize, to:usize) -> usize {
//     loop {
//         print!("{} [{}:{}] > ", prompt, from, to);
//         io::stdout().flush().unwrap();

//         let mut input = String::new();

//         io::stdin()
//             .read_line(&mut input)
//             .unwrap();

//         let number = input.trim().parse::<usize>();

//         if let Err(_err) = number {
//             println!("not a number or invaid");
//             continue;
//         }

//         let number = number.unwrap();

//         if number < from {
//             println!("number too small");
//             continue;
//         }

//         if number > to {
//             println!("number too big");
//             continue;
//         }

//         return number;
//     }
// }
