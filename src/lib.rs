pub fn find(n: usize, f: usize) {
    let zeros = &"0".repeat(n);

    (1_u32..)
        .map(|num| (num, sha256::digest(num.to_string())))
        .filter(|(_, hash)| hash.ends_with(zeros))
        .take(f)
        .for_each(|(num, hash)| {
            println!("{num}, {hash:?}");
        })
}
