use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::process::exit;





pub static mut INPUT_TEXT: String = String::new();





pub fn search(event_pump: &mut sdl2::EventPump) -> String
{   
    loop 
    {
        for event in event_pump.poll_iter()
        {
            match event 
            {
                //===============================================================================================================//
                //------------------------------------------------USER INPUT (KEYBOARD)------------------------------------------//
                //===============================================================================================================//
                Event::TextInput{text, .. } =>
                {
                    unsafe{INPUT_TEXT.push_str(&text)};
                }

                
               |Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                {
                    unsafe 
                    {
                        if !INPUT_TEXT.is_empty() 
                        {
                            INPUT_TEXT.pop();
                        }
                    }
                }


               Event::KeyDown{keycode: Some(Keycode::Return), .. } => 
               {


                    if unsafe{INPUT_TEXT.len()} < 4
                    {
                        println!("not enough letters, please write more than 4 letters");
                    } 
                    else 
                    {   
                        return unsafe{INPUT_TEXT.clone()};
                    }
               }
                

               _ => {}
            }
        }
    };
}



pub fn choose(amount_limit: usize, event_pump: &mut sdl2::EventPump) -> usize 
{
    let mut selected: usize = 0;
 

    loop 
    {
        println!("\n ========================== \n amount of choices = {},\n ========================== \n  selected = {} \n ========================== \n", amount_limit, selected + 1);
     

        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => 
                {
                    if selected + 1 <= amount_limit
                    {
                        selected += 1;
                    }
                }


                Event::KeyDown { keycode: Some(Keycode::Up), .. } => 
                {
                    if selected >= 1
                    {
                        selected -= 1;
                    }
                }


                Event::KeyDown { keycode: Some(Keycode::Return), .. } => 
                {
                    return selected;
                }


                _ => {}
            }
        }
    }
}   



pub fn quit(event_pump: &mut sdl2::EventPump)
{
    loop 
    {
        for event in event_pump.poll_iter()
        {
            match event 
            {
                //===============================================================================================================//
                //------------------------------------QUIT EVENT & QUIT KEYCHECKER (KEYBOARD)------------------------------------//
                //===============================================================================================================//
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
                {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("bye bye :3");
                    exit(0);
                }


                _ => {}
            }
        }
    }
}



// pub fn sdl_events(event_pump: &mut sdl2::EventPump)
// {
//     for event in event_pump.poll_iter() 
//     {
//         match event 
//         {
// 
//             //===============================================================================================================//
//             //------------------------------------------------------MOUSE CHECKER--------------------------------------------//
//             //===============================================================================================================//
//             // Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => {
//             //     if x >= buttons_rect_vec[0].x && x <= buttons_rect_vec[0].x + DEFAULT_BUTTON_SIZE[0] && y >= buttons_rect_vec[0].y && y <= buttons_rect_vec[0].y + DEFAULT_BUTTON_SIZE[1] {
//             //         shuffle_toggle_set();
//             //     }
//             // }
// 
//             //===============================================================================================================//
//             //------------------------------------------------KEYCHECKER (KEYBOARD)------------------------------------------//
//             //===============================================================================================================//
//             // Event::KeyDown { keycode: Some(Keycode::Space), .. } | Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
//             //     PlayerCtl::play_pause();
//             // }
//             
//             _ => {}
//         }
//     }
// }
