use rayon::prelude::*;

struct Pair {
    number: u32,
    hash: String,
}

fn find_all_with_n_zeros(n: usize) -> impl ParallelIterator<Item = Pair> {
    let zeros = "0".repeat(n);

    (1_u32..)
        .par_bridge()
        .map(|number| Pair {
            number,
            hash: sha256::digest(number.to_string()),
        })
        .filter(move |it| it.hash.ends_with(&zeros))
}

pub fn find(n: usize, f: usize) {
    find_all_with_n_zeros(n)
        .take_any(f)
        .for_each(|Pair { number, hash }| {
            println!(r#"{number}, "{hash}""#);
        });
}
