/* #![allow(warnings)] */


// MODULES
mod keyboard;
pub use crate::keyboard::keyboard_md;

mod commands;
mod print;
mod state;


fn main()
{
  keyboard_md::start();
}
