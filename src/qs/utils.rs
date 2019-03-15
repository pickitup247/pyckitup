use crate::prelude::*;

pub(crate) fn resources_mut(vm: &mut VirtualMachine) -> &mut ResourceConfig {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("resources").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut ResourceConfig) }
}

pub(crate) fn sprites_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> &'b mut Asset<Resources> {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("sprites").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut Asset<Resources>) }
}


pub(crate) fn window_sprites_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> (&'b mut Window, &'b mut Asset<Resources>) {
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
        unsafe { &mut *(sprptr as *mut Asset<Resources>) }
    )
}

pub(crate) fn window_mut<'a, 'b>(vm: &'a mut VirtualMachine) -> &'b mut Window {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).expect("qs is not loaded");
    let ptr = qs.get_item("window").expect("window is not initiailized");
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    unsafe { &mut *(ptr as *mut Window) }
}

pub(crate) fn to_i32(p: &PyObjectRef) -> i32 {
    objint::get_value(p).to_i32().unwrap()
}

pub(crate) fn to_usize(p: &PyObjectRef) -> usize {
    objint::get_value(p).to_usize().unwrap()
}

pub(crate) fn to_f32(p: &PyObjectRef) -> f32 {
    match &p.payload {
        PyObjectPayload::Integer { value } => value.to_i32().unwrap() as f32,
        PyObjectPayload::Float { value } => *value as f32,
        f => panic!("TODO {:#?}", f),
    }
}

pub(crate) fn to_f64(p: &PyObjectRef) -> f64 {
    match &p.payload {
        PyObjectPayload::Integer { value } => value.to_i32().unwrap() as f64,
        PyObjectPayload::Float { value } => *value as f64,
        f => panic!("TODO {:#?}", f),
    }
}

pub(crate) fn get_tranform_arg(t: &PyObjectRef) -> Transform {
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
pub(crate) fn get_triangle_arg(loc: &PyObjectRef) -> Triangle {
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

pub(crate) fn get_point_arg(p0: &Vec<PyObjectRef>) -> (f32, f32) {
    (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()))
}

/// [[x1, y1], [x2, y2]]
pub(crate) fn get_rect_arg(loc: &PyObjectRef) -> Rectangle {
    let rect_loc = get_elements(loc);
    let (p0, p1) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap()),
    );
    let (p0x, p0y) = get_point_arg(&p0);
    let (p1x, p1y) = get_point_arg(&p1);
    Rectangle::new((p0x, p0y), (p1x, p1y))
}

pub(crate) fn get_color_arg(loc: &PyObjectRef) -> Color {
    let rgba = get_elements(loc);
    let (r, g, b, a) = (
        to_f32(rgba.get(0).unwrap()),
        to_f32(rgba.get(1).unwrap()),
        to_f32(rgba.get(2).unwrap()),
        to_f32(rgba.get(3).unwrap()),
    );

    Color { r, g, b, a }
}
