//! An ultra-simple library with a single function for simple duration printing.


/// Prints duration in form `hour:min:sec:milli:micro:nano`.
/// ```
/// use print_duration::print_duration;
/// let duration = std::time::Duration::from_nanos(2212123443);
/// assert_eq!(print_duration(duration, 0..6), "00:00:02:212:123:443");
/// assert_eq!(print_duration(duration, 2..6), "02:212:123:443");
/// assert_eq!(print_duration(duration, 2..4), "02:212");
/// ```
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
