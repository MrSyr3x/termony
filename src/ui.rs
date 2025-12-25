use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Line, Text},
    widgets::{block::Title, Block, Paragraph, Borders, BorderType},
    Frame,
};
use crate::app::App;
use crate::spotify::PlayerState;

// Catppuccin Mocha Palette
#[allow(dead_code)]
struct Mocha;
#[allow(dead_code)]
impl Mocha {
    // ... exact palette as before ... (omitted for brevity in replacement if possible, but write_to_file needs full)
    const ROSEWATER: Color = Color::Rgb(245, 224, 220);
    const FLAMINGO: Color = Color::Rgb(242, 205, 205);
    const PINK: Color = Color::Rgb(245, 194, 231);
    const MAUVE: Color = Color::Rgb(203, 166, 247);
    const RED: Color = Color::Rgb(243, 139, 168);
    const MAROON: Color = Color::Rgb(235, 160, 172);
    const PEACH: Color = Color::Rgb(250, 179, 135);
    const YELLOW: Color = Color::Rgb(249, 226, 175);
    const GREEN: Color = Color::Rgb(166, 227, 161);
    const TEAL: Color = Color::Rgb(148, 226, 213);
    const SKY: Color = Color::Rgb(137, 220, 235);
    const SAPPHIRE: Color = Color::Rgb(116, 199, 236);
    const BLUE: Color = Color::Rgb(137, 180, 250);
    const LAVENDER: Color = Color::Rgb(180, 190, 254);
    const TEXT: Color = Color::Rgb(205, 214, 244);
    const SUBTEXT1: Color = Color::Rgb(186, 194, 222);
    const OVERLAY2: Color = Color::Rgb(147, 153, 178);
    const OVERLAY1: Color = Color::Rgb(127, 132, 156);
    const OVERLAY0: Color = Color::Rgb(108, 112, 134);
    const SURFACE2: Color = Color::Rgb(88, 91, 112);
    const SURFACE1: Color = Color::Rgb(69, 71, 90);
    const SURFACE0: Color = Color::Rgb(49, 50, 68);
    const BASE: Color = Color::Rgb(30, 30, 46);
    const MANTLE: Color = Color::Rgb(24, 24, 37);
    const CRUST: Color = Color::Rgb(17, 17, 27);
}

pub fn ui(f: &mut Frame, app: &mut App) {
    // Responsive Layout Check
    let area = f.area();
    // Mini Mode Rule: 
    let is_compressed = !app.show_lyrics || area.height < 40; 

    // 1. Unified Layout (Single Card + Footer)
    let main_constraints = vec![
        Constraint::Min(0),     // Unified Music+Lyrics Card (Takes all space)
        Constraint::Length(1),  // Footer
    ];

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_constraints)
        .split(area);

    // --- UNIFIED CARD ---
    let music_title = Title::from(Line::from(vec![
        Span::styled(" termony ", Style::default().fg(Mocha::CRUST).bg(Mocha::MAUVE))
    ]));

    let music_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(music_title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Mocha::MAUVE)) 
        .style(Style::default().bg(Color::Reset));
    
    let unified_area = music_block.inner(main_chunks[0]);
    f.render_widget(music_block, main_chunks[0]);

    // Inner Layout (Music Top, Lyrics Bottom)
    // Adjust artwork constraints based on compression
    let art_height = if is_compressed { 20 } else { 24 };

    let unified_constraints = vec![
        Constraint::Length(art_height), // 0: Artwork
        Constraint::Length(4),          // 1: Info 
        Constraint::Length(1),          // 2: Gauge (Row)
        Constraint::Length(1),          // 3: Time
        Constraint::Length(1),          // 4: Controls
        Constraint::Length(1),          // 5: Padding
        Constraint::Min(0),             // 6: Lyrics Area (Flexible)
    ];

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(unified_constraints)
        .split(unified_area);

    // 1. Artwork
    let artwork_area = content_chunks[0];
    if let Some(data) = &app.artwork {
        let mut lines = Vec::new();
        // We render what we can fit
        for row in data.iter().take(art_height as usize) {
            let mut spans = Vec::new();
            for (fg, bg) in row {
                spans.push(Span::styled(
                    "▀",
                    Style::default()
                        .fg(Color::Rgb(fg.0, fg.1, fg.2))
                        .bg(Color::Rgb(bg.0, bg.1, bg.2))
                ));
            }
            lines.push(Line::from(spans));
        }
        
        let artwork_widget = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(Block::default().style(Style::default().bg(Color::Reset)));
        f.render_widget(artwork_widget, artwork_area);

    } else {
       // Placeholder
       let text = if app.track.is_some() {
           "\n\n\n\n\n        Loading...".to_string()
       } else {
           "\n\n\n\n\n        ♪\n    No Album\n      Art".to_string()
       };
       let p = Paragraph::new(text)
           .alignment(Alignment::Center)
           .block(Block::default().style(Style::default().fg(Mocha::OVERLAY0).bg(Color::Reset)));
       f.render_widget(p, artwork_area);
    }

    // 2. Track Info
    if let Some(track) = &app.track {
        let info_text = vec![
            Line::from(Span::styled(
                format!("🎵 {}", track.name),
                Style::default().fg(Mocha::TEXT).add_modifier(Modifier::BOLD)
            )),
            Line::from(vec![
                Span::raw("🎤 "),
                Span::styled(&track.artist, Style::default().fg(Mocha::PINK)), 
            ]),
            Line::from(vec![
                Span::raw("💿 "),
                Span::styled(&track.album, Style::default().fg(Mocha::TEAL).add_modifier(Modifier::DIM)), 
            ]),
        ];
        
        let info = Paragraph::new(info_text)
            .alignment(Alignment::Center)
             .block(Block::default().style(Style::default().bg(Color::Reset)));
        f.render_widget(info, content_chunks[1]);

        // 3. Progress Gauge
        let gauge_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10), 
                Constraint::Percentage(80), 
                Constraint::Percentage(10), 
            ])
            .split(content_chunks[2])[1];

        let ratio = if track.duration_ms > 0 {
            track.position_ms as f64 / track.duration_ms as f64
        } else {
            0.0
        };
        
        let width = gauge_area.width as usize;
        let occupied_width = (width as f64 * ratio.min(1.0).max(0.0)) as usize;
        let fill_style = Style::default().fg(Mocha::MAUVE);
        let empty_style = Style::default().fg(Mocha::SURFACE2);
        
        let mut bar_spans: Vec<Span> = Vec::with_capacity(width);
        for i in 0..width {
             if i < occupied_width {
                if i >= occupied_width.saturating_sub(1) {
                    bar_spans.push(Span::styled("▓", fill_style));
                } else if i >= occupied_width.saturating_sub(2) {
                    bar_spans.push(Span::styled("▒", fill_style));
                } else {
                    bar_spans.push(Span::styled("█", fill_style));
                }
            } else {
                bar_spans.push(Span::styled("░", empty_style));
            }
        }

        let gauge_p = Paragraph::new(Line::from(bar_spans))
            .alignment(Alignment::Left)
            .block(Block::default().style(Style::default().bg(Color::Reset)));
        f.render_widget(gauge_p, gauge_area);
        
        app.progress_rect = gauge_area;

        // 4. Time
        let time_str = format!(
            "{:02}:{:02} / {:02}:{:02}",
            track.position_ms / 60000,
            (track.position_ms % 60000) / 1000,
            track.duration_ms / 60000,
            (track.duration_ms % 60000) / 1000
        );
        let time_label = Paragraph::new(time_str)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Mocha::OVERLAY1));
        f.render_widget(time_label, content_chunks[3]);

        // 5. Controls
        let play_icon = if track.state == PlayerState::Playing { "⏸" } else { "▶" };
        let btn_style = Style::default().fg(Mocha::TEXT).add_modifier(Modifier::BOLD);
        
        let prev_str = "   ⏮   ";
        let next_str = "   ⏭   ";
        let play_str = format!("   {}   ", play_icon); 
        
        let controls_text = Line::from(vec![
            Span::styled(prev_str, btn_style),
            Span::raw("   "), 
            Span::styled(play_str, btn_style),
            Span::raw("   "), 
            Span::styled(next_str, btn_style),
        ]);
        
        let controls = Paragraph::new(controls_text)
            .alignment(Alignment::Center)
            .block(Block::default().style(Style::default().bg(Color::Reset)));
        
        f.render_widget(controls, content_chunks[4]);

        let area = content_chunks[4];
        let mid_x = area.x + area.width / 2;
        let y = area.y;
        
        app.prev_btn = ratatui::layout::Rect::new(mid_x.saturating_sub(13), y, 7, 1);
        app.play_btn = ratatui::layout::Rect::new(mid_x.saturating_sub(3), y, 7, 1);
        app.next_btn = ratatui::layout::Rect::new(mid_x + 7, y, 7, 1);

        // --- LYRICS (Merged Inside) ---
        // Render only if !is_compressed
        app.lyrics_hitboxes.clear(); 
        let lyrics_area = content_chunks[6]; // Flexible area at bottom

        if !is_compressed && lyrics_area.height > 2 {
            // Optional: Draw a subtle separator line?
             let separator = Block::default()
                 .borders(Borders::TOP)
                 .border_style(Style::default().fg(Mocha::SURFACE1));
             f.render_widget(separator, lyrics_area);
             
             // Inner margin for text
             let inner_lyrics_area = lyrics_area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 });
            
            if let Some(lyrics) = &app.lyrics {
                let height = inner_lyrics_area.height as usize;
                if height > 0 {
                    let current_time = track.position_ms;
                    let current_idx = lyrics.iter()
                       .position(|l| l.timestamp_ms > current_time)
                       .map(|i| if i > 0 { i - 1 } else { 0 })
                       .unwrap_or(lyrics.len().saturating_sub(1));

                   let start_idx = if let Some(offset) = app.lyrics_offset {
                        offset.min(lyrics.len().saturating_sub(1))
                   } else {
                        let half_height = height / 2;
                        current_idx.saturating_sub(half_height)
                   };
                   
                   let end_idx = (start_idx + height).min(lyrics.len());
                   
                   let mut lines = Vec::new();
                   
                   for (offset, (i, line)) in lyrics.iter().enumerate().skip(start_idx).take(end_idx - start_idx).enumerate() {
                       let style = if i == current_idx {
                           Style::default().add_modifier(Modifier::BOLD).fg(Mocha::GREEN)
                       } else {
                           Style::default().fg(Mocha::OVERLAY0)
                       };
                       
                       let prefix = if i == current_idx { "● " } else { "  " };
                       let prefix_span = if i == current_idx {
                           Span::styled(prefix, Style::default().fg(Mocha::GREEN))
                       } else {
                            Span::styled(prefix, style)
                       };

                       lines.push(Line::from(vec![
                           prefix_span,
                           Span::styled(line.text.clone(), style)
                       ]));
                       
                       let line_y = inner_lyrics_area.y + offset as u16;
                       let hitbox = Rect::new(inner_lyrics_area.x, line_y, inner_lyrics_area.width, 1);
                       app.lyrics_hitboxes.push((hitbox, line.timestamp_ms));
                   }
                   
                   let lyrics_widget = Paragraph::new(lines)
                       .alignment(Alignment::Center)
                       .block(Block::default().style(Style::default().bg(Color::Reset)));
                       
                   f.render_widget(lyrics_widget, inner_lyrics_area);
                }
            } else {
                let no_lyrics = Paragraph::new(Text::styled("\nNo Lyrics Found", Style::default().fg(Mocha::OVERLAY0)))
                    .alignment(Alignment::Center)
                     .block(Block::default().style(Style::default().bg(Color::Reset)));
                 f.render_widget(no_lyrics, inner_lyrics_area);
            }
        }

        // --- FOOTER (Always Render) ---
        let desc_style = Style::default().fg(Mocha::SUBTEXT1); 
        
        let footer_text = Line::from(vec![
            Span::styled(" q ", Style::default().fg(Mocha::RED).add_modifier(Modifier::BOLD)), 
            Span::styled("Exit   ", desc_style),
            
            Span::styled("n ", Style::default().fg(Mocha::SAPPHIRE).add_modifier(Modifier::BOLD)), 
            Span::styled("Next   ", desc_style),
            
            Span::styled("p ", Style::default().fg(Mocha::SAPPHIRE).add_modifier(Modifier::BOLD)), 
            Span::styled("Prev   ", desc_style),
            
            Span::styled("Space ", Style::default().fg(Mocha::GREEN).add_modifier(Modifier::BOLD)), 
            Span::styled("Play/Pause", desc_style),
        ]);
        
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Right)
            .block(Block::default().style(Style::default().bg(Color::Reset)));
        f.render_widget(footer, main_chunks[1]);

    } else {
        // IDLE STATE
        let t = Paragraph::new("Music Paused / Not Running")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Mocha::TEXT))
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" music ").title_alignment(Alignment::Center).style(Style::default().bg(Color::Reset)));
        f.render_widget(t, main_chunks[0]);
    }
}
