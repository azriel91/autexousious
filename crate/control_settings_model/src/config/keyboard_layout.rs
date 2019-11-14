use amethyst::winit::VirtualKeyCode;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Keyboard layout variants.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum KeyboardLayout {
    /// US keyboard layout
    #[derivative(Default)]
    Us,
}

impl KeyboardLayout {
    /// Returns the keyboard buttons present for this keyboard layout.
    ///
    /// Development note: This is not called `keys()` as it is ambiguous with [`HashMap::keys`].
    pub fn buttons(self) -> Vec<VirtualKeyCode> {
        match self {
            KeyboardLayout::Us => {
                vec![
                    // Bottom row.
                    VirtualKeyCode::LControl,
                    VirtualKeyCode::LWin,
                    VirtualKeyCode::LAlt,
                    VirtualKeyCode::Space,
                    VirtualKeyCode::RAlt,
                    VirtualKeyCode::RWin,
                    VirtualKeyCode::Apps, // Context menu
                    VirtualKeyCode::RControl,
                    VirtualKeyCode::Left,
                    VirtualKeyCode::Down,
                    VirtualKeyCode::Right,
                    VirtualKeyCode::Numpad0,
                    VirtualKeyCode::NumpadComma,
                    VirtualKeyCode::NumpadEnter,
                    // Second row.
                    VirtualKeyCode::LShift,
                    VirtualKeyCode::Z,
                    VirtualKeyCode::X,
                    VirtualKeyCode::C,
                    VirtualKeyCode::V,
                    VirtualKeyCode::B,
                    VirtualKeyCode::N,
                    VirtualKeyCode::M,
                    VirtualKeyCode::Comma,
                    VirtualKeyCode::Period,
                    VirtualKeyCode::Slash,
                    VirtualKeyCode::RShift,
                    VirtualKeyCode::Up,
                    VirtualKeyCode::Numpad1,
                    VirtualKeyCode::Numpad2,
                    VirtualKeyCode::Numpad3,
                    // Third row.
                    VirtualKeyCode::Capital,
                    VirtualKeyCode::A,
                    VirtualKeyCode::S,
                    VirtualKeyCode::D,
                    VirtualKeyCode::F,
                    VirtualKeyCode::G,
                    VirtualKeyCode::H,
                    VirtualKeyCode::J,
                    VirtualKeyCode::K,
                    VirtualKeyCode::L,
                    VirtualKeyCode::Semicolon,  // Colon
                    VirtualKeyCode::Apostrophe, // Quote / double quote.
                    VirtualKeyCode::Return,     // Enter
                    VirtualKeyCode::Numpad4,
                    VirtualKeyCode::Numpad5,
                    VirtualKeyCode::Numpad6,
                    VirtualKeyCode::Add,
                    // Fourth row.
                    VirtualKeyCode::Tab,
                    VirtualKeyCode::Q,
                    VirtualKeyCode::W,
                    VirtualKeyCode::E,
                    VirtualKeyCode::R,
                    VirtualKeyCode::T,
                    VirtualKeyCode::Y,
                    VirtualKeyCode::U,
                    VirtualKeyCode::I,
                    VirtualKeyCode::O,
                    VirtualKeyCode::P,
                    VirtualKeyCode::LBracket,
                    VirtualKeyCode::RBracket,
                    VirtualKeyCode::Backslash,
                    VirtualKeyCode::End,
                    VirtualKeyCode::Delete,
                    VirtualKeyCode::PageDown,
                    VirtualKeyCode::Numpad7,
                    VirtualKeyCode::Numpad8,
                    VirtualKeyCode::Numpad9,
                    // Fifth row.
                    VirtualKeyCode::Grave,
                    VirtualKeyCode::Key1,
                    VirtualKeyCode::Key2,
                    VirtualKeyCode::Key3,
                    VirtualKeyCode::Key4,
                    VirtualKeyCode::Key5,
                    VirtualKeyCode::Key6,
                    VirtualKeyCode::Key7,
                    VirtualKeyCode::Key8,
                    VirtualKeyCode::Key9,
                    VirtualKeyCode::Key0,
                    VirtualKeyCode::Minus, // Underline
                    VirtualKeyCode::Equals,
                    VirtualKeyCode::Back,
                    VirtualKeyCode::Insert,
                    VirtualKeyCode::Home,
                    VirtualKeyCode::PageUp,
                    VirtualKeyCode::Numlock,
                    VirtualKeyCode::Divide,
                    VirtualKeyCode::Multiply,
                    VirtualKeyCode::Subtract, // NumpadSubtract
                    // Top row.
                    VirtualKeyCode::Escape,
                    VirtualKeyCode::F1,
                    VirtualKeyCode::F2,
                    VirtualKeyCode::F3,
                    VirtualKeyCode::F4,
                    VirtualKeyCode::F5,
                    VirtualKeyCode::F6,
                    VirtualKeyCode::F7,
                    VirtualKeyCode::F8,
                    VirtualKeyCode::F9,
                    VirtualKeyCode::F10,
                    VirtualKeyCode::F11,
                    VirtualKeyCode::F12,
                    VirtualKeyCode::Snapshot, // Print Screen
                    VirtualKeyCode::Sysrq,
                    VirtualKeyCode::Scroll,
                    VirtualKeyCode::Pause,
                ]
            }
        }
    }
}
