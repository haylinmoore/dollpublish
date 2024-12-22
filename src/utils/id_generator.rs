use rand::seq::SliceRandom;

const WORDS: &[&str] = &[
    "penguin", "giraffe", "walrus", "dolphin", "raccoon", "platypus", "octopus", "kangaroo",
    "waffle", "taco", "sushi", "pizza", "banana", "mango", "cookie", "pretzel", "pencil", "bucket",
    "hammer", "rocket", "basket", "camera", "compass", "ladder", "river", "mountain", "forest",
    "desert", "island", "volcano", "glacier", "canyon", "purple", "orange", "crimson", "azure",
    "golden", "silver", "scarlet", "emerald", "dancing", "jumping", "flying", "running", "sailing",
    "diving", "climbing", "floating",
];

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    (0..4)
        .map(|_| *WORDS.choose(&mut rng).unwrap())
        .collect::<Vec<&str>>()
        .join("-")
}
