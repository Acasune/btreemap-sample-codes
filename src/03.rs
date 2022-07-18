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
    println!("[Before] map: {:?}", map);
    // [Before] map: {2: [1, 2, 3], 4: [2, 4, 6], 6: [3, 6, 9], 8: [4, 8, 12], 10: [5, 10, 15]}

    // Replacing procedure
    for i in 1..=5 {
        let idx = 2 * i - 1;
        if let Some((&key, _)) = map.range((idx)..).next() {
            let val = map.remove(&key).unwrap();
            map.insert(idx, val);
        }
    }
    println!("[After] map: {:?}", map);
    // [After] map: {1: [1, 2, 3], 3: [2, 4, 6], 5: [3, 6, 9], 7: [4, 8, 12], 9: [5, 10, 15]}
}
