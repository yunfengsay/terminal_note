extern crate eventual;
extern crate chrono;
extern crate termion;
use termion::terminal_size;
use eventual::Timer;
use std::sync::mpsc::Sender;
use chrono::{NaiveTime, Utc, DateTime, Local};
use termion::{color, style};
use std::process::{Command};
use std::io::{self, Write};
use std::time::Instant;


fn main() {
	let cols =  Command::new("tput").arg("cols").output().expect("failed to exec tput cols");
	let lines =  Command::new("tput").arg("lines").output().expect("failed to exec tpu lines");
	println!("{:?} {:?}", cols.stdout, &lines.stdout);
    let timer = Timer::new();
    let ticks = timer.interval_ms(1000).iter();
    for _ in ticks {
		print!("{}",color::Fg(color::Red));		
		print!("{}", style::Bold);		
		// 清除屏幕
        print!("{}[2J", 27 as char);
		let size = terminal_size().unwrap();
		// println!("{:?}", size);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(size.0/2 as u16, size.1 / 2 as u16));
		println!("Hello this is word's center");
    }
}
