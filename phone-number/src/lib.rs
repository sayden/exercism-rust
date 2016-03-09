pub fn number(p: &str) -> Option<String> {
    if p.len() < 10 {
        None
    } else if p.len() == 11 {
        let mut chars = p.chars();
        if chars.next() == Some('1') {
            Some(chars.collect::<String>())
        } else {
            None
        }
    } else {
        Some(p.chars().filter(|c| c.is_alphanumeric() && !c.is_alphabetic()).collect::<String>())
    }
}

pub fn area_code(p: &str) -> Option<String> {
    let chars = p.chars();
    if p.len() == 11 {
        Some(chars.skip(1).take(3).collect())
    } else if p.len() > 11 {
        None
    } else {
        Some(chars.take(3).collect())
    }
}

pub fn pretty_print(p: &str) -> String {
    let chars = p.chars();
    let mut res = String::from("(");
    if p.len() > 11 {
        return "invalid".to_string();
    } else if p.len() < 10 {
        return "invalid".to_string();
    } else if p.len() == 11 {
        for (i, v) in chars.enumerate() {
            if i == 0 && v == '1' {
                continue;
            } else if i == 4 {
                res.push_str(") ");
            } else if i == 7 {
                res.push_str("-");
            }

            res.push(v);
        }
        res
    } else {
        for (i, v) in chars.enumerate() {
            if i == 3 {
                res.push_str(") ");
            } else if i == 6 {
                res.push_str("-");
            }

            res.push(v);
        }
        res
    }
}
