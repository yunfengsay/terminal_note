use core::num::FpCategory::Infinite;
use chrono::ParseResult;
use std::cell::RefCell;

#[derive(Debug)]
pub(crate) struct Point{
    pub row: usize,
    pub col: usize,
}
#[derive(Debug)]
pub(crate) struct Canvas{
    pub row_size: usize,
    pub col_size: usize,
    map: RefCell<Vec<Vec<char>>>,
}

impl Canvas{
    pub fn new(row_size: usize, col_size: usize) -> Canvas {
        Canvas{
            row_size: row_size,
            col_size: col_size,
            map : RefCell::new(vec![vec![]; row_size]),
        }
    }
    // , from: Point, to: Point
    pub fn line_to(&self, from: Point, to: Point){
        // rust斜率无穷太麻烦了直接计算map
        let dlty =to.row - from.row ;
        let dltx = to.col - from.col;
        let k = if dltx == 0 {
            Infinite as usize
        } else {
           dlty/dltx
        };
        std::mem::replace(&mut &self.map.borrow_mut()[1][2], &'*');
        println!("not runing here  {:?}", "why?");
        // for irow in 0..dltx {
        //     let new_x = from.row + irow;
        //     let predirect_y = ((new_x*k) as f64).round() as usize;
        //     // &self.map.borrow_mut()[new_x].(predirect_y) = &" ";
        //     std::mem::replace(&mut &self.map.borrow_mut()[new_x][predirect_y], &'*');
        // };
        println!("😄  {:?}", &self);
        println!("😄  {:?}", from);
        println!("😄  {:?}", to);
    }

    pub fn flush(&self) {
        println!("😄  {:?}", "fresh");
    }
}