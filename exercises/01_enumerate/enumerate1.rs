// TODO: Explain what a combinator is use enumerate as example: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.enumerate
// TODO: Explain pattern matching like `let (i, val) = (42, "bar");` or link to
// docs

fn longest_word(vec: Vec<String>) -> usize {
    let longest = String::new();
    let longest_pos = 0;

    for word in vec {
        if word.len() > longest_pos {
            longest = word;
            longest_pos = todo!();
        }
    }

    longest_pos
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn improve_me() {
        let list = ["foo", "todo", "känguru"]
            .iter()
            .map(String::from)
            .collect();

        assert_eq!(longest_word(list), 2);
    }
}
