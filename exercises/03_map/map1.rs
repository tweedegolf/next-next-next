// A common use of iterators is to apply a function to every element in Rust
// this is done using the `.map()` combinator
//
// TODO: List out aliasses in other languages
//
// GOAL: Introduce map as a tool

fn starting_letters(list: Vec<String>) -> impl Iterator<Item = char> {
    list.into_iter().map(|element| 'a')
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_message() {
        let list = ["Foo", "Erris", "Rust", "River", "Ice", "Stratum"]
            .into_iter()
            .map(String::from)
            .collect();

        let answer: String = starting_letters(list).collect();
        assert_eq!(answer, "FERRIS");
    }
}
