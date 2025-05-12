use crate::renderer::css::token::CssToken;
use core::iter::Peekable;

use super::token::CssTokenizer;

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: t.peekable() }
    }
}
