#[cfg(test)]
mod tests {
    const TIME: u64 = 2212123443;
    #[test]
    fn nanos() {
        let duration = std::time::Duration::from_nanos(TIME);
        assert_eq!(print_duration::print_duration(duration, 5..6), "443");
    }
    #[test]
    fn micros() {
        let duration = std::time::Duration::from_nanos(TIME);
        assert_eq!(print_duration::print_duration(duration, 4..5), "123");
    }
    #[test]
    fn millis() {
        let duration = std::time::Duration::from_nanos(TIME);
        assert_eq!(print_duration::print_duration(duration, 3..4), "212");
    }
    #[test]
    fn seconds() {
        let duration = std::time::Duration::from_nanos(TIME);
        assert_eq!(print_duration::print_duration(duration, 2..3), "02");
    }
    #[test]
    fn mixed() {
        let duration = std::time::Duration::from_nanos(TIME);
        assert_eq!(
            print_duration::print_duration(duration, 0..6),
            "00:00:02:212:123:443"
        );
    }
}
