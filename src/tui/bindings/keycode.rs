#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    MediaPlay,
    MediaPause,
    MediaPlayPause,
    MediaReverse,
    MediaStop,
    MediaFastForward,
    MediaRewind,
    MediaTrackNext,
    MediaTrackPrevious,
    MediaRecord,
    MediaLowerVolume,
    MediaRaiseVolume,
    MediaMuteVolume,
    ModifierLeftShift,
    ModifierLeftControl,
    ModifierLeftAlt,
    ModifierLeftSuper,
    ModifierLeftHyper,
    ModifierLeftMeta,
    ModifierRightShift,
    ModifierRightControl,
    ModifierRightAlt,
    ModifierRightSuper,
    ModifierRightHyper,
    ModifierRightMeta,
    ModifierIsoLevel3Shift,
    ModifierIsoLevel5Shift,
}

impl<'de> serde::Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl From<KeyCode> for crossterm::event::KeyCode {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Backspace => crossterm::event::KeyCode::Backspace,
            KeyCode::Enter => crossterm::event::KeyCode::Enter,
            KeyCode::Left => crossterm::event::KeyCode::Left,
            KeyCode::Right => crossterm::event::KeyCode::Right,
            KeyCode::Up => crossterm::event::KeyCode::Up,
            KeyCode::Down => crossterm::event::KeyCode::Down,
            KeyCode::Home => crossterm::event::KeyCode::Home,
            KeyCode::End => crossterm::event::KeyCode::End,
            KeyCode::PageUp => crossterm::event::KeyCode::PageUp,
            KeyCode::PageDown => crossterm::event::KeyCode::PageDown,
            KeyCode::Tab => crossterm::event::KeyCode::Tab,
            KeyCode::BackTab => crossterm::event::KeyCode::BackTab,
            KeyCode::Delete => crossterm::event::KeyCode::Delete,
            KeyCode::Insert => crossterm::event::KeyCode::Insert,
            KeyCode::F1 => crossterm::event::KeyCode::F(1),
            KeyCode::F2 => crossterm::event::KeyCode::F(2),
            KeyCode::F3 => crossterm::event::KeyCode::F(3),
            KeyCode::F4 => crossterm::event::KeyCode::F(4),
            KeyCode::F5 => crossterm::event::KeyCode::F(5),
            KeyCode::F6 => crossterm::event::KeyCode::F(6),
            KeyCode::F7 => crossterm::event::KeyCode::F(7),
            KeyCode::F8 => crossterm::event::KeyCode::F(8),
            KeyCode::F9 => crossterm::event::KeyCode::F(9),
            KeyCode::F10 => crossterm::event::KeyCode::F(10),
            KeyCode::F11 => crossterm::event::KeyCode::F(11),
            KeyCode::F12 => crossterm::event::KeyCode::F(12),
            KeyCode::Char(chr) => crossterm::event::KeyCode::Char(chr),
            KeyCode::Null => crossterm::event::KeyCode::Null,
            KeyCode::Esc => crossterm::event::KeyCode::Esc,
            KeyCode::CapsLock => crossterm::event::KeyCode::CapsLock,
            KeyCode::ScrollLock => crossterm::event::KeyCode::ScrollLock,
            KeyCode::NumLock => crossterm::event::KeyCode::NumLock,
            KeyCode::PrintScreen => crossterm::event::KeyCode::PrintScreen,
            KeyCode::Pause => crossterm::event::KeyCode::Pause,
            KeyCode::Menu => crossterm::event::KeyCode::Menu,
            KeyCode::KeypadBegin => crossterm::event::KeyCode::KeypadBegin,
            KeyCode::MediaPlay => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Play)
            }
            KeyCode::MediaPause => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Pause)
            }
            KeyCode::MediaPlayPause => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::PlayPause)
            }
            KeyCode::MediaReverse => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Reverse)
            }
            KeyCode::MediaStop => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Stop)
            }
            KeyCode::MediaFastForward => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::FastForward)
            }
            KeyCode::MediaRewind => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Rewind)
            }
            KeyCode::MediaTrackNext => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::TrackNext)
            }
            KeyCode::MediaTrackPrevious => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::TrackPrevious)
            }
            KeyCode::MediaRecord => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::Record)
            }
            KeyCode::MediaLowerVolume => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::LowerVolume)
            }
            KeyCode::MediaRaiseVolume => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::RaiseVolume)
            }
            KeyCode::MediaMuteVolume => {
                crossterm::event::KeyCode::Media(crossterm::event::MediaKeyCode::MuteVolume)
            }
            KeyCode::ModifierLeftShift => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftShift)
            }
            KeyCode::ModifierLeftControl => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftControl)
            }
            KeyCode::ModifierLeftAlt => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftAlt)
            }
            KeyCode::ModifierLeftSuper => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftSuper)
            }
            KeyCode::ModifierLeftHyper => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftHyper)
            }
            KeyCode::ModifierLeftMeta => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftMeta)
            }
            KeyCode::ModifierRightShift => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightShift)
            }
            KeyCode::ModifierRightControl => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightControl)
            }
            KeyCode::ModifierRightAlt => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightAlt)
            }
            KeyCode::ModifierRightSuper => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightSuper)
            }
            KeyCode::ModifierRightHyper => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightHyper)
            }
            KeyCode::ModifierRightMeta => {
                crossterm::event::KeyCode::Modifier(crossterm::event::ModifierKeyCode::RightMeta)
            }
            KeyCode::ModifierIsoLevel3Shift => crossterm::event::KeyCode::Modifier(
                crossterm::event::ModifierKeyCode::IsoLevel3Shift,
            ),
            KeyCode::ModifierIsoLevel5Shift => crossterm::event::KeyCode::Modifier(
                crossterm::event::ModifierKeyCode::IsoLevel5Shift,
            ),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeyCodeError {
    #[error("Cannot parse '{}' as key code", .0)]
    CannotParseToKeyCode(String),
}

impl std::str::FromStr for KeyCode {
    type Err = KeyCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "backspace" => Self::Backspace,
            "enter" => Self::Enter,
            "left" => Self::Left,
            "right" => Self::Right,
            "up" => Self::Up,
            "down" => Self::Down,
            "home" => Self::Home,
            "end" => Self::End,
            "pageup" => Self::PageUp,
            "pagedown" => Self::PageDown,
            "tab" => Self::Tab,
            "backtab" => Self::BackTab,
            "delete" => Self::Delete,
            "insert" => Self::Insert,
            "f1" => Self::F1,
            "f2" => Self::F2,
            "f3" => Self::F3,
            "f4" => Self::F4,
            "f5" => Self::F5,
            "f6" => Self::F6,
            "f7" => Self::F7,
            "f8" => Self::F8,
            "f9" => Self::F9,
            "f10" => Self::F10,
            "f11" => Self::F11,
            "f12" => Self::F12,
            "null" => Self::Null,
            "esc" => Self::Esc,
            "capslock" => Self::CapsLock,
            "scrolllock" => Self::ScrollLock,
            "numlock" => Self::NumLock,
            "printscreen" => Self::PrintScreen,
            "pause" => Self::Pause,
            "menu" => Self::Menu,
            "keypadbegin" => Self::KeypadBegin,
            "play" => Self::MediaPlay,
            "media_pause" => Self::MediaPause,
            "media_playpause" => Self::MediaPlayPause,
            "media_reverse" => Self::MediaReverse,
            "media_stop" => Self::MediaStop,
            "media_fastforward" => Self::MediaFastForward,
            "media_rewind" => Self::MediaRewind,
            "media_tracknext" => Self::MediaTrackNext,
            "media_trackprevious" => Self::MediaTrackPrevious,
            "media_record" => Self::MediaRecord,
            "media_lowervolume" => Self::MediaLowerVolume,
            "media_raisevolume" => Self::MediaRaiseVolume,
            "media_mutevolume" => Self::MediaMuteVolume,
            "modifier_leftshift" => Self::ModifierLeftShift,
            "modifier_leftcontrol" => Self::ModifierLeftControl,
            "modifier_leftalt" => Self::ModifierLeftAlt,
            "modifier_leftsuper" => Self::ModifierLeftSuper,
            "modifier_lefthyper" => Self::ModifierLeftHyper,
            "modifier_leftmeta" => Self::ModifierLeftMeta,
            "modifier_rightshift" => Self::ModifierRightShift,
            "modifier_rightcontrol" => Self::ModifierRightControl,
            "modifier_rightalt" => Self::ModifierRightAlt,
            "modifier_rightsuper" => Self::ModifierRightSuper,
            "modifier_righthyper" => Self::ModifierRightHyper,
            "modifier_rightmeta" => Self::ModifierRightMeta,
            "modifier_isolevel3shift" => Self::ModifierIsoLevel3Shift,
            "modifier_isolevel5shift" => Self::ModifierIsoLevel5Shift,
            other => {
                if other.len() == 1 {
                    Self::Char(other.chars().next().unwrap()) // Safe because of above check
                } else {
                    return Err(KeyCodeError::CannotParseToKeyCode(other.to_string()));
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::KeyCode;

    #[test]
    fn test_deserialize() {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct M {
            key: KeyCode,
        }

        let mapping = r#"key = 'h'"#;

        assert_eq!(
            M {
                key: KeyCode::Char('h')
            },
            toml::from_str(mapping).unwrap()
        );
    }
}
