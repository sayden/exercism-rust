pub fn chain(vec: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    if vec.is_empty() {
        Some(vec![])
    } else if vec.len() == 1 {
        let res = vec.clone();
        Some(res)
    } else {
        // Flatten
        let flattened = vec.iter().flat_map(|(a, b)| vec![a, b]).collect();
        let mut res = Vec::with_capacity(vec.len());
        for (i, n) in flattened.sort().iter().enumerate() {

        }
    }
}
