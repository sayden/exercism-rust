use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, n: u32, name: &str) {
        let mut names: Vec<String> = Vec::new();

        match self.grades.get(&n) {
            Some(vector) => {
                names.append(&mut vector.clone());
                names.push(name.to_string());
            }
            None => names.push(name.to_string()),
        }

        names.sort();
        self.grades.insert(n, names);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res = self.grades.keys().map(|k| k.clone()).collect::<Vec<u32>>();
        res.sort();
        res
    }
    pub fn grade(&self, n: u32) -> Option<&Vec<String>> {
        self.grades.get(&n)
    }
}
