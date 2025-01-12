### A simple library to get basic profile statistics from steam
library uses [ureq](https://crates.io/crates/ureq) and [scraper](https://crates.io/crates/scraper) to retrive account information from a steam page

### Features
* sync - on by default provides sync implimentation of this library
* async - provides sync implimentation of this library
* print - on by default provides pretty print method for Profile  
### Sync Example
```rust
fn sync_example() {
    use steam_profile::synclib::Profile;
    let profile: Profile = Profile::get_full_profile("test" /*Name Id or URL*/);
    profile.print_profile(); // Prints stats in a nice looking table
}

// requires async feature
async fn async_example() {
    use steam_profile::asynclib::Profile;
    let profile: Profile = Profile::get_full_profile("test" /*Name Id or URL*/).await;
    profile.print_profile(); // Prints stats in a nice looking table
}
```
