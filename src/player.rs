use std::process::Command;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub artwork_url: Option<String>,
    pub duration_ms: u64,
    pub position_ms: u64,
    pub state: PlayerState,
    pub source: String, // "Spotify" or "Music"
}

// Unified Player Struct
pub struct Player;

impl Player {
    /// Detect which player is active: "Spotify", "Music", or None.
    /// Prioritizes Spotify if both are running.
    pub fn detect_active_player() -> Option<&'static str> {
        if Self::is_app_running("Spotify") {
            Some("Spotify")
        } else if Self::is_app_running("Music") {
            Some("Music")
        } else {
            None
        }
    }

    fn is_app_running(app_name: &str) -> bool {
        let output = Command::new("pgrep")
            .arg("-x")
            .arg(app_name)
            .output();
        match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }

    /// Run an AppleScript command
    fn run_script(script: &str) -> Result<String> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .context("Failed to execute AppleScript")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("AppleScript error: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn get_current_track() -> Result<Option<TrackInfo>> {
        let app_name = match Self::detect_active_player() {
            Some(app) => app,
            None => return Ok(None),
        };

        // AppleScript Logic (Polylimorphic)
        // Note: 'artwork url' only works for Spotify. 
        // For Apple Music, we return empty string for Art and handle it in main.rs via Search API.
        
        let script = format!(r#"
            tell application "{}"
                if player state is stopped then
                    return "STOPPED"
                end if
                
                set tName to name of current track
                set tArtist to artist of current track
                set tAlbum to album of current track
                set tDuration to duration of current track -- in milliseconds (Music app uses seconds usually? No, seconds for Music app!)
                set tPosition to player position -- in seconds
                set tState to player state as string
                
                if "{}" is "Spotify" then
                    -- Spotify Duration is ms
                    set tArtwork to artwork url of current track
                    return tName & "|||" & tArtist & "|||" & tAlbum & "|||" & tDuration & "|||" & tPosition & "|||" & tState & "|||" & tArtwork
                else
                    -- Music App Duration is SECONDS usually? Let's check docs. Standard is seconds.
                    -- Actually Music App: duration is seconds.
                    -- Warning: Spotify: duration is ms.
                    
                    set tDurSec to duration of current track
                    set tDuration to tDurSec * 1000
                    return tName & "|||" & tArtist & "|||" & tAlbum & "|||" & tDuration & "|||" & tPosition & "|||" & tState & "|||" & "NONE"
                end if
            end tell
        "#, app_name, app_name);

        match Self::run_script(&script) {
            Ok(output) => {
                if output == "STOPPED" {
                    return Ok(None);
                }

                let parts: Vec<&str> = output.split("|||").collect();
                if parts.len() < 7 {
                    return Ok(None);
                }

                let position_secs: f64 = parts[4].replace(',', ".").parse().unwrap_or(0.0);
                
                let state = match parts[5] {
                    "playing" => PlayerState::Playing,
                    "paused" => PlayerState::Paused,
                    _ => PlayerState::Stopped,
                };
                
                // Duration parsing
                let duration_ms: u64 = parts[3].parse::<f64>().unwrap_or(0.0) as u64;

                Ok(Some(TrackInfo {
                    name: parts[0].to_string(),
                    artist: parts[1].to_string(),
                    album: parts[2].to_string(),
                    duration_ms,
                    position_ms: (position_secs * 1000.0) as u64,
                    state,
                    artwork_url: Some(parts[6].to_string()).filter(|s| !s.is_empty() && s != "NONE"),
                    source: app_name.to_string(),
                }))
            },
            Err(_) => Ok(None)
        }
    }

    pub fn play_pause() -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
            Self::run_script(&format!("tell application \"{}\" to playpause", app))?;
        }
        Ok(())
    }

    pub fn next() -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
            Self::run_script(&format!("tell application \"{}\" to next track", app))?;
        }
        Ok(())
    }

    pub fn prev() -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
             Self::run_script(&format!("tell application \"{}\" to previous track", app))?;
        }
        Ok(())
    }

    pub fn seek(position_secs: f64) -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
            Self::run_script(&format!("tell application \"{}\" to set player position to {}", app, position_secs))?;
        }
        Ok(())
    }

    pub fn volume_up() -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
             Self::run_script(&format!("tell application \"{}\" to set sound volume to (sound volume + 10)", app))?;
        }
        Ok(())
    }

    pub fn volume_down() -> Result<()> {
        if let Some(app) = Self::detect_active_player() {
            Self::run_script(&format!("tell application \"{}\" to set sound volume to (sound volume - 10)", app))?;
        }
        Ok(())
    }
}
