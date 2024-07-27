use core::fmt;
use selthi::Select;
use selthi::Input;
use thirtyfour::prelude::*;
use scraper::{Selector, Html};
use crate::{
    quit::quit,
    driver::click_element::click_element,
    cli::choose_lang::choose_lang,

    TRANSLATION,
    VIM_MODE
};



#[derive(Clone)]
pub struct Media {
    pub title: String,
    pub url: String,
    pub poster_url: String,
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title - {} ; Link - {} ; Poster Url - {}",
            self.title, self.url, self.poster_url
        )
    }
}



pub fn choose_media(medias: Vec<Media>, img_mode: bool, images_path: Vec<String>) -> Result<Media, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();

    let options: Vec<String> = medias
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{} {}", index + 1, item.title))
        .collect();

    let help_msg = format!("{} {}", language.total_media_misc_text, options.len());
    let options = options.iter().map(String::as_str).collect();

    print!("\x1B[2J\x1B[1;1H");

    let ans = if img_mode {
        let images_path = images_path.iter().map(String::as_str).collect();

        Select::new(language.select_media_misc_text, options)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(*vim_mode)
            .with_images(images_path)
            .prompt()
    } else {
        Select::new(language.select_media_misc_text, options)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(*vim_mode)
            .prompt()
    };

    match ans {
        Some(choice) => {
            let mut media_index = choice.split_whitespace();
            let index: usize = media_index.next().unwrap().parse::<usize>().unwrap();
            let media = medias[index - 1].clone();
            Ok(media)
        }
        None => Err(println!("{}", language.choose_media_err)),
    }
}



pub async fn get_medias(media_name: &str) -> Vec<Media> {
    let language = TRANSLATION.get().unwrap();

    let url = format!("https://vizer.in/pesquisar/{}", media_name);
    let response = reqwest::get(url).await.expect(language.response_expect);
    let html = response.text().await.unwrap();

    let document = Html::parse_document(html.as_str());

    let list_posters_selector = Selector::parse(r#"div[class="listItems"]"#).unwrap();
    let img_selector = Selector::parse(r#"img[class="img"]"#).unwrap();
    let poster_selector = Selector::parse("a").unwrap();

    let list_media = document.select(&list_posters_selector).next().unwrap();
    let mut medias = Vec::new();

    for (poster, img) in list_media
        .select(&poster_selector)
        .zip(list_media.select(&img_selector))
    {
        let media_title = poster
            .value()
            .attr("title")
            .unwrap()
            .replace("Assistir ", "")
            .replace(" online", "");

        let media_url = poster.value().attr("href").unwrap();
        let img_src = img.value().attr("src").unwrap();

        // we replace the "size" of the image in the url
        // to improve the quality of the image
        let img_url = format!("https://vizertv.in{}", img_src.replace("t/185", "t/342"));

        let media = Media {
            title: media_title,
            url: media_url.to_string(),
            poster_url: img_url,
        };

        medias.push(media);
    }

    medias
}



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



pub async fn get_video_url(driver: &WebDriver, media_url: String) -> WebDriverResult<String> {
    let language = TRANSLATION.get().unwrap();

    driver.goto(media_url).await?;
    driver.enter_frame(0).await?;

    let play_button = driver
        .query(By::ClassName("vjs-big-play-button"))
        .first()
        .await?;

    click_element(driver, play_button, language.click_episode_err).await?;

    let video = driver.find(By::Id("videojs_html5_api")).await?;
    
    let mut video_attr_src = String::new();
    match video.attr("src").await? {
        Some(string) => {
            video_attr_src.push_str(&string);
        },
        None => quit(Some(driver.to_owned()), None, Some(&format!("video_attr_src ERROR, video_attr_src output = {}", video_attr_src))).await,
    };

    let video_url = format!("https:{}", video_attr_src);

    Ok(video_url)
}



pub fn get_media_name_from_user() -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();

    let ans: Option<String> = Input::new(language.select_media_misc_text)
        .with_minimum_chars(4)
        .prompt();

    match ans {
        Some(media_name) => Ok(media_name),
        None => Err(println!("{}", language.choose_media_err)),
    }
}
