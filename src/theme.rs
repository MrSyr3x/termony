use ratatui::style::Color;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub name: String,
    pub base: Color,
    pub text: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub surface: Color,
    pub overlay: Color,
}

impl Theme {
    pub fn default() -> Self {
        // Fallback: Catppuccin Mocha
        Self {
            name: "Catppuccin Mocha".to_string(),
            base: Color::Rgb(30, 30, 46),      // #1e1e2e
            text: Color::Rgb(205, 214, 244),   // #cdd6f4
            red: Color::Rgb(243, 139, 168),    // #f38ba8
            green: Color::Rgb(166, 227, 161),  // #a6e3a1
            yellow: Color::Rgb(249, 226, 175), // #f9e2af
            blue: Color::Rgb(137, 180, 250),   // #89b4fa
            magenta: Color::Rgb(203, 166, 247),// #cba6f7
            cyan: Color::Rgb(148, 226, 213),   // #94e2d5
            surface: Color::Rgb(49, 50, 68),   // #313244 (Surface0)
            overlay: Color::Rgb(108, 112, 134),// #6c7086 (Overlay0)
        }
    }
}

pub fn load_current_theme() -> Theme {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    
    // 1. Get Current Theme ID
    let cache_path = PathBuf::from(&home).join(".cache/current-theme");
    let theme_id = if let Ok(content) = fs::read_to_string(&cache_path) {
        content.trim().to_string()
    } else {
        return Theme::default();
    };

    // 2. Read Definitions
    let definitions_path = PathBuf::from(&home).join(".dotfiles/theme-selector/themes.sh");
    let content = match fs::read_to_string(&definitions_path) {
        Ok(c) => c,
        Err(_) => return Theme::default(),
    };

    // 3. Parse Shell Script
    // Looking for: THEMES[theme_id]="Name|Base|..."
    let key = format!("THEMES[{}]=", theme_id);
    
    for line in content.lines() {
        if let Some(pos) = line.find(&key) {
            // Extract value inside quotes
            let remainder = &line[pos + key.len()..];
            let raw_value = remainder.trim_matches('"');
            
            let parts: Vec<&str> = raw_value.split('|').collect();
            if parts.len() >= 11 {
                return Theme {
                    name: parts[0].to_string(),
                    base: parse_hex(parts[1]),
                    text: parse_hex(parts[2]),
                    red: parse_hex(parts[3]),
                    green: parse_hex(parts[4]),
                    yellow: parse_hex(parts[5]),
                    blue: parse_hex(parts[6]),
                    magenta: parse_hex(parts[7]),
                    cyan: parse_hex(parts[8]),
                    surface: parse_hex(parts[9]),
                    overlay: parse_hex(parts[10]),
                };
            }
        }
    }

    Theme::default()
}

fn parse_hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color::Rgb(r, g, b)
    } else {
        Color::Reset
    }
}
