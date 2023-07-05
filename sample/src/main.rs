const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    let test = vec!["one", "two", "three"];
    let index = test.iter().position(|&r| r == "two").unwrap();
    println!("{}", index);
}


// fn find_term(search_term: &str, mut quote: &str) {
//     let mut tii = Vec::new();
//     let mut s = String::new();
//     if quote.chars().last().unwrap() != '\n' {
//         quote += "\n"
//     }
// }