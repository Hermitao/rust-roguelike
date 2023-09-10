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



fn main() -> std::io::Result<()> {
    /*// using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;*/

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

    let playerPos = Position {
        x: 4,
        y: 4,
    };

    let mut stdout = stdout();
   
    execute!(
        stdout,
        Clear(ClearType::All),
    )?;

    const desiredFps: f32 = 60.0;

    let mut lastFrameTime = Instant::now();
    let mut timer: f32 = 0.0;
    loop {
        let currentTime = Instant::now();

        let deltaTime = currentTime.duration_since(lastFrameTime);
        let deltaTimeSecs = deltaTime.as_secs_f32();

        lastFrameTime = currentTime;

        timer += deltaTimeSecs;

        if (timer >= 1.0 / desiredFps)
        {
            queue!(
                stdout,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print((map)),
                cursor::MoveTo(playerPos.x, playerPos.y),
                Print(("@")),
                Print(( timer )),
            )?;

            stdout.flush();
            timer = 0.0;
        }

    }


    Ok(())
}
