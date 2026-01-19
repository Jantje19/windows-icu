use crate::icu::{
    calendar::ICUCalendar, get_time_zone_data_version, iana_to_windows, now,
    timezones::TimeZoneIterator,
};

mod icu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ICU version: {}", get_time_zone_data_version()?);

    let mut cal = ICUCalendar::new("Europe/Amsterdam")?;

    let offset = cal.get_time_zone_offset_in_ms(now())?;
    println!("Offset: {}h", offset / (1000 * 60 * 60));

    let offset = cal.get_time_zone_offset_in_ms(1774821600000.)?;
    println!("Offset: {}h", offset / (1000 * 60 * 60));

    for v in TimeZoneIterator::new()? {
        println!("Timezone: {v}");

        let mut cal = ICUCalendar::new(&v)?;
        println!(
            "\t{}h",
            cal.get_time_zone_offset_in_ms(now())? / (1000 * 60 * 60)
        );
        println!("\t{:?}", iana_to_windows(&v)?);
    }

    Ok(())
}
