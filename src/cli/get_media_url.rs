use thirtyfour::prelude::*;

use crate::{cli::choose_lang::choose_lang, TRANSLATION};

pub async fn get_media_url(driver: &WebDriver) -> WebDriverResult<String> {
    let language = TRANSLATION.get().unwrap();
    println!("{}", language.getting_language_misc_text);

    let langs_items = driver.query(By::Css("div[data-audio]")).all().await?;

    let mut langs_opts: Vec<String> = Vec::new();

    for lang in &langs_items {
        let opt = lang
            .attr("data-audio")
            .await?
            .expect(language.language_option_expect);
        langs_opts.push(opt);
    }

    let lang_opt = if langs_opts.len() == 2 {
        choose_lang(langs_opts.clone()).unwrap()
    } else {
        langs_opts[0].to_string()
    };

    let mut media_id: Option<String> = None;
    for i in 0..langs_opts.len() {
        if langs_opts[i] == lang_opt {
            media_id = langs_items[i].attr("data-load-player").await?;
        }
    }

    println!("{}", language.fetching_misc_text);

    let media_url = format!(
        "https://vizer.in/embed/getEmbed.php?id={}&sv=mixdrop",
        media_id.unwrap()
    );

    Ok(media_url)
}
