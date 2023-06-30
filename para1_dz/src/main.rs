fn main() {
    for i in 1..101{
        if i % 3 == 0{
            if i % 15 == 0{println!("FizzBazz")}
            else{println!("Fizz")}
        } else if i % 5 == 0{
            if i % 15 == 0{println!("FizzBazz")}
            else{println!("Bazz")}
        } else{
            println!("{i}")
        }
    }
}



// ИГРА УГАДАЙКА (описать зависимость  RAND в Cargo.toml )
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
//
// fn main() {
//     println!("Guess the number!");
//
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//
//     loop {
//         println!("Please input your guess.");
//
//         let mut guess = String::new();
//
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         println!("You guessed: {guess}");
//
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }


// ИЗМЕНЕНИЕ ПЕРЕМЕННЫХ
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// ОБЪЯВЛЕНИЕ CONST
// const HOLA : i32 = 8 * 3 * 52;
// fn main(){
//
// }


// ЗАТЕНЕНИЕ ПЕРЕМЕННЫХ
// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is: {x}");
// }


// ФУНКЦИИ СИНТАКС
// fn main(){
//     let x = 643;
//     let x_v2 = x_sum_w_7(x);
//     print_x(x_v2)
//
// }
//
// fn print_x(a: i32){
//     println!("let x = {}", a)
// }
//
// fn x_sum_w_7(a: i32) -> i32{
//     a + 7
// }


// СТРОКА , ДОБАВЛЕНИЕ К СТРОКЕ
// fn main() {
//     let mut s = String::from("hello");
//
//     s.push_str(", world!"); // push_str() добавляет literal в String
//
//     println!("{}", s);
// }


// КЛОНИРОВАНИЕ ПЕРЕМЕННЫХ
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//
//     println!("s1 = {}, s2 = {}", s1, s2);
// }


// возврат права владения на параметры
// fn main() {
//     let s1 = String::from("hello");
//
//     let (s2, len) = calculate_length(s1);
//
//     println!("The length of '{}' is {}.", s2, len);
// }
//
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//
//     (s, length)
// }


// В ОДНОЙ ЗОНЕ ВИДИМОСТИ НЕ МОЖЕТ БЫТЬ ДВЕ ССЫЛКИ НА ОДНУ ПЕРЕМЕННУЮ ОДНОВРЕМЕННО
// fn main() {
//     let mut s = String::from("hello");
//
//     {
//         let r1 = &mut s;
//         println!("{}", r1)
//     } // r1 goes out of scope here, so we can make a new reference with no problems.
//
//     let r2 = &mut s;
//     println!("{}", r2)
// }


// СРЕЗЫ
// fn main() {
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     let world = &s[6..11];
// }
// fn main() {
//     let s = String::from("hello");
//
//     let len = s.len();
//
//     let slice = &s[0..len];
//     println!("{}", slice);
//     let slice = &s[..];
//     println!("{}", slice);
//
// }
// ЧТО-ТО ПОЛЕЗНОЕ
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");
//
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     let word = first_word(&my_string);
//
//     let my_string_literal = "hello world";
//
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);
//
//     let word = first_word(my_string_literal);
// }
