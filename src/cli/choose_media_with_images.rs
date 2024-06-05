use selthi::Selthi;

use crate::{media::Media, TRANSLATION, VIM_MODE};

pub fn choose_media_with_images(medias: Vec<Media>, images_path: Vec<String>) -> Result<Media, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();

    let options: Vec<String> = medias
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{} {}", index + 1, item.title))
        .collect();

    let help_msg = format!("{} {}", language.total_media_misc_text, options.len());

    let images_path = images_path.iter().map(String::as_str).collect();
    let options = options.iter().map(String::as_str).collect();

    print!("\x1B[2J\x1B[1;1H");
    let ans = Selthi::new(language.select_media_misc_text, options)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .with_images(images_path)
        .prompt();

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
