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

        sleep();

        println!("{} {}",
            "Ipsum-- . ---        | Lorem".green(),
            _y.to_string().green()
        );

        sleep();

        println!("{} {} {}",
            "Lorem Ipsum--- 1 ---------------- {deadx}  \033[32mą, ć, ę, ł, ń, ó, ś, ź, ż".yellow(),
            _x.to_string().yellow(),
            ", ć, ę, ł, ń, ó, ś, ź, ż".green().italic()
        );

        sleep();

        println!("{} {} {}",
            "Ipsum Lorem---- 4 ----------------".red(),
            _y.to_string().green(),
            "愛 時 愛 時".green().italic()
        );

        sleep();

        println!("{}",
       "π-----π 3.1415926535897932384626433832795028841971693993751058209749 --- !@#$%^&*()-=+".truecolor(128, 128, 128)
        );

        sleep();
    }
}