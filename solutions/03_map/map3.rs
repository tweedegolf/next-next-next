use next_next_next::Record;

fn extract_delay(records: impl Iterator<Item = Record>) -> impl Iterator<Item = (String, u32)> {
    records.map(name_and_delay)
}

fn name_and_delay(record: Record) -> (String, u32) {
    let delay = match (record.stop_arrival_delay, record.stop_departure_delay) {
        (None, None) => unreachable!("a train always arrives, and or departs at a station"),
        (Some(delay), None) | (None, Some(delay)) => delay,
        (Some(d1), Some(d2)) => u32::max(d1, d2),
    };

    (record.stop_station_name, delay)
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
        let names = extract_delay(records);

        // Sum up all the delays
        let mut delay_per_station: HashMap<String, u32> = HashMap::new();
        for (name, delay) in names {
            // TODO: Add link to an explination of the entry API
            let entry = delay_per_station.entry(name).or_default();
            *entry += delay;
        }

        // Sort the list by the delay and take just the top 5 stations
        let mut top_list: Vec<_> = delay_per_station.into_iter().collect();
        top_list.sort_unstable_by_key(|(_name, delay)| *delay);
        top_list.reverse();
        top_list.truncate(5);

        // Check against the refrence
        assert_eq!(
            top_list,
            [
                ("Amsterdam Centraal", 54859),
                ("Rotterdam Centraal", 40597),
                ("Schiphol Airport", 34695),
                ("Utrecht Centraal", 31426),
                ("Amsterdam Sloterdijk", 28444),
            ]
            .into_iter()
            .map(|(name, delay)| (String::from(name), delay))
            .collect::<Vec<_>>()
        );
    }
}
