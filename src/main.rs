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
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
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
    let map = r#"####################################################################
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

    let mut player_pos = Position {
        x: 4.0,
        y: 25.0,
    };

    let mut player_velocity = Velocity {
        x: 8.0,
        y: -50.0,
    };


    let gravity: f32 = 60.0;
    let ground_pos = 26;

    let mut stdout = stdout();
   
    execute!(
        stdout,
        Clear(ClearType::All),
    )?;


    thread::spawn(|| {
        print_events();
    });

    const target_fps: f32 = 120.0;

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

            player_velocity.y += gravity * timer;

            player_pos.x += player_velocity.x * timer;
            player_pos.y += player_velocity.y * timer;

            if (player_pos.y as u16 > ground_pos - 1)
            {
                player_pos.y = (ground_pos - 1) as f32;
                player_velocity.y = 0.0;
            }

            queue!(
                stdout,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print(map),
                cursor::MoveTo(player_pos.x as u16, player_pos.y as u16),
                Print("@"),
                cursor::MoveTo(90, 0),
                Print(player_pos.y),
                cursor::MoveTo(90, 1),
                Print(timer),
            );

            stdout.flush();
            timer = 0.0;
        }
    }
    

    Ok(())
}
