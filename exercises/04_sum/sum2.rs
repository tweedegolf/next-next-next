// Using aggregations such as sum becommes even more powerful, when combinint it
// with map to extract the interesting parts of a piece of data.
//
// Sum up all the delays the trains experienced in the dataset. If a stop has
// both an arrival an an departure delay use the departure delay.

use next_next_next::Record;

fn may_delay(records: impl Iterator<Item = Record>) -> u32 {
    // TODO return the sum of departure delays in the records iterator
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use jiff::civil::Date;
    use next_next_next::read_services;

    use super::*;

    #[test]
    fn delays_day1() {
        let records = read_services("services-2026-05.csv.gz")
            .take_while(|r| r.service_date == Date::constant(2026, 5, 1));
        let total_delay: u32 = may_delay(records);

        assert_eq!(total_delay, 46120);
    }

    #[test]
    fn delays_all_may() {
        let records = read_services("services-2026-05.csv.gz");
        let total_delay: u32 = may_delay(records);

        assert_eq!(total_delay, 1537043);
    }
}
