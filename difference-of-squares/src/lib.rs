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
        let temp = i * i;
        sum += temp;
    }

    return sum;
}


pub fn difference(n: i32) -> i32 {
    let a = square_of_sum(n);
    let b = sum_of_squares(n);

    return a - b;
}
