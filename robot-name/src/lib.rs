extern crate rand;

pub struct Robot {
    name: String,
}

use rand::{thread_rng, Rng};

pub fn generate_number() -> String {
    return thread_rng().gen_range::<u32>(100, 999).to_string();
}

pub fn generate_chars() -> String {
    let choices = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                       'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut res = "".to_string();
    for _ in 0..2 {
        res.push_str(thread_rng().choose(&choices).unwrap().to_string().as_str());
    }

    return res.to_uppercase();
}

pub fn generate_name() -> String {
    let mut name = generate_chars();
    name.push_str(generate_number().as_str());
    return name;
}


impl Robot {
    pub fn new() -> Robot {
        return Robot { name: generate_name() };
    }

    pub fn name<'a>(&'a self) -> &'a str {
        return self.name.as_str();
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name()
    }
}
