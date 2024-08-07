use crate::back_end::click_element::click_element;
use fantoccini::{elements::Element, Client, Locator};





#[derive(Clone)]
pub struct Season 
{
    pub text: String,
    pub web_element: Element,
}

impl Season 
{
    pub async fn click_season(self, driver: &Client, error_message: &str) 
    {
        click_element(driver, self.web_element, error_message).await;
    }
}





pub async fn parse_seasons(driver: &Client) -> Vec<Season>
{
    let season_css_selector = "div[data-season-id]";

    driver.wait().for_element(Locator::Css(season_css_selector)).await.unwrap();
    let season_items = driver.find_all(Locator::Css(season_css_selector)).await.unwrap();

    let mut seasons = Vec::new();
    for season_element in season_items 
    {
        let season = Season 
        {
            text: season_element.html(true).await.unwrap(),
            web_element: season_element,
        };

        seasons.push(season);
    }


    seasons
}

