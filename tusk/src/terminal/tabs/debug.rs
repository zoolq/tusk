use ratatui::prelude::*;

use crate::terminal::{
	modules::{debug::log::draw_log, tracked::error::draw_error},
	App,
};

pub fn window_debug<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {}
