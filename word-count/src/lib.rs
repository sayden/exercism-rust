use std::collections::HashMap;

pub fn word_count(raw: &str) -> HashMap<String, u32> {
    let mut mymap: HashMap<String, u32> = HashMap::new();

    // Check is not empty
    if raw.is_empty() {
        return mymap;
    }


    // Take only characters and numbers
    for word in raw.split_whitespace()
                   .flat_map(|w| w.split(","))
                   .flat_map(|w| w.split("_"))
                   .map(|w| w.to_lowercase())
                   .map(|w| w.chars().filter(|_w| _w.is_alphanumeric()).collect::<String>())
                   .filter(|w| !w.is_empty())
                   .into_iter() {


        let res = match mymap.get(&word) {
            Some(n) => n + 1,
            None => 1u32,
        };

        mymap.insert(word.to_string(), res);

    }

    return mymap;
}
