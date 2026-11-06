// Now that we can built chains of iterator combinators we can explore ways to
// consume those iterators.
//
// One simple operation is summing op all the elements which can be done with the [`.sum()`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.sum) method.
//
// TODO: Describe what consuming an iterator means

fn sum_all_the_numbers(list: Vec<i32>) -> i32 {
    // TODO replace this with an iterator chain
    let mut sum = 0;
    for i in 1..list.len() {
        sum += list[i];
    }
    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_all_values() {
        assert_eq!(sum_all_the_numbers(vec![0, 1, 2, 3, 4]), 10);
        assert_eq!(sum_all_the_numbers(vec![42, 1, 2, 3, 4]), 52);
        assert_eq!(sum_all_the_numbers(vec![]), 0);
    }
}
