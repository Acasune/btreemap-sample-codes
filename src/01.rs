fn main() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::<usize, Vec<i32>>::new();

    // Preparation
    for i in 1..=5 {
        let mut vec = vec![];
        for j in 1..=3 {
            vec.push((i * j) as i32);
        }
        map.insert(2 * i, vec);
    }
    // [Before] map: {2: [1, 2, 3], 4: [2, 4, 6], 6: [3, 6, 9], 8: [4, 8, 12], 10: [5, 10, 15]}

    // Replacing procedure
    // You'll encounter a compile error at line 19
    for i in 1..=5 {
        let idx = 2 * i - 1;
        if let Some((&key, &value)) = map.range((idx)..).next() {
            map.insert(i, value);
            map.remove(&key);
        }
    }
}
