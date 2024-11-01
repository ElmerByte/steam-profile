### A simple library to get basic profile statistics from steam
library uses [ureq](https://crates.io/crates/ureq) and [scraper](https://crates.io/crates/scraper) to retrive account information from a steam page

### Features
* sync - on by default provides sync implimentation of this library
* async - provides sync implimentation of this library
* print - on by default provides pretty print method for Profile  
### Sync Example
```rust
use steam_profile::synclib::Profile;
fn main() {
    let profile = Profile::get_full_profile("test");
    profile.print_profile();
}
```
