use core::cell::RefCell;

use alloc::{rc::Rc, string::ToString};
use noli::window::Window;
use saba_core::{
    browser::Browser,
    constants::{WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS},
};

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser: browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_HEIGHT,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }
}
