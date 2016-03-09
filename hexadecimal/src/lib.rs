fn hex_char_to_int(c: char) -> Option<i64> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _ => None,
    }
}

pub fn hex_to_int(s: &str) -> Option<i64> {
    let base: i64 = 16;
    let positions = 0..s.len() as u32;

    s.chars()
     .rev()
     .zip(positions)
     .fold(Some(0), |accumulator, (c, pos)| {
         hex_char_to_int(c).and_then(|n| accumulator.map(|acc| acc + n * base.pow(pos)))
     })
}
