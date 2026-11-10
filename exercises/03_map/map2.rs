use next_next_next::Record;

// TODO: use map to extract just the `station_name` field from every record.

fn extract_station_name(records: impl Iterator<Item = Record>) -> impl Iterator<Item = String> {
    records.map(|r| todo!("fill me in"))
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use next_next_next::read_services;

    use super::*;

    #[test]
    fn top5_stations() {
        // Read in the dataset
        let records = read_services("services-2026-05.csv.gz");

        // Extract just the station names
        let names = extract_station_name(records);

        // Count how often each name shows up
        let mut name_counts: HashMap<String, usize> = HashMap::new();
        for name in names {
            // TODO: Add link to an explination of the entry API
            let entry = name_counts.entry(name).or_default();
            *entry += 1;
        }

        // Sort the list by the count and take just the top 5 stations
        let mut top_list: Vec<_> = name_counts.into_iter().collect();
        top_list.sort_unstable_by_key(|(_name, count)| *count);
        top_list.reverse();
        top_list.truncate(5);

        // Check against the refrence
        assert_eq!(
            top_list,
            [
                ("Amsterdam Centraal", 43128),
                ("Utrecht Centraal", 38373),
                ("Amsterdam Sloterdijk", 28760),
                ("Rotterdam Centraal", 26040),
                ("Schiphol Airport", 25394),
            ]
            .into_iter()
            .map(|(name, count)| (String::from(name), count))
            .collect::<Vec<_>>()
        );
    }
}
