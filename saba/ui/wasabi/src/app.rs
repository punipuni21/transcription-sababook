use core::cell::RefCell;

use alloc::{rc::Rc, string::ToString};
use noli::error::Result as OsResult;
use noli::window::{StringSize, Window};
use saba_core::{
    browser::Browser,
    constants::{
        ADDRESSBAR_HEIGHT, BLACK, DARKGREY, GREY, LIGHTGREY, TOOLBAR_HEIGHT, WHITE, WINDOW_HEIGHT,
        WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH,
    },
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

    fn setup_toolbar(&mut self) -> OsResult<()> {
        self.window
            .fill_rect(LIGHTGREY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        self.window
            .draw_line(GREY, 0, TOOLBAR_HEIGHT, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT)?;

        self.window.draw_line(
            DARKGREY,
            0,
            TOOLBAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT + 1,
        )?;

        self.window
            .draw_string(BLACK, 5, 5, "Adress:", StringSize::Medium, false)?;

        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESSBAR_HEIGHT)?;

        self.window.draw_line(GREY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window
            .draw_line(GREY, 70, 2, 70, 2 + ADDRESSBAR_HEIGHT)?;

        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;
        self.window
            .draw_line(GREY, 71, 3, 71, 1 + ADDRESSBAR_HEIGHT)?;
        Ok(())
    }
}
