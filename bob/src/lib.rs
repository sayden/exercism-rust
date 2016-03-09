pub fn reply(q: &str) -> &str {
    if q.is_empty() {
        "Fine. Be that way!"
    } else if q.chars().all(|c| !c.is_lowercase()) {
        "Whoa, chill out!"
    } else if q.ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
