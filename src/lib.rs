/// Ranges hh:mm:ss:ms:us:ns (hour:min:sec:milli:micro:nano)
pub fn print_duration(duration: std::time::Duration, range: std::ops::Range<usize>) -> String {
    assert!(
        range.end < 7,
        "Range passed to `print_duration` must be within or equal to the range 0..6"
    );
    let times: [u16; 6] = [
        (duration.as_secs() / 3600) as u16,
        ((duration.as_secs() / 60) % 60) as u16,
        (duration.as_secs() % 60) as u16,
        (duration.as_millis() % 1000) as u16,
        (duration.as_micros() % 1000) as u16,
        (duration.as_nanos() % 1000) as u16,
    ];

    let outs: [String; 6] = [
        format!("{:#02}", times[0]),
        format!("{:#02}", times[1]),
        format!("{:#02}", times[2]),
        format!("{:#03}", times[3]),
        format!("{:#03}", times[4]),
        format!("{:#03}", times[5]),
    ];
    outs[range].join(":")
}
