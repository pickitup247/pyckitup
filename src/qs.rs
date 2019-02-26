use crate::prelude::*;
use num_traits::ToPrimitive;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol, TypeProtocol, FromPyObjectRef};

pub fn rect(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (loc, None),
            (color, None)
        ],
        optional = [
            (transform, None),
            (z, None)
        ]
    );

    let z = to_i32(&z.unwrap_or(&Rc::clone(&vm.new_int(0))));
    let rot = 1;

    let window = window(vm);
    window.draw_ex(&get_coord_arg(loc), Col(get_color_arg(color)), Transform::rotate(rot), z);
    Ok(vm.get_none())
}

pub fn get_color_arg(loc: &PyObjectRef) -> Color {
    let rgba = get_elements(loc);
    // dbg!(rgba.get(0).unwrap());

    let (r, g, b, a) = (
        to_f32(rgba.get(0).unwrap()),
        to_f32(rgba.get(1).unwrap()),
        to_f32(rgba.get(2).unwrap()),
        to_f32(rgba.get(3).unwrap()),
    );

    Color {r,g,b,a}
}

pub fn get_coord_arg(loc: &PyObjectRef) -> Rectangle {
    let rect_loc = get_elements(loc);
    let (p0, p1) = (get_elements(rect_loc.get(0).unwrap()), get_elements(rect_loc.get(1).unwrap()));
    let (p0x, p0y)  = ( to_i32(p0.get(0).unwrap()), to_i32(p0.get(1).unwrap()));
    let (p1x, p1y) = ( to_i32(p1.get(0).unwrap()), to_i32(p1.get(1).unwrap()));

    Rectangle::new((p0x, p0y), (p1x, p1y))
}

pub fn window(vm: &mut VirtualMachine) -> &mut Window {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("window").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr).to_usize().unwrap();
    let window: &mut Window = unsafe { &mut *(ptr as *mut Window) };
    window
}

fn to_i32(p: &PyObjectRef) -> i32 {
    objint::get_value(p).to_i32().unwrap()
}

fn to_f32(p: &PyObjectRef) -> f32 {
    objfloat::get_value(p) as f32
}