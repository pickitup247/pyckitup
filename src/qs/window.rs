use crate::prelude::*;
use crate::qs::utils::*;

pub(crate) fn window_clear(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required=[(color, None)]);

    let color = get_color_arg(color);
    let window  = window_mut(vm);
    window.clear(color).expect("Failed to clear window");
    Ok(vm.get_none())
}

pub(crate) fn mouse_wheel_delta(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let Vector {x,y} = window.mouse().wheel();
    let d = vm.new_dict();
    d.set_item(&vm.ctx, "x", vm.new_int(x));
    d.set_item(&vm.ctx, "y", vm.new_int(y));
    Ok(d)
}


pub(crate) fn mouse_pos(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let Vector {x,y} = window.mouse().pos();
    let d = vm.new_dict();
    d.set_item(&vm.ctx, "x", vm.new_int(x));
    d.set_item(&vm.ctx, "y", vm.new_int(y));
    Ok(d)
}

pub(crate) fn update_rate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let update_rate = window.update_rate();
    Ok(vm.new_int(update_rate))
}

pub(crate) fn set_update_rate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(rate, None)]);
    let window = window_mut(vm);
    window.set_update_rate(to_f32(rate) as f64);
    Ok(vm.get_none())
}

const KEY_LIST: &[Key] = &[Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9, Key::Key0, Key::A, Key::B, Key::C, Key::D,
    Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L, Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X, Key::Y, Key::Z,
    Key::Escape, Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12, Key::F13, Key::F14, Key::F15, Key::F16, Key::F17, Key::F18,
    Key::F19, Key::F20, Key::F21, Key::F22, Key::F23, Key::F24, Key::Snapshot, Key::Scroll, Key::Pause, Key::Insert, Key::Home, Key::Delete, Key::End, Key::PageDown, Key::PageUp, Key::Left, Key::Up, Key::Right,
    Key::Down, Key::Back, Key::Return, Key::Space, Key::Compose, Key::Caret, Key::Numlock, Key::Numpad0, Key::Numpad1, Key::Numpad2, Key::Numpad3, Key::Numpad4, Key::Numpad5,
    Key::Numpad6, Key::Numpad7, Key::Numpad8, Key::Numpad9, Key::AbntC1, Key::AbntC2, Key::Add, Key::Apostrophe, Key::Apps, Key::At, Key::Ax, Key::Backslash, Key::Calculator,
    Key::Capital, Key::Colon, Key::Comma, Key::Convert, Key::Decimal, Key::Divide, Key::Equals, Key::Grave, Key::Kana, Key::Kanji, Key::LAlt, Key::LBracket, Key::LControl,
    Key::LShift, Key::LWin, Key::Mail, Key::MediaSelect, Key::MediaStop, Key::Minus, Key::Multiply, Key::Mute, Key::MyComputer, Key::NavigateForward,
    Key::NavigateBackward, Key::NextTrack, Key::NoConvert, Key::NumpadComma, Key::NumpadEnter, Key::NumpadEquals, Key::OEM102, Key::Period, Key::PlayPause,
    Key::Power, Key::PrevTrack, Key::RAlt, Key::RBracket, Key::RControl, Key::RShift, Key::RWin, Key::Semicolon, Key::Slash, Key::Sleep, Key::Stop, Key::Subtract,
    Key::Sysrq, Key::Tab, Key::Underline, Key::Unlabeled, Key::VolumeDown, Key::VolumeUp, Key::Wake, Key::WebBack, Key::WebFavorites, Key::WebForward, Key::WebHome,
    Key::WebRefresh, Key::WebSearch, Key::WebStop, Key::Yen
];
pub(crate) fn keyboard(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let keys = window.keyboard();
    let d = vm.new_dict();
    for key in KEY_LIST {
        let val = vm.new_str(format!("{:?}", keys[*key]));
        d.set_item(&vm.ctx, &format!("{:?}", key), val);
    }
    Ok(d)
}

pub(crate) fn keyboard_bool(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let keys = window.keyboard();
    let d = vm.new_dict();
    for key in KEY_LIST {
        let val = vm.new_bool(keys[*key].is_down());
        d.set_item(&vm.ctx, &format!("{:?}", key), val);
    }
    Ok(d)
}

pub(crate) fn set_view(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    use quicksilver::graphics::View;
    arg_check!( vm, args, required = [(loc, None)]);
    let window  = window_mut(vm);
    window.set_view(View::new(get_rect_arg(loc)));
    Ok(vm.get_none())
}