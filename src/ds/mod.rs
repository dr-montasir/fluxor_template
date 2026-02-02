/// 1. Fail: Unacceptable, completely missed requirements.
/// 2. Poor: Well below expectations; significant shortcomings.
/// 3. Fair: Below average; meets minimum requirements but with deficiencies.
/// 4. OK (or Satisfactory/Average): Meets standard expectations; acceptable.
/// 5. Good: Above average; solid, reliable performance.
/// 6. High (or Very Good): Exceeds expectations.
/// 7. Top (or Excellent/Outstanding): Exceptional, superior performance. 
pub fn get_health_description(index: u64) -> &'static str {
    match index {
        0..=9 => "Fail",
        10..=99 => "Poor",
        100..=249 => "Fair",
        250..=499 => "OK",
        500..=999 => "Good",
        1000..=9999 => "High",
        _ => "Top",
    }
}

pub fn get_days(current: &str, created: &str) -> u64 {
    // Extract "YYYY-MM-DD" from both strings
    // Current: "2026-02-01" | Created: "2025-10-16"
    let curr_s = &current[0..10];
    let crea_s = &created[0..10];

    // Helper to parse a slice into i32
    let parse = |s: &str| s.parse::<i32>().unwrap_or(0);

    // Parse Y, M, D
    let y1 = parse(&curr_s[0..4]);
    let m1 = parse(&curr_s[5..7]);
    let d1 = parse(&curr_s[8..10]);

    let y2 = parse(&crea_s[0..4]);
    let m2 = parse(&crea_s[5..7]);
    let d2 = parse(&crea_s[8..10]);

    // Rata Die algorithm to convert date to absolute days since 0001-01-01
    fn to_absolute_days(y: i32, m: i32, d: i32) -> i32 {
        let (y, m) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
        (365 * y) + (y / 4) - (y / 100) + (y / 400) + ((306 * (m + 1)) / 10) + d - 428
    }

    let days1 = to_absolute_days(y1, m1, d1);
    let days2 = to_absolute_days(y2, m2, d2);

    // Return difference as u64
    let age_in_days: u64 = (days1 - days2).abs() as u64;
    age_in_days
}