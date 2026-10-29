use minifb::Key;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
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
