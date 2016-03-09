pub fn square_of_sum(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..n + 1 {
        sum += i
    }

    return sum * sum;
}

pub fn sum_of_squares(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i * i;
    }

    return sum;
}


pub fn difference(n: i32) -> i32 {
    return square_of_sum(n) - sum_of_squares(n);
}
