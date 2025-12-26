use crate::player::{TrackInfo};
use crate::lyrics::{LyricLine};
use image::DynamicImage;
use ratatui::layout::Rect;

use crate::theme::Theme;

pub struct App {
    pub theme: Theme,
    pub is_running: bool,
    pub track: Option<TrackInfo>,
    pub lyrics: Option<Vec<LyricLine>>,
    pub artwork: Option<DynamicImage>,
    // Button Hit Areas
    pub prev_btn: Rect,
    pub play_btn: Rect,
    pub next_btn: Rect,
    pub progress_rect: Rect,
    // (Rect, Timestamp in ms)
    pub lyrics_hitboxes: Vec<(Rect, u64)>,
    
    // Manual Scroll State (None = Auto-sync)
    pub lyrics_offset: Option<usize>,
    
    // Display Mode
    pub app_show_lyrics: bool,
    pub is_tmux: bool, // New field for layout logic
    pub is_compact: bool, // New field for Compact Mode
}



impl App {
    pub fn new(app_show_lyrics: bool, is_tmux: bool, is_compact: bool) -> Self {
        let theme = crate::theme::load_current_theme();
        
        Self {
            theme,
            is_running: true,
            track: None,
            lyrics: None,
            artwork: None,
            prev_btn: Rect::default(),
            play_btn: Rect::default(),
            next_btn: Rect::default(),
            progress_rect: Rect::default(),
            lyrics_hitboxes: Vec::new(),
            lyrics_offset: None,
            app_show_lyrics,
            is_tmux,
            is_compact,
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if self.prev_btn.contains((x, y).into()) {
            let _ = crate::player::Player::prev();
        } else if self.play_btn.contains((x, y).into()) {
            let _ = crate::player::Player::play_pause();
        } else if self.next_btn.contains((x, y).into()) {
             let _ = crate::player::Player::next();
        } else if self.progress_rect.contains((x, y).into()) {
            if let Some(track) = &self.track {
                if track.duration_ms > 0 {
                     let relative_x = x.saturating_sub(self.progress_rect.x);
                     let width = self.progress_rect.width.max(1);
                     let percent = relative_x as f64 / width as f64;
                     let target_sec = (track.duration_ms as f64 / 1000.0) * percent;
                     let _ = crate::player::Player::seek(target_sec);
                }
            }
        }
    }
}
