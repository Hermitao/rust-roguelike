#![allow(unused_imports)] 
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::thread;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    ExecutableCommand,
    QueueableCommand,
    event,
    queue,
    terminal::{ClearType, Clear},
    cursor,
};


struct Position {
    x: u16,
    y: u16,
}

struct Player {
    
}


fn print_events() -> std::io::Result<()> {
    loop {
        // `read()` blocks until an `Event` is available
        match event::read()? {
            event::Event::FocusGained => println!("FocusGained"),
            event::Event::FocusLost => println!("FocusLost"),
            event::Event::Key(event) => println!("{:?}", event),
            event::Event::Mouse(event) => println!("{:?}", event),
            //#[cfg(feature = "bracketed-paste")]
            event::Event::Paste(data) => println!("{:?}", data),
            event::Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let map = r#"
####################################################################
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
#..................................................................#
####################################################################
    "#;

    let player_pos = Position {
        x: 4,
        y: 4,
    };

    let mut stdout = stdout();
   
    execute!(
        stdout,
        Clear(ClearType::All),
    )?;


    thread::spawn(|| {
        print_events();
    });

    const target_fps: f32 = 12.0;

    let mut last_frame_time = Instant::now();
    let mut timer: f32 = 0.0;

    loop {
        let current_time = Instant::now();

        let delta_time = current_time.duration_since(last_frame_time);
        let delta_time_secs = delta_time.as_secs_f32();

        last_frame_time = current_time;

        timer += delta_time_secs;

        if timer >= 1.0 / target_fps
        {
            queue!(
                stdout,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print(map),
                cursor::MoveTo(player_pos.x, player_pos.y),
                Print("@"),
            );

            stdout.flush();
            timer = 0.0;
        }
    }
    

    Ok(())
}
