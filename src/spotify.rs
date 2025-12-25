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
}

pub struct Spotify;

impl Spotify {
    /// Check if Spotify is running
    pub fn is_running() -> bool {
        let output = Command::new("pgrep")
            .arg("-x")
            .arg("Spotify")
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
        if !Self::is_running() {
            return Ok(None);
        }

        // Fetch all properties in one go for performance
        let script = r#"
            tell application "Spotify"
                if player state is stopped then
                    return "STOPPED"
                end if
                
                set tName to name of current track
                set tArtist to artist of current track
                set tAlbum to album of current track
                set tDuration to duration of current track -- in milliseconds
                set tPosition to player position -- in seconds (float)
                set tState to player state as string
                set tArtwork to artwork url of current track
                
                -- Delimiter for parsing
                return tName & "|||" & tArtist & "|||" & tAlbum & "|||" & tDuration & "|||" & tPosition & "|||" & tState & "|||" & tArtwork
            end tell
        "#;

        match Self::run_script(script) {
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

                Ok(Some(TrackInfo {
                    name: parts[0].to_string(),
                    artist: parts[1].to_string(),
                    album: parts[2].to_string(),
                    duration_ms: parts[3].parse().unwrap_or(0),
                    position_ms: (position_secs * 1000.0) as u64,
                    state,
                    artwork_url: Some(parts[6].to_string()).filter(|s| !s.is_empty()),
                }))
            },
            Err(_) => Ok(None)
        }
    }

    pub fn play_pause() -> Result<()> {
        Self::run_script("tell application \"Spotify\" to playpause")?;
        Ok(())
    }

    pub fn next() -> Result<()> {
        Self::run_script("tell application \"Spotify\" to next track")?;
        Ok(())
    }

    pub fn prev() -> Result<()> {
        Self::run_script("tell application \"Spotify\" to previous track")?;
        Ok(())
    }

    pub fn seek(position_secs: f64) -> Result<()> {
        Self::run_script(&format!("tell application \"Spotify\" to set player position to {}", position_secs))?;
        Ok(())
    }
}
