use core::cell::RefCell;

use alloc::{
    rc::{Rc, Weak},
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    browser::Browser,
    display_item::DisplayItem,
    http::HttpResponse,
    renderer::{
        css::{
            cssom::{CssParser, StyleSheet},
            token::CssTokenizer,
        },
        dom::{api::get_style_content, node::Window},
        html::{parser::HtmlParser, token::HtmlTokenizer},
        layout::layout_view::LayoutView,
    },
};

#[derive(Debug, Clone)]
pub struct Page {
    browser: Weak<RefCell<Browser>>,
    frame: Option<Rc<RefCell<Window>>>,
    style: Option<StyleSheet>,
    layout_view: Option<LayoutView>,
    display_items: Vec<DisplayItem>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            browser: Weak::new(),
            frame: None,
            style: None,
            layout_view: None,
            display_items: Vec::new(),
        }
    }

    pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
        self.browser = browser;
    }

    pub fn receive_response(&mut self, response: HttpResponse) {
        self.create_frame(response.body());

        self.set_layout_view();
        self.paint_tree();
    }

    fn create_frame(&mut self, html: String) {
        let html_tokenizer = HtmlTokenizer::new(html);
        let frame = HtmlParser::new(html_tokenizer).construct_tree();
        let dom = frame.borrow().document();

        let style = get_style_content(dom);
        let css_tokenizer = CssTokenizer::new(style);
        let cssom = CssParser::new(css_tokenizer).parse_stylesheet();

        self.frame = Some(frame);
        self.style = Some(cssom);
    }
}
