### A simple library to get basic profile statistics from steam
library uses [ureq](https://crates.io/crates/ureq) and [scraper](https://crates.io/crates/scraper) to retrive account information from a steam page

### Sync Example
```rust
use steam_profile::synclib::Profile;
fn main() {
    let profile = Profile::get_full_profile("test");
    profile.print_profile();
}
```
