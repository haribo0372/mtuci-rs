/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */
const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    println!("{:?}", find_term(SEARCH_TERM, QUOTE))
}

fn find_term(search_term: &str, quote: &str)  -> String{
    let mut vec = Vec::new();
    let mut quote_string = quote.clone().to_string();
    let mut rowww = "".to_string();
    let mut s = String::new();
    if quote_string.chars().last().unwrap() != '\n'{ quote_string.push_str("\n")}
    for i in 0..quote_string.len() {
        if &quote_string[i..i + 1] == "\n" {
            vec.push(s.clone());
            if s.clone().contains(search_term) {
                rowww = s.clone()
            }
            s = "".to_string();
        } else { s.push_str(&quote_string[i..i + 1]) }
    }
    return format!("{}: {}", vec.iter().position(|r| r == &rowww).unwrap() + 1, rowww)
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
