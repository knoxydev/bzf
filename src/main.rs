#![allow(warnings)]


// MODULES
mod init;
pub use crate::init::init_md;

mod keyboard;
pub use crate::keyboard::keyboard_md;

mod print;


use std::env;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


fn main()
{
	keyboard_md::start();
}
