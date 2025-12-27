use serde::Deserialize;
use ratatui::style::Color;
use std::fs;

// Copy-paste of theme.rs logic to be exact (since I can't easily link private modules in examples without pub)
// OR I can try to access the library if I made it a lib?
// Easier to just reproduce the code block to test validity.

#[derive(Clone, Debug, Deserialize)]
pub struct Theme {
    pub base: Color,
    pub surface: Color,
    pub overlay: Color,
    pub text: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            base: Color::Rgb(30, 30, 46), // Catppuccin Base
            // ... truncated for brevity, we know default is Catppuccin
            surface: Color::Rgb(49, 50, 68),
            overlay: Color::Rgb(108, 112, 134),
            text: Color::Rgb(205, 214, 244),
            red: Color::Rgb(243, 139, 168),
            green: Color::Rgb(166, 227, 161),
            yellow: Color::Rgb(249, 226, 175),
            blue: Color::Rgb(137, 180, 250),
            magenta: Color::Rgb(203, 166, 247),
            cyan: Color::Rgb(148, 226, 213),
        }
    }
}

// Helper for deserialization
#[derive(Deserialize)]
struct ThemeFile {
    theme: Theme,
}

pub fn load_current_theme() -> Theme {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = std::path::PathBuf::from(&home).join(".config/vyom/theme.toml");
    
    println!("Reading from: {:?}", path);

    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            println!("Content:\n{}", content);
            
            // Try parsing as nested [theme] first (Theme Selector format)
            match toml::from_str::<ThemeFile>(&content) {
                Ok(wrapper) => {
                    println!("Parsed as Nested ThemeFile!");
                    return wrapper.theme;
                },
                Err(e) => println!("Failed Nested: {}", e),
            }

            // Fallback: Try parsing as flat file (Manual/Legacy format)
            match toml::from_str::<Theme>(&content) {
                Ok(theme) => {
                    println!("Parsed as Flat Theme!");
                    return theme;
                },
                Err(e) => println!("Failed Flat: {}", e),
            }
        }
    } else {
        println!("File does not exist!");
    }
    println!("Falling back to Default");
    Theme::default()
}

fn main() {
    let theme = load_current_theme();
    println!("Loaded Base Color: {:?}", theme.base);
}
