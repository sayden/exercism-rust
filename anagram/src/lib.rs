fn words_are_equal(a: &str, b: &str) -> bool {
    let mut new_a: Vec<char> = a.to_lowercase().chars().collect();
    let mut new_b: Vec<char> = b.to_lowercase().chars().collect();

    new_a.sort();
    new_b.sort();

    return new_a == new_b;

}

pub fn anagrams_for<'a>(search_word: &str, input: &'a [&str]) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();

    for w in input {
        if search_word.to_lowercase().to_string() == w.to_lowercase().to_string() {
            res.push(w)
        } else {
            if search_word.len() == w.len() {
                if words_are_equal(search_word, w) {
                    res.push(w);
                } else {
                    println!("\n \n -------------> Words {} and {} are not anagrams \n \n ",
                             &search_word,
                             &w);
                }
            } else {
                println!("\n \n -------------> Words {} and {} are different in length \n \n ",
                         &search_word,
                         &w);
            }
        }
    }

    return res;
}
