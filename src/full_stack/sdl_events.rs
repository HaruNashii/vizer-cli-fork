use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::process::exit;





pub fn search(event_pump: &mut sdl2::EventPump) -> String 
{   
    let mut input_text = String::new();


    loop 
    {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", input_text);


        for event in event_pump.poll_iter() 
        {
            match event 
            {
                //===============================================================================================================//
                //------------------------------------------------USER INPUT (KEYBOARD)------------------------------------------//
                //===============================================================================================================//
                Event::TextInput{text, .. } =>
                {
                    input_text.push_str(&text);
                }

                
               |Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                {
                    if !input_text.is_empty() {
                        input_text.pop();
                    }
                }


               Event::KeyDown{keycode: Some(Keycode::Return), .. } => 
               {


                    if input_text.len() < 4 
                    {
                        println!("not enough letters, please write more than 4 letters");
                    }
                    else 
                    {
                        return input_text;
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
        print!("\x1B[2J\x1B[1;1H");
        println!("\n amount of choices = {}, \n selected = {} \n", amount_limit, selected);
     

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
    for event in event_pump.poll_iter() 
    {
        match event 
        {
            //===============================================================================================================//
            //------------------------------------QUIT EVENT & QUIT KEYCHECKER (KEYBOARD)------------------------------------//
            //===============================================================================================================//
            sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => 
            {
                exit(0);
            }


            _ => {}
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
