// // Assignment 1

// const FREEZING_POINT_F: f64 = 32.0;

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - FREEZING_POINT_F) * 5.0 / 9.0
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     (c * 9.0 / 5.0) + FREEZING_POINT_F
// }

// fn main() {
//     let mut temp_f = 32.0; // Starting temperature in Fahrenheit
//     let temp_c = fahrenheit_to_celsius(temp_f);
//     println!("{:.2}°F is {:.2}°C", temp_f, temp_c);


//     for i in 1..=5 {
//         let next_temp_f = temp_f + i as f64;
//         let next_temp_c = fahrenheit_to_celsius(next_temp_f);
//         println!("{:.2}°F is {:.2}°C", next_temp_f, next_temp_c);
//     }
// }

// fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }


// // Assignment 2

// fn main() {
//     let numbers = [4, 3, 4, 5, 7, 60, 45, 12, 4, 14];

//     for &num in numbers.iter() {
//         if num % 3 == 0 && num % 5 == 0 {
//             println!("{}: FizzBuzz", num);
//         } else if num % 3 == 0 {
//             println!("{}: Fizz", num);
//         } else if num % 5 == 0 {
//             println!("{}: Buzz", num);
//         } else if is_even(num) {
//             println!("{}: Even", num);
//         } else {
//             println!("{}: Odd", num);
//         }
//     }

  
//     let mut sum = 0;
//     let mut index = 0;
//     while index < numbers.len() {
//         sum += numbers[index];
//         index += 1;
//     }
//     println!("Sum of all numbers: {}", sum);

//     // Find and print the largest number
//     let mut max_num = numbers[0];
//     for &num in numbers.iter() {
//         if num > max_num {
//             max_num = num;
//         }
//     }
//     println!("Largest number: {}", max_num);
// }


// // Assignment 3

// // fn check_guess(guess: i32, secret: i32) -> i32 {
// //     if guess == secret {
// //         0
// //     } else if guess > secret {
// //         1
// //     } else {
// //         -1
// //     }
// // }

// // fn main() {
// //     let secret = 2; 
// //     let mut guess;
// //     let mut attempts = 0;

// //     loop {
      
// //         guess = [0, 4, 1, 2][attempts]; 
// //         attempts += 1;

// //         let result = check_guess(guess, secret);
// //         if result == 0 {
// //             println!("Correct! The secret number is {}", secret);
// //             break;
// //         } else if result == 1 {
// //             println!("{} is too high!", guess);
// //         } else {
// //             println!("{} is too low!", guess);
// //         }
// //     }

// //     println!("You guessed the correct number in {} attempts!", attempts);
// // }




