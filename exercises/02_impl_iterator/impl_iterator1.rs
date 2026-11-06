// Iterators are values in Rust so you can also take them as arguments or return
// them from functions like you would with a String for example.
//
// TODO: Add link to `impl Trait` docs
//
// Change the function so instead of just returning the elemnts in the iterator
// also adds the index to it, just like `.enumerate()` would.
//
// GOAL: Learn to return Iterators

fn enumerate_vec(vec: Vec<&str>) -> impl Iterator<Item = (usize, &str)> {
    vec.into_iter()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        todo!();
    }
}
