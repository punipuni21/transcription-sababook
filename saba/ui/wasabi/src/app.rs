use core::cell::RefCell;

use alloc::rc::Rc;
use noli::window::Window;
use saba_core::browser::Browser;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}
