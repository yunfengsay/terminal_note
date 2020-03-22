extern crate eventual;
extern crate chrono;
extern crate termion;
use eventual::Timer;
use std::sync::mpsc::Sender;
use chrono::{NaiveTime, Utc, DateTime, Local};
use termion::{color, style, terminal_size};
use termion::screen::*;
use std::process::{Command};
use std::io::{self, Write, stdout};
use std::time::Instant;
use crate::canvas::Point;

mod canvas;

fn main() {
	let mut screen = AlternateScreen::from(stdout());
    screen.flush().unwrap();
	let cols =  Command::new("tput").arg("cols").output().expect("failed to exec tput cols");
	let lines =  Command::new("tput").arg("lines").output().expect("failed to exec tpu lines");
	// println!("{:?} {:?}", cols.stdout, &lines.stdout);
    let timer = Timer::new();
    let ticks = timer.interval_ms(1000).iter();
	let size = terminal_size().unwrap();
	// println!("{:?}", size);
	let center_txt = String::from("Hello this is word's center");
	let center_txt_len = center_txt.chars().count() as u8;
	let x = size.0/2 as u16 - u16::from(center_txt_len)/2;
	let y = size.1/2 as u16;
	print!("{}",color::Fg(color::Red));		
	print!("{}", style::Bold);
	let tcanvas = canvas::Canvas::new(size.1 as usize,  size.0 as usize);
	let from = Point{row: 12, col: 12};
	let end =  Point{row: 24, col: 24};
	tcanvas.line_to(from, end);
	tcanvas.flush();
    for _ in ticks {
		// Command::new("clear").output().expect("clear error");
		// 清除屏幕
        // print!("{}[2J", 27 as char);
   	    // print!("{}{}", termion::clear::All, termion::cursor::Goto(x, y));
		// println!("{}", center_txt);
    }
}
