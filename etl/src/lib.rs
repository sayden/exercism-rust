use std::collections::BTreeMap;

pub fn transform(map: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res: BTreeMap<String, i32> = BTreeMap::new();
    for (&n, s) in map {
        for w in s.iter() {
            res.insert(w.clone().to_lowercase(), n);
        }
    }

    res
}
