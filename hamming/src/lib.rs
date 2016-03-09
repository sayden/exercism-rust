pub fn hamming_distance<'a>(a: &str, b: &str) -> Result<usize, &'a str> {
    if a.len() != b.len() {
        let err: &'a str = "inputs of different length";
        return Result::Err(err);
    }

    Result::Ok(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
}
