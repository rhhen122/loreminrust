use std::{thread, time::Duration};
use colored::Colorize;

fn sleep() {
    thread::sleep(Duration::from_millis(0256));
}

fn main() {
    let mut _x: i64 = 0;
    let mut _y: i64 = 0;

    _x += 1;
    _y -= 1;

    clearscreen::clear().unwrap();

    loop {
        println!("{} {}",
            "lorem- 3 ---        | Ipsum".purple(),
            _x.to_string().purple()
        );

        _x += 1;
        _y -= 1;

        sleep();

        println!("{} {}",
            "Ipsum-- . ---        | Lorem".green(),
            _y.to_string().green()
        );

        _x += 1;
        _y -= 1;

        sleep();

        println!("{} {}{}",
            "Lorem Ipsum--- 1 ----------------".yellow(),
            _x.to_string().yellow(),
            ", ć, ę, ł, ń, ó, ś, ź, ż".green().italic()
        );
        
        _x += 1;
        _y -= 1;

        sleep();

        println!("{} {} {}",
            "Ipsum Lorem---- 4 ----------------".red(),
            _y.to_string().green(),
            "愛 時 愛 時".green().italic()
        );

        _x += 1;
        _y -= 1;

        sleep();

        println!("{} {} {}",
            "π-----π 3.1415926535897932384626433832795028841971693993751058209749 --- !@#$%^&*()-=+".truecolor(128, 128, 128),
            _x.to_string().truecolor(128, 128, 128), _y.to_string().truecolor(128, 128, 128)
        );

        _x += 1;
        _y -= 1;

        sleep();
    }
}