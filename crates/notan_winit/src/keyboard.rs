use notan_core::events::Event;
use notan_core::keyboard::KeyCode;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode as WKeyCode;

pub fn process_events(event: &WindowEvent) -> Option<Event> {
    match event {
        WindowEvent::KeyboardInput {
            device_id,
            event,
            is_synthetic,
        } => {
            let key = keyboard_button_to_nae(event.physical_key.as_ref());
            let evt = match event.state {
                ElementState::Pressed => Event::KeyDown { key },
                _ => Event::KeyUp { key },
            };

            Some(evt)
        }
        _ => None,
    }
}

fn keyboard_button_to_nae(key: Option<&WKeyCode>) -> KeyCode {
    match key {
        Some(k) => match k {
            WKeyCode::Digit1 => KeyCode::Key1,
            WKeyCode::Digit2 => KeyCode::Key2,
            WKeyCode::Digit3 => KeyCode::Key3,
            WKeyCode::Digit4 => KeyCode::Key4,
            WKeyCode::Digit5 => KeyCode::Key5,
            WKeyCode::Digit6 => KeyCode::Key6,
            WKeyCode::Digit7 => KeyCode::Key7,
            WKeyCode::Digit8 => KeyCode::Key8,
            WKeyCode::Digit9 => KeyCode::Key9,
            WKeyCode::Digit0 => KeyCode::Key0,
            WKeyCode::KeyA => KeyCode::A,
            WKeyCode::KeyB => KeyCode::B,
            WKeyCode::KeyC => KeyCode::C,
            WKeyCode::KeyD => KeyCode::D,
            WKeyCode::KeyE => KeyCode::E,
            WKeyCode::KeyF => KeyCode::F,
            WKeyCode::KeyG => KeyCode::G,
            WKeyCode::KeyH => KeyCode::H,
            WKeyCode::KeyI => KeyCode::I,
            WKeyCode::KeyJ => KeyCode::J,
            WKeyCode::KeyK => KeyCode::K,
            WKeyCode::KeyL => KeyCode::L,
            WKeyCode::KeyM => KeyCode::M,
            WKeyCode::KeyN => KeyCode::N,
            WKeyCode::KeyO => KeyCode::O,
            WKeyCode::KeyP => KeyCode::P,
            WKeyCode::KeyQ => KeyCode::Q,
            WKeyCode::KeyR => KeyCode::R,
            WKeyCode::KeyS => KeyCode::S,
            WKeyCode::KeyT => KeyCode::T,
            WKeyCode::KeyU => KeyCode::U,
            WKeyCode::KeyV => KeyCode::V,
            WKeyCode::KeyW => KeyCode::W,
            WKeyCode::KeyX => KeyCode::X,
            WKeyCode::KeyY => KeyCode::Y,
            WKeyCode::KeyZ => KeyCode::Z,
            WKeyCode::Escape => KeyCode::Escape,
            WKeyCode::F1 => KeyCode::F1,
            WKeyCode::F2 => KeyCode::F2,
            WKeyCode::F3 => KeyCode::F3,
            WKeyCode::F4 => KeyCode::F4,
            WKeyCode::F5 => KeyCode::F5,
            WKeyCode::F6 => KeyCode::F6,
            WKeyCode::F7 => KeyCode::F7,
            WKeyCode::F8 => KeyCode::F8,
            WKeyCode::F9 => KeyCode::F9,
            WKeyCode::F10 => KeyCode::F10,
            WKeyCode::F11 => KeyCode::F11,
            WKeyCode::F12 => KeyCode::F12,
            WKeyCode::F13 => KeyCode::F13,
            WKeyCode::F14 => KeyCode::F14,
            WKeyCode::F15 => KeyCode::F15,
            WKeyCode::F16 => KeyCode::F16,
            WKeyCode::F17 => KeyCode::F17,
            WKeyCode::F18 => KeyCode::F18,
            WKeyCode::F19 => KeyCode::F19,
            WKeyCode::F20 => KeyCode::F20,
            WKeyCode::F21 => KeyCode::F21,
            WKeyCode::F22 => KeyCode::F22,
            WKeyCode::F23 => KeyCode::F23,
            WKeyCode::F24 => KeyCode::F24,
            WKeyCode::PrintScreen => KeyCode::Snapshot,
            WKeyCode::ScrollLock => KeyCode::Scroll,
            WKeyCode::Pause => KeyCode::Pause,
            WKeyCode::Insert => KeyCode::Insert,
            WKeyCode::Home => KeyCode::Home,
            WKeyCode::Delete => KeyCode::Delete,
            WKeyCode::End => KeyCode::End,
            WKeyCode::PageDown => KeyCode::PageDown,
            WKeyCode::PageUp => KeyCode::PageUp,
            WKeyCode::ArrowLeft => KeyCode::Left,
            WKeyCode::ArrowUp => KeyCode::Up,
            WKeyCode::ArrowRight => KeyCode::Right,
            WKeyCode::ArrowDown => KeyCode::Down,
            WKeyCode::Back => KeyCode::Back,
            WKeyCode::Return => KeyCode::Return,
            WKeyCode::Space => KeyCode::Space,
            WKeyCode::Compose => KeyCode::Compose,
            WKeyCode::Caret => KeyCode::Caret,
            WKeyCode::NumLock => KeyCode::Numlock,
            WKeyCode::Numpad0 => KeyCode::Numpad0,
            WKeyCode::Numpad1 => KeyCode::Numpad1,
            WKeyCode::Numpad2 => KeyCode::Numpad2,
            WKeyCode::Numpad3 => KeyCode::Numpad3,
            WKeyCode::Numpad4 => KeyCode::Numpad4,
            WKeyCode::Numpad5 => KeyCode::Numpad5,
            WKeyCode::Numpad6 => KeyCode::Numpad6,
            WKeyCode::Numpad7 => KeyCode::Numpad7,
            WKeyCode::Numpad8 => KeyCode::Numpad8,
            WKeyCode::Numpad9 => KeyCode::Numpad9,
            WKeyCode::AbntC1 => KeyCode::AbntC1,
            WKeyCode::AbntC2 => KeyCode::AbntC2,
            WKeyCode::NumpadAdd => KeyCode::Add,
            WKeyCode::Apostrophe => KeyCode::Apostrophe,
            WKeyCode::Apps => KeyCode::Apps,
            WKeyCode::At => KeyCode::At,
            WKeyCode::Ax => KeyCode::Ax,
            WKeyCode::Backslash => KeyCode::Backslash,
            WKeyCode::Calculator => KeyCode::Calculator,
            WKeyCode::Capital => KeyCode::Capital,
            WKeyCode::Colon => KeyCode::Colon,
            WKeyCode::Comma => KeyCode::Comma,
            WKeyCode::Convert => KeyCode::Convert,
            WKeyCode::NumpadDecimal => KeyCode::Decimal,
            WKeyCode::NumpadDivide => KeyCode::Divide,
            WKeyCode::Equal => KeyCode::Equals,
            WKeyCode::Grave => KeyCode::Grave,
            WKeyCode::KanaMode => KeyCode::Kana,
            WKeyCode::KanjiMode => KeyCode::Kanji,
            WKeyCode::AltLeft => KeyCode::LAlt,
            WKeyCode::BracketLeft => KeyCode::LBracket,
            WKeyCode::ControlLeft => KeyCode::LControl,
            WKeyCode::ShiftLeft => KeyCode::LShift,
            WKeyCode::SuperLeft => KeyCode::LWin,
            WKeyCode::LaunchMail => KeyCode::Mail,
            WKeyCode::MediaSelect => KeyCode::MediaSelect,
            WKeyCode::MediaStop => KeyCode::MediaStop,
            WKeyCode::Minus => KeyCode::Minus,
            WKeyCode::NumpadMultiply => KeyCode::Multiply,
            WKeyCode::AudioVolumeMute => KeyCode::Mute,
            WKeyCode::MyComputer => KeyCode::MyComputer,
            WKeyCode::NavigateForward => KeyCode::NavigateForward,
            WKeyCode::NavigateBackward => KeyCode::NavigateBackward,
            WKeyCode::NextTrack => KeyCode::NextTrack,
            WKeyCode::NoConvert => KeyCode::NoConvert,
            WKeyCode::NumpadComma => KeyCode::NumpadComma,
            WKeyCode::NumpadEnter => KeyCode::NumpadEnter,
            WKeyCode::NumpadEquals => KeyCode::NumpadEquals,
            WKeyCode::OEM102 => KeyCode::OEM102,
            WKeyCode::Period => KeyCode::Period,
            WKeyCode::PlayPause => KeyCode::PlayPause,
            WKeyCode::Power => KeyCode::Power,
            WKeyCode::PrevTrack => KeyCode::PrevTrack,
            WKeyCode::AltRight => KeyCode::RAlt,
            WKeyCode::BracketRight => KeyCode::RBracket,
            WKeyCode::ControlRight => KeyCode::RControl,
            WKeyCode::ShiftRight => KeyCode::RShift,
            WKeyCode::SuperRight => KeyCode::RWin,
            WKeyCode::Semicolon => KeyCode::Semicolon,
            WKeyCode::Slash => KeyCode::Slash,
            WKeyCode::Sleep => KeyCode::Sleep,
            WKeyCode::Stop => KeyCode::Stop,
            WKeyCode::NumpadSubtract => KeyCode::Subtract,
            WKeyCode::Sysrq => KeyCode::Sysrq,
            WKeyCode::Tab => KeyCode::Tab,
            WKeyCode::Underline => KeyCode::Underline,
            WKeyCode::Unlabeled => KeyCode::Unlabeled,
            WKeyCode::AudioVolumeDown => KeyCode::VolumeDown,
            WKeyCode::AudioVolumeUp => KeyCode::VolumeUp,
            WKeyCode::WakeUp => KeyCode::Wake,
            WKeyCode::WebBack => KeyCode::WebBack,
            WKeyCode::WebFavorites => KeyCode::WebFavorites,
            WKeyCode::WebForward => KeyCode::WebForward,
            WKeyCode::WebHome => KeyCode::WebHome,
            WKeyCode::WebRefresh => KeyCode::WebRefresh,
            WKeyCode::WebSearch => KeyCode::WebSearch,
            WKeyCode::WebStop => KeyCode::WebStop,
            WKeyCode::Yen => KeyCode::Yen,
            WKeyCode::Copy => KeyCode::Copy,
            WKeyCode::Paste => KeyCode::Paste,
            WKeyCode::Cut => KeyCode::Cut,
            WKeyCode::Asterisk => KeyCode::Asterisk,
            WKeyCode::Plus => KeyCode::Plus,
        },
        _ => KeyCode::Unknown,
    }
}
