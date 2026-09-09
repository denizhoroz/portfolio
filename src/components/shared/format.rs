// Formatting shared by more than one route. Content stores dates in ISO because
// that sorts and cannot be misread as US order; this is where they become
// something to read.

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

/// `2025-06-14` -> `14 June 2025`.
///
/// Anything that does not parse is returned as written rather than dropped -- a
/// visible odd date is a bug report, a missing one is silence.
pub fn format_date(iso: &str) -> String {
    let mut parts = iso.split('-');
    let (Some(y), Some(m), Some(d), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return iso.to_string();
    };

    let (Ok(month), Ok(day)) = (m.parse::<usize>(), d.parse::<u32>()) else {
        return iso.to_string();
    };

    match MONTHS.get(month.wrapping_sub(1)) {
        Some(name) => format!("{day} {name} {y}"),
        None => iso.to_string(),
    }
}
