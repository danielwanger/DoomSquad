use minifb::{Key, KeyRepeat, Window};
use std::collections::HashMap;
use std::fs;

const KEYBINDINGS_FILE: &str = "keybindings.txt";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Sprint,
    Jump,
    Crouch,
    RocketJump,
    Interact,
    CrosshairUp,
    CrosshairDown,
    TurnLeft,
    TurnRight,
    ToggleAim,
    FireBullet,
    FireArrow,
    Godmode,
    LockEnemies,
    Exit,
}

impl Action {
    pub const ALL: [Action; 19] = [
        Action::MoveForward,
        Action::MoveBackward,
        Action::MoveLeft,
        Action::MoveRight,
        Action::Sprint,
        Action::Jump,
        Action::Crouch,
        Action::RocketJump,
        Action::Interact,
        Action::CrosshairUp,
        Action::CrosshairDown,
        Action::TurnLeft,
        Action::TurnRight,
        Action::ToggleAim,
        Action::FireBullet,
        Action::FireArrow,
        Action::Godmode,
        Action::LockEnemies,
        Action::Exit,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Action::MoveForward => "Forwards",
            Action::MoveBackward => "Backwards",
            Action::MoveLeft => "Left",
            Action::MoveRight => "Right",
            Action::Sprint => "Sprint",
            Action::Jump => "Jump",
            Action::Crouch => "Crouch",
            Action::RocketJump => "Rocket Jump",
            Action::Interact => "Interact",
            Action::CrosshairUp => "Crosshair up",
            Action::CrosshairDown => "Crosshair down",
            Action::TurnLeft => "Turn left",
            Action::TurnRight => "Turn right",
            Action::ToggleAim => "Fine Aiming",
            Action::FireBullet => "Fire Bullet",
            Action::FireArrow => "Fire Arrow",
            Action::Godmode => "Godmode",
            Action::LockEnemies => "Lock enemies",
            Action::Exit => "Exit to menu",
        }
    }
}

const BINDABLE_KEYS: [Key; 106] = [
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Up,
    Key::Apostrophe,
    Key::Backquote,
    Key::Backslash,
    Key::Comma,
    Key::Equal,
    Key::LeftBracket,
    Key::Minus,
    Key::Period,
    Key::RightBracket,
    Key::Semicolon,
    Key::Slash,
    Key::Backspace,
    Key::Delete,
    Key::End,
    Key::Enter,
    Key::Escape,
    Key::Home,
    Key::Insert,
    Key::Menu,
    Key::PageDown,
    Key::PageUp,
    Key::Pause,
    Key::Space,
    Key::Tab,
    Key::NumLock,
    Key::CapsLock,
    Key::ScrollLock,
    Key::LeftShift,
    Key::RightShift,
    Key::LeftCtrl,
    Key::RightCtrl,
    Key::NumPad0,
    Key::NumPad1,
    Key::NumPad2,
    Key::NumPad3,
    Key::NumPad4,
    Key::NumPad5,
    Key::NumPad6,
    Key::NumPad7,
    Key::NumPad8,
    Key::NumPad9,
    Key::NumPadDot,
    Key::NumPadSlash,
    Key::NumPadAsterisk,
    Key::NumPadMinus,
    Key::NumPadPlus,
    Key::NumPadEnter,
    Key::LeftAlt,
    Key::RightAlt,
    Key::LeftSuper,
    Key::RightSuper,
];

pub struct KeyBindings {
    map: HashMap<Action, Key>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Action::MoveForward, Key::W);
        map.insert(Action::MoveBackward, Key::S);
        map.insert(Action::MoveLeft, Key::A);
        map.insert(Action::MoveRight, Key::D);
        map.insert(Action::Sprint, Key::LeftShift);
        map.insert(Action::Jump, Key::Space);
        map.insert(Action::Crouch, Key::C);
        map.insert(Action::RocketJump, Key::R);
        map.insert(Action::Interact, Key::F);
        map.insert(Action::CrosshairUp, Key::Up);
        map.insert(Action::CrosshairDown, Key::Down);
        map.insert(Action::TurnLeft, Key::Left);
        map.insert(Action::TurnRight, Key::Right);
        map.insert(Action::ToggleAim, Key::E);
        map.insert(Action::FireBullet, Key::RightCtrl);
        map.insert(Action::FireArrow, Key::RightShift);
        map.insert(Action::Godmode, Key::G);
        map.insert(Action::LockEnemies, Key::L);
        map.insert(Action::Exit, Key::Escape);
        Self { map }
    }
}

impl KeyBindings {
    pub fn is_down(&self, window: &Window, action: Action) -> bool {
        match self.map.get(&action) {
            Some(key) => window.is_key_down(*key),
            None => false,
        }
    }

    pub fn is_pressed(&self, window: &Window, action: Action) -> bool {
        match self.map.get(&action) {
            Some(key) => window.is_key_pressed(*key, KeyRepeat::No),
            None => false,
        }
    }

    pub fn key_for(&self, action: Action) -> Option<Key> {
        self.map.get(&action).copied()
    }

    pub fn set(&mut self, action: Action, key: Key) {
        self.map.insert(action, key);
        self.save();
    }

    fn save(&self) {
        let mut text = String::new();
        for action in Action::ALL {
            if let Some(key) = self.map.get(&action) {
                text.push_str(&format!("{:?}={:?}\n", action, key));
            }
        }
        let _ = fs::write(KEYBINDINGS_FILE, text);
    }

    pub fn load() -> Self {
        let mut bindings = Self::default();
        if let Ok(content) = fs::read_to_string(KEYBINDINGS_FILE) {
            for line in content.lines() {
                if let Some((action_name, key_name)) = line.split_once('=') {
                    let action = Action::ALL
                        .into_iter()
                        .find(|a| format!("{:?}", a) == action_name.trim());
                    let key = BINDABLE_KEYS
                        .into_iter()
                        .find(|k| format!("{:?}", k) == key_name.trim());
                    if let (Some(action), Some(key)) = (action, key) {
                        bindings.map.insert(action, key);
                    }
                }
            }
        }
        bindings
    }
}
