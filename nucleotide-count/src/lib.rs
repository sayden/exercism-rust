use std::collections::HashMap;

pub fn count(search: char, list: &str) -> usize {
    return list.chars()
               .map(|c| {
                   if c == search {
                       1
                   } else {
                       0
                   }
               })
               .fold(0, |a, b| a + b);
}


pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();

    res.insert('A', 0);
    res.insert('C', 0);
    res.insert('G', 0);
    res.insert('T', 0);

    for c in s.chars() {

        let count = match res.get(&c) {
            Some(n) => n + 1,
            None => 1,
        };

        res.insert(c, count);
    }

    return res;
}
