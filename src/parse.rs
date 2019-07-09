use proptest::prelude::*;

// YYYY-MM-DD -> (YYYY, MM, DD)
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() {
        return None;
    }
    if "-" != &s[4..5] || "-" != &s[7..8] {
        return None;
    }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d))))
}

proptest! {
    #[test]

    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }
}
