use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};
use thirtyfour::prelude::*;
use selthi::Select;
use crate::driver::click_element::click_element;
use crate::{TRANSLATION, VIM_MODE};




#[derive(Clone)]
pub struct Season {
    pub text: String,
    pub web_element: WebElement,
}

impl Season {
    pub async fn click_season(self, driver: &WebDriver, error_message: &str) -> WebDriverResult<()> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}



pub fn choose_season(seasons: Vec<&str>) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_season_misc_text, seasons.len());

    let ans = Select::new(language.select_season_misc_text, seasons)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .prompt();

    match ans {
        Some(opt) => {
            let number = opt.split('º').next().unwrap();
            Ok(number.parse::<usize>().unwrap())
        }
        None => Err(println!("{}", language.choose_season_err)),
    }
}



pub async fn parse_seasons(driver: &WebDriver) -> WebDriverResult<Vec<Season>> {
    let season_items = driver.find_all(By::Css("div[data-season-id]")).await?;
    let mut seasons = Vec::new();

    for season_element in season_items {
        let season = Season {
            text: season_element.inner_html().await?,
            web_element: season_element,
        };
        seasons.push(season);
    }

    Ok(seasons)
}




