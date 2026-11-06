use next_next_next::read_services;

// GOAL introduce to train dataset
//
// Hints: open the docs by running `cargo doc --open`, see that `read_services`
// returns an Iterator over Records, have a look at Records to see which fields
// are available Optionally look at the csv file?

fn find_first_to_stop_in(station_name: &str) -> Option<u32> {
    for record in read_services("services-2026-05.csv.gz") {
        // TODO: find the first train in the dataset that stops at `station_name` and
        // return its `service_train_number`

        dbg!(record);
        return Some(42);
    }

    return None;
}

fn main() {
    dbg!(find_first_to_stop_in("Utrecht Centraal"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        todo!();
    }
}
