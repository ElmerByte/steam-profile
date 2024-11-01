fn check_id(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            count += 1;
        }
    }
    count == 17
}
fn check_link(s: &str) -> bool {
    s.contains("steamcommunity.com")
}
pub fn convert_to_url(inp: &str) -> String {
    if check_link(inp) {
        inp.to_owned()
    } else if check_id(inp) {
        format!("https://steamcommunity.com/profiles/{}", inp)
    } else {
        format!("https://steamcommunity.com/id/{}", inp)
    }
}
