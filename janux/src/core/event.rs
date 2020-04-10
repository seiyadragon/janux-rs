use crate::core::*;

#[derive(Clone, Copy)]
pub enum Key
{
    Escape,
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

    Tilde,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,
    Backspace,

    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    OpenBracket,
    CloseBracket,
    Backslash,

    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    SingleQuote,
    Enter,

    Shift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    Slash,
    RShift,

    Ctrl,
    Super,
    Alt,
    Space,
    RAlt,
    Fnk,
    Menu,
    RCtrl,

    PrintScreen,
    ScrollLock,
    PauseBreak,
    
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,

    Up,
    Down,
    Left,
    Right,

    KeypadNumLock,
    KeypadSlash,
    KeypadAsterisc,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
}

#[derive(Clone, Copy)]
pub enum Button
{
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
}

#[derive(Clone, Copy)]
pub struct Mods
{
    shift: bool,
    ctrl: bool,
    alt: bool,
    supr: bool,
}

#[derive(Clone, Copy)]
pub enum Action
{
    Press,
    Release,
    Type,
    Enter,
    Leave,
    Move,
}

pub struct WindowListener
{
    pub on_move: dyn Fn(&minifb::Window),
    pub on_resize: dyn Fn(&minifb::Window),
    pub on_close: dyn Fn(&minifb::Window),
}

pub struct KeyListener
{
    pub on_key: dyn Fn(Key, Action, Mods),
}

pub struct MouseListener
{
    pub on_button: dyn Fn(Button, Action, Mods),
    pub on_cursor: dyn Fn(Action, f64, f64),
}

pub struct SceneListener
{
    pub on_render: dyn Fn(Clock),
    pub on_update: dyn Fn(Clock),
}