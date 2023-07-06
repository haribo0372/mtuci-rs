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

fn find_term(search_term: &str, quote: &str) -> String{
    let mut vec = Vec::new();
    let mut index = 0;
    let mut str = "".to_string();
    for row in quote.split('\n'){
        vec.push(row);
        if row.contains(search_term){
            str = row.clone().to_string()
        }
    }
    return format!("{}: {}", vec.iter().position(|r| r == &str).unwrap() + 1, str)

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
