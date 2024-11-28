use tokio::time::Duration;

// Orphan rule :(

pub fn dur_from_str(input: &str) -> Option<Duration> {
    let mut split = input.split(':').collect::<Vec<_>>();
    if split.len() > 3 { return None; }
    let mut secs = split.pop()?.parse::<u64>().ok()?;
    if !split.is_empty() {
        let mut mins = split.pop()?.parse::<u64>().ok()?;
        if !split.is_empty() {
            let hours = split.pop()?.parse::<u64>().ok()?;
            mins += hours * 60;
        }
        secs += mins * 60;
    }
    Some(Duration::from_secs(secs))
}

pub fn dur_from_alt_str(input: &str) -> Option<Duration> {
    let chars = input.split_at(input.len() - 1);
    Some(Duration::from_secs(
        match chars.1 {
            "s" => chars.0.parse::<u64>().ok(),
            "m" => chars.0.parse::<u64>().ok()?.checked_mul(60),
            "h" => chars.0.parse::<u64>().ok()?.checked_mul(3600),
            _ => None
        }?
    ))
}