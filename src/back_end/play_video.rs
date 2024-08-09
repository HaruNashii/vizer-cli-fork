use std::process::Command;
use crate::TRANSLATION;
use crate::USE_MPV;





pub fn play_video(video_url: &str) 
{
    let use_mpv = USE_MPV.get().unwrap();

    if *use_mpv 
    {
        open_mpv(video_url);
    }
    else 
    {
        open_vlc(video_url);
    }
}



fn open_vlc(video_url: &str) 
{
    let language = TRANSLATION.get().unwrap();

    //println!("{}", language.players_start_misc_text);
    let output = Command::new("vlc").args(["--fullscreen", "--play-and-exit", video_url]).spawn();

    match output 
    {
        Ok(mut child) => match child.wait() 
        {
            Ok(status) => 
            {
                if status.success() 
                {
                    print!("\x1B[2J\x1B[1;1H");
                }
                else 
                {
                    println!("{} {:?}", language.vlc_exit_with_err, status.code());
                }
            }

            Err(err) => 
            {
                println!("{} {}", language.vlc_wait_err, err);
            }
        },


        Err(err) => 
        {
            println!("{} {}", language.vlc_start_err, err);
        }
    }
}



fn open_mpv(video_url: &str) {
    let language = TRANSLATION.get().unwrap();

    //println!("{}", language.players_start_misc_text);
    let output = Command::new("mpv").args(["--fs", "--really-quiet", video_url]).spawn();

    match output 
    {
        Ok(mut child) => match child.wait() 
        {
            Ok(status) => 
            {
                if status.success() 
                {
                    print!("\x1B[2J\x1B[1;1H");
                }
                else 
                {
                    println!("{} {:?}", language.mpv_exit_with_err, status.code());
                }
            }

            Err(err) => 
            {
                println!("{} {}", language.mpv_wait_err, err);
            }
        },


        Err(err) => 
        {
            println!("{} {}", language.mpv_start_err, err);
        }
    }
}