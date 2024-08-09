use crate::back_end::click_element::click_element;

use crate::full_stack::
{
    //sdl_events::choose,
    language::Translations,
};

//use crate::
//{
//    SEASON_OPTIONS,
//    SEASON_SELECTED,
//};

use fantoccini::{elements::Element, Client, Locator};





#[derive(Clone)]
struct Season 
{
    pub web_element: Element,
}

impl Season 
{
    async fn click_season(self, driver: &Client, error_message: &str) 
    {
        click_element(driver, self.web_element, error_message).await;
    }
}





async fn parse_seasons(driver: &Client) -> Vec<Season>
{
    //println!("{}", language.getting_seasons_misc_text);
    let season_css_selector = "div[data-season-id]";

    driver.wait().for_element(Locator::Css(season_css_selector)).await.unwrap();
    let season_items = driver.find_all(Locator::Css(season_css_selector)).await.unwrap();

    let mut seasons = Vec::new();
    for season_element in season_items 
    {
        let season = Season 
        {
            //text: season_element.html(true).await.unwrap(),
            web_element: season_element,
        };

        seasons.push(season);
    }

    seasons
}



pub async fn select_season(language: &Translations, driver: &Client)
{
        let seasons = parse_seasons(&driver).await;
        let season_selected = 0;

        seasons[season_selected].clone().click_season(&driver, language.click_season_err).await;
}
