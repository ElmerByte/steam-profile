use crate::error::ProfileError;
use crate::url;
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::selectable::Selectable;
use serde::Deserialize;
use std::collections::HashMap;
/// Fetches basic inventory information (Name of the game, Amounts of items) from a specified profile
/// and parses it into a `HashMap`.
///
/// # Arguments
///
/// * `url_name_id` - A string representing either a URL or a username/profile name as well as steamID. This is used to construct the request URL.
///
/// # Returns
///
/// Returns a `Result` containing a `HashMap<String, u32>` mapping game names to their item counts, or a `ProfileError` if an error occurs during fetching or parsing.
///
pub async fn get_inventory_by_url(url_name_id: &str) -> Result<HashMap<String, u32>, ProfileError> {
    let url = url::convert_to_url(url_name_id);
    let content = match ureq::get(format!("{}/inventory", url).as_str()).call() {
        Ok(r) => r.into_string()?,
        Err(_) => {
            return Err(ProfileError::FetchError(format!(
                "Failed to parse, url: {}",
                url
            )))
        }
    };
    let document = scraper::Html::parse_document(&content);
    let selector = scraper::Selector::parse("a.games_list_tab")?;
    let stats = document.select(&selector);

    let mut links: HashMap<String, u32> = HashMap::new();

    for profile in stats {
        let count: String = profile
            .select(&scraper::Selector::parse("span.games_list_tab_number")?)
            .next()
            .map(|num| num.text().collect::<String>())
            .unwrap_or("0".to_string())
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect();
        let name = profile
            .select(&scraper::Selector::parse("span.games_list_tab_name")?)
            .next()
            .map(|name| name.text().collect::<String>())
            .unwrap_or("Game Name Not Found".to_owned());
        let number = count.parse().unwrap_or(0);
        if number > 0 {
            links.insert(name, number);
        }
    }
    Ok(links)
}
/// Fetches activities based on a URL or name and returns a map of names to counts.
///
/// This function takes a URL or name as input, fetches the corresponding web page, parses it to extract activity counts associated with various profiles, and returns a `HashMap` mapping profile names to their activity counts. If the fetching or parsing fails, it returns an error indicating the failure.
///
/// # Arguments
///
/// * `url_name_id` - A string representing either a URL or a username/profile name. This is used to construct the request URL.
///
/// # Errors
///
/// This function returns a `ProfileError` in case of failure during fetching or parsing. Possible errors include network issues (`FetchError`) or parsing failures (`ParseError`).
///
pub async fn get_activities_by_url(
    url_name_id: &str,
) -> Result<HashMap<String, u32>, ProfileError> {
    let url = url::convert_to_url(url_name_id);
    let content = match ureq::get(&url).call() {
        Ok(r) => r.into_string()?,
        Err(_) => {
            return Err(ProfileError::FetchError(format!(
                "Failed to parse, url: {}",
                url
            )))
        }
    };
    let document = scraper::Html::parse_document(&content);
    let html_product_selector = scraper::Selector::parse("div.profile_count_link")?;
    let html_products = document.select(&html_product_selector);

    let mut links: HashMap<String, u32> = HashMap::new();

    for profile in html_products {
        let count: String = profile
            .select(&scraper::Selector::parse("span.profile_count_link_total")?)
            .next()
            .map(|text| text.text().collect::<String>())
            .unwrap_or_default()
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect();
        let name = profile
            .select(&scraper::Selector::parse("span.count_link_label")?)
            .next()
            .map(|number| number.text().collect::<String>())
            .unwrap_or_default();

        let number = count.parse().unwrap_or(0);
        if number > 0 {
            links.insert(name, number);
        }
    }
    Ok(links)
}

pub async fn get_name_by_url(url_name_id: &str) -> Result<SteamProfile, ProfileError> {
    let url = url::convert_to_url(url_name_id);
    let content = match ureq::get(&url).call() {
        Ok(r) => r.into_string()?,
        Err(_) => {
            return Err(ProfileError::FetchError(format!(
                "Failed to parse, url: {}",
                url
            )))
        }
    };
    let document = scraper::Html::parse_document(&content);
    let selector = scraper::Selector::parse("div#responsive_page_template_content")?;
    let pp: String = document
        .select(&scraper::Selector::parse("div.playerAvatarAutoSizeInner")?)
        .next()
        .ok_or(ProfileError::Anyhow(anyhow!("Scrapper Failed")))?
        .child_elements()
        .last()
        .ok_or(ProfileError::Anyhow(anyhow!("Scrapper Failed")))?
        .attr("src")
        .unwrap_or("none")
        .to_string();
    let lvl: String = document
        .select(&scraper::Selector::parse("span.friendPlayerLevelNum")?)
        .next()
        .ok_or(ProfileError::Anyhow(anyhow!("Scrapper Failed")))?
        .inner_html()
        .to_string();
    let mut stats: String = document
        .select(&selector)
        .next()
        .ok_or(ProfileError::Anyhow(anyhow!("Scrapper Failed")))?
        .child_elements()
        .next()
        .ok_or(ProfileError::Anyhow(anyhow!("Scrapper Failed")))?
        .text()
        .collect();
    stats = stats.replace(['\\', '\n', '\t'], "" )/*. replace(r#""url":"https:steamcommunity.comid","#,"" ) */.replace("g_rgProfileData = ","" ).replace(";const g_bViewingOwnProfile = 0;$J( function() {window.Responsive_ReparentItemsInResponsiveMode && Responsive_ReparentItemsInResponsiveMode( '.responsive_groupfriends_element', $J('#responsive_groupfriends_element_ctn') );SetupAnimateOnHoverImages();});","" );
    let re = Regex::new(r#""summary":"(.*?)\s*"}"#)?;
    let test = re.captures(&stats).ok_or(anyhow!("Nothing is Captured"))?;
    let stf: SteamProfile = serde_json::from_str(
        &stats
            .replace(&test[1], &test[1].replace('"', r#"\""#))
            .replace(
                '}',
                format!(r#","imgurl":"{}","lvl":"{}"}}"#, pp, lvl).as_str(),
            ),
    )?;
    Ok(stf)
}
/// Represents a user's Steam profile.
#[derive(Debug, Deserialize, Default)]
pub struct Profile {
    /// The URL of the Steam profile.
    url: String,
    /// The name of the user.
    name: String,
    /// The Steam ID of the user.
    steamid: String,
    /// A description of the user.
    description: String,
    /// The URL of the user's profile picture.
    profilepic: String,
    /// The user's level on Steam.
    level: u32,
    /// A hashmap of game statistics.
    stats: HashMap<String, u32>,
    /// A hashmap of inventory information.
    inventory: HashMap<String, u32>,
}

#[derive(Debug, Deserialize, Default)]
struct SteamProfile {
    url: String,
    steamid: String,
    personaname: String,
    summary: String,
    imgurl: String,
    lvl: String,
}
impl Profile {
    pub async fn get_full_profile(url_name_id: &str) -> Self {
        // Await all tasks to complete and collect their results
        let results = tokio::join!(
            get_name_by_url(url_name_id),
            get_activities_by_url(url_name_id),
            get_inventory_by_url(url_name_id),
        );
        let prof = results.0.unwrap_or_default();
        Self {
            url: prof.url,
            name: prof.personaname,
            steamid: prof.steamid,
            description: prof.summary,
            profilepic: prof.imgurl,
            level: prof.lvl.parse().unwrap_or_default(),
            stats: results.1.unwrap_or_default(),
            inventory: results.2.unwrap_or_default(),
        }
    }
    /// Prints the profile information in a formatted table.
    ///
    /// This method uses the `tabled` crate to create a nicely formatted table displaying the profile information.
    ///
    /// # Arguments
    ///
    /// None
    ///
    #[cfg(feature = "print")]
    pub fn print_profile(self) {
        use owo_colors::OwoColorize;
        use tabled::{col, row, Table};
        let table1 = Table::new(self.stats)
            .with(tabled::settings::Style::modern())
            .to_owned();
        let table2 = Table::new(self.inventory)
            .with(tabled::settings::Style::modern())
            .to_owned();
        let mut table3 = row![col!["Statistics", table1], col!["Inventory", table2]];
        println!(
            "\nName: {}, \nLevel: {}, \nSteamID: {}, \nUrl: {}, \nProfile Picture: {}, \nDescription: {}, \n{}",
            self.name.red(),
            self.level.purple(),
            self.steamid.yellow(),
            self.url.green(),
            self.profilepic.cyan(),
            self.description.blue(),
            table3.with(tabled::settings::Style::modern()).green()
        )
    }
}
