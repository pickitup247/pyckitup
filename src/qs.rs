use crate::prelude::*;
use num_traits::ToPrimitive;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol, FromPyObjectRef, TypeProtocol};

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

fn rect(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None)]);

    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);

    let coord = get_rect_arg(loc);
    let window = window_mut(vm);
    window.draw_ex(&coord, Col(color), transform, z);
    Ok(vm.get_none())
}

fn circ(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None), (radius, None)]);

    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);

    let p0 = get_elements(loc);
    let r = radius;
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let r = to_f32(r);
    let circle = Circle::new((p0x, p0y), r);
    let window = window_mut(vm);
    window.draw_ex(&circle, Col(color), transform, z);
    Ok(vm.get_none())
}

fn triangle(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None)]);

    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);

    let tri = get_triangle_arg(loc);
    let window = window_mut(vm);
    window.draw_ex(&tri, Col(color), transform, z);
    Ok(vm.get_none())
}

fn line(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None)]);
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);
    let thickness = args.kwargs.get("thickness").map(to_f32).unwrap_or(1.);

    let rect_loc = get_elements(loc);
    let (p0, p1) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap())
    );
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let (p1x, p1y) = (to_f32(p1.get(0).unwrap()), to_f32(p1.get(1).unwrap()));
    let line = Line::new((p0x, p0y), (p1x, p1y)).with_thickness(thickness);

    let window = window_mut(vm);
    window.draw_ex(&line, Col(color), transform, z);
    Ok(vm.get_none())
}

fn init_sprites(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(l, Some(vm.ctx.list_type()))]
    );
    let resources = resources_mut(vm);
    for image in get_elements(l).iter() {
        let image_params = get_elements(image);
        let name = objstr::get_value(image_params.get(0).unwrap());
        let path = objstr::get_value(image_params.get(1).unwrap());
        resources.0.push((name, path));
    }
    Ok(vm.get_none())
}

fn init_anims(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(l, Some(vm.ctx.list_type()))]
    );
    let resources = resources_mut(vm);
    for anim in get_elements(l).iter() {
        let anim_params = get_elements(anim);
        let name = objstr::get_value(anim_params.get(0).unwrap());
        let path = objstr::get_value(anim_params.get(1).unwrap());
        let w = to_usize(anim_params.get(2).unwrap());
        let h = to_usize(anim_params.get(3).unwrap());
        let dur = to_f64(anim_params.get(4).unwrap());

        resources.1.push((name, path, (w, h, dur)));
    }
    Ok(vm.get_none())
}

fn init_sounds(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(l, Some(vm.ctx.list_type()))]
    );
    let resources = resources_mut(vm);
    for sound in get_elements(l).iter() {
        let sound_params = get_elements(sound);
        let name = objstr::get_value(sound_params.get(0).unwrap());
        let path = objstr::get_value(sound_params.get(1).unwrap());
        resources.2.push((name, path));
    }
    Ok(vm.get_none())
}

fn sound(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [ (name, Some(vm.ctx.str_type())) ]);

    let name = objstr::get_value(name);
    let sprites = sprites_mut(vm);
    sprites.execute(|sprites| {
        sprites.get_sound(&name).unwrap().play();
        Ok(())
    }).unwrap();
    Ok(vm.get_none())
}

fn sprite(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [ (name, Some(vm.ctx.str_type())) ],
        optional = [(transform, None), (z, None)]
    );
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);

    let name = objstr::get_value(name);

    let (window, sprites) = window_sprites_mut(vm);
    match (args.kwargs.get("rect"), args.kwargs.get("p0")) {
        (Some(loc), None) => {
            let coord = get_rect_arg(loc);

            sprites.execute(|sprites| {
                let im = sprites.get_img(&name).unwrap();
                window.draw_ex(&coord, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, Some(p0)) => {
            let (p0x, p0y) = get_point_arg(&get_elements(p0));
            let pos = Vector::new(p0x, p0y);

            sprites.execute(|sprites| {
                let im = sprites.get_img(&name).unwrap();
                let Rectangle {pos:_, size} = im.area();
                window.draw_ex(&Rectangle{pos, size}, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, None) => panic!("sprite() must have either `p0=` or `rect=` named argument"),
        (Some(_), Some(_)) => panic!("sprite() must have either `p0=` or `rect=` named argument, but not both"),
    };

    Ok(vm.get_none())
}

fn anim(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(name, Some(vm.ctx.str_type()))],
        optional = [(transform, None), (z, None)]
    );
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let name = objstr::get_value(name);

    let (window, sprites) = window_sprites_mut(vm);
    match (args.kwargs.get("rect"), args.kwargs.get("p0")) {
        (Some(loc), None) => {
            let coord = get_rect_arg(loc);
            sprites.execute(|sprites| {
                let im = sprites.get_anim(&name).expect(&format!("no animation called {}", name))
                    .current_frame();
                window.draw_ex(&coord, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, Some(p0)) => {
            let (p0x, p0y) = get_point_arg(&get_elements(p0));
            let pos = Vector::new(p0x, p0y);

            sprites.execute(|sprites| {
                let im = sprites.get_anim(&name).expect(&format!("no animation called {}", name))
                    .current_frame();
                let Rectangle {pos:_, size} = im.area();
                window.draw_ex(&Rectangle{pos, size}, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, None) => panic!("anim() must have either `p0=` or `rect=` named argument"),
        (Some(_), Some(_)) => panic!("anim() must have either `p0=` or `rect=` named argument, but not both"),
    };

    Ok(vm.get_none())
}

fn mouse_pos(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let Vector {x,y} = window.mouse().pos();
    let d = vm.new_dict();
    d.set_item(&vm.ctx, "x", vm.new_int(x));
    d.set_item(&vm.ctx, "y", vm.new_int(y));
    Ok(d)
}

fn update_rate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let update_rate = window.update_rate();
    Ok(vm.new_int(update_rate))
}

fn set_update_rate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(rate, None)]);
    let window = window_mut(vm);
    window.set_update_rate(to_f32(rate) as f64);
    Ok(vm.get_none())
}

fn keyboard(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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

fn keyboard_bool(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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

fn set_view(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    use quicksilver::graphics::View;
    arg_check!( vm, args, required = [(loc, None)]);
    let window  = window_mut(vm);
    window.set_view(View::new(get_rect_arg(loc)));
    Ok(vm.get_none())
}

fn window_clear(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required=[(color, None)]);

    let color = get_color_arg(color);
    let window  = window_mut(vm);
    window.clear(color);
    Ok(vm.get_none())
}

fn mouse_wheel_delta(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let window  = window_mut(vm);
    let Vector {x,y} = window.mouse().wheel();
    let d = vm.new_dict();
    d.set_item(&vm.ctx, "x", vm.new_int(x));
    d.set_item(&vm.ctx, "y", vm.new_int(y));
    Ok(d)
}


fn get_color_arg(loc: &PyObjectRef) -> Color {
    let rgba = get_elements(loc);
    let (r, g, b, a) = (
        to_f32(rgba.get(0).unwrap()),
        to_f32(rgba.get(1).unwrap()),
        to_f32(rgba.get(2).unwrap()),
        to_f32(rgba.get(3).unwrap()),
    );

    Color { r, g, b, a }
}

fn get_tranform_arg(t: &PyObjectRef) -> Transform {
    let t = get_elements(t);
    let (p0, p1, p2) = (
        get_elements(t.get(0).unwrap()),
        get_elements(t.get(1).unwrap()),
        get_elements(t.get(2).unwrap()),
    );
    let (p0x, p0y, p0z) = (
        to_f32(p0.get(0).unwrap()),
        to_f32(p0.get(1).unwrap()),
        to_f32(p0.get(2).unwrap()),
    );
    let (p1x, p1y, p1z) = (
        to_f32(p1.get(0).unwrap()),
        to_f32(p1.get(1).unwrap()),
        to_f32(p1.get(2).unwrap()),
    );
    let (p2x, p2y, p2z) = (
        to_f32(p2.get(0).unwrap()),
        to_f32(p2.get(1).unwrap()),
        to_f32(p2.get(2).unwrap()),
    );
    Transform::from_array([
        [p0x, p0y, p0z],
        [p1x, p1y, p1z],
        [p2x, p2y, p2z]
    ])
}


/// [[x1, y1], [x2, y2], [x3, y3]]
fn get_triangle_arg(loc: &PyObjectRef) -> Triangle {
    let rect_loc = get_elements(loc);
    let (p0, p1, p2) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap()),
        get_elements(rect_loc.get(2).unwrap()),
    );
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let (p1x, p1y) = (to_f32(p1.get(0).unwrap()), to_f32(p1.get(1).unwrap()));
    let (p2x, p2y) = (to_f32(p2.get(0).unwrap()), to_f32(p2.get(1).unwrap()));
    Triangle::new((p0x, p0y), (p1x, p1y), (p2x, p2y))
}

fn get_point_arg(p0: &Vec<PyObjectRef>) -> (f32, f32) {
    (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()))
}

/// [[x1, y1], [x2, y2]]
fn get_rect_arg(loc: &PyObjectRef) -> Rectangle {
    let rect_loc = get_elements(loc);
    let (p0, p1) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap()),
    );
    let (p0x, p0y) = get_point_arg(&p0);
    let (p1x, p1y) = get_point_arg(&p1);
    Rectangle::new((p0x, p0y), (p1x, p1y))
}


fn resources_mut(vm: &mut VirtualMachine) -> &mut Resources {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("resources").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut Resources) }
}

fn sprites_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> &'b mut Asset<Sprites> {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("sprites").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut Asset<Sprites>) }
}


fn window_sprites_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> (&'b mut Window, &'b mut Asset<Sprites>) {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("window").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    let sprptr = qs.get_item("sprites").unwrap();
    let sprptr = rustpython_vm::obj::objint::get_value(&sprptr)
        .to_usize()
        .unwrap();
    (
        unsafe { &mut *(ptr as *mut Window) },
        unsafe { &mut *(sprptr as *mut Asset<Sprites>) }
    )
}

fn window_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> &'b mut Window {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).expect("qs is not loaded");
    let ptr = qs.get_item("window").expect("window is not initiailized");
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut Window) }
}

fn to_i32(p: &PyObjectRef) -> i32 {
    objint::get_value(p).to_i32().unwrap()
}

fn to_usize(p: &PyObjectRef) -> usize {
    objint::get_value(p).to_usize().unwrap()
}

fn to_f32(p: &PyObjectRef) -> f32 {
    match &p.payload {
        PyObjectPayload::Integer { value } => value.to_i32().unwrap() as f32,
        PyObjectPayload::Float { value } => *value as f32,
        f => panic!("TODO {:#?}", f),
    }
}

fn to_f64(p: &PyObjectRef) -> f64 {
    match &p.payload {
        PyObjectPayload::Integer { value } => value.to_i32().unwrap() as f64,
        PyObjectPayload::Float { value } => *value as f64,
        f => panic!("TODO {:#?}", f),
    }
}


pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, MOD_NAME, {
        "rect" => ctx.new_rustfunc(rect),
        "circ" => ctx.new_rustfunc(circ),
        "triangle" => ctx.new_rustfunc(triangle),
        "line" => ctx.new_rustfunc(line),

        "sprite" => ctx.new_rustfunc(sprite),
        "anim" => ctx.new_rustfunc(anim),
        "sound" => ctx.new_rustfunc(sound),

        "clear" => ctx.new_rustfunc(window_clear),

        "init_sprites" => ctx.new_rustfunc(init_sprites),
        "init_anims" => ctx.new_rustfunc(init_anims),
        "init_sounds" => ctx.new_rustfunc(init_sounds),

        "mouse_pos" => ctx.new_rustfunc(mouse_pos),
        "mouse_wheel_delta" => ctx.new_rustfunc(mouse_wheel_delta),

        "keyboard" => ctx.new_rustfunc(keyboard),
        "keyboard_bool" => ctx.new_rustfunc(keyboard_bool),

        "set_view" => ctx.new_rustfunc(set_view),
        "update_rate" => ctx.new_rustfunc(update_rate),
        "set_update_rate" => ctx.new_rustfunc(set_update_rate),

    })
}
