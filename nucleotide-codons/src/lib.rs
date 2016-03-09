pub struct Info {
    pairs: Vec<(&'static str, &'static str)>,
}

impl Info {
    pub fn name_for(&self, s: &str) -> Result<&str, &str> {
        if s.len() != 3 {
            return Err("invalid length");
        }

        let mut final_search = "".to_string();

        for i in s.chars() {
            match i {
                'W' | 'M' | 'R' | 'N' | 'H' => final_search.push_str("A"),
                'S' | 'K' => final_search.push_str("G"),
                'Y' => final_search.push_str("T"),
                _ => final_search.push_str(i.to_string().as_str()),
            }
        }

        match self.pairs.iter().find(|w| w.0 == final_search) {
            Some(n) => Ok(n.1),
            None => Err("Error"),
        }
    }
}

pub fn parse(vec: Vec<(&'static str, &'static str)>) -> Info {
    return Info { pairs: vec };
}
