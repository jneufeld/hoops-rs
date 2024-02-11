fn time_conversion(s: &str) -> String {
    // Input format: HH:MM:SS[AM|PM]
    let mut splitter = s.split(':');

    let hour = splitter
        .next()
        .expect("Bad input format for hour")
        .parse::<i32>()
        .expect("Can't parse hour to i32");

    let minute = splitter.next().expect("Bad input format for minute");

    let (second, meridiem) = splitter
        .next()
        .expect("Bad format for second and meridiem")
        .split_at(2);

    // Convert 12-hour to 24-hour value using meridiem value. Ugly, but it
    // works :shrug:
    let mil_hour = match meridiem {
        "AM" => {
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        "PM" => {
            if hour == 12 {
                12
            } else {
                (hour + 12) % 24
            }
        }
        uhoh => panic!("Neither AM nor PM: {uhoh}"),
    };

    format!("{:02}:{minute}:{second}", mil_hour)
}
