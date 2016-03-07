pub fn is_leap_year(year: i32) -> bool {
    match year % 4 {
        0 => {
            match year % 400 {
                0 => return true,
                _ => {
                    match year % 100 {
                        0 => return false,
                        _ => return true,
                    }
                }
            }
        }
        _ => return false,
    };
}
