use crate::prelude::*;
use num_traits::ToPrimitive;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol, FromPyObjectRef, TypeProtocol};

macro_rules! decl_shape_fn {
    ($fn_name: tt, $shape_fn: expr) => {
        fn $fn_name(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
            arg_check!(
                vm,
                args,
                required = [(loc, None), (color, None)],
                optional = [(transform, None), (z, None)]
            );
            let z = to_i32(&z.unwrap_or(&Rc::clone(&vm.new_int(0))));
            let window = window(vm);
            let coord = $shape_fn(loc);
            let color = get_color_arg(color);
            let transform = transform
                .map(|t| get_tranform_arg(t))
                .unwrap_or(Transform::IDENTITY);
            window.draw_ex(&coord, Col(color), transform, z);
            Ok(vm.get_none())
        }
    };
}

decl_shape_fn!(rect, get_rect_arg);
decl_shape_fn!(circ, get_circ_arg);
decl_shape_fn!(triangle, get_triangle_arg);
decl_shape_fn!(line, get_line_arg);

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
fn get_line_arg(loc: &PyObjectRef) -> Line {
    let rect_loc = get_elements(loc);
    let (p0, p1, t) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap()),
        rect_loc.get(2).map(|t|to_f32(t)).unwrap_or(1.),
    );
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let (p1x, p1y) = (to_f32(p1.get(0).unwrap()), to_f32(p1.get(1).unwrap()));
    Line::new((p0x, p0y), (p1x, p1y)).with_thickness(t)
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

/// [[x1, y1], [x2, y2]]
fn get_rect_arg(loc: &PyObjectRef) -> Rectangle {
    let rect_loc = get_elements(loc);
    let (p0, p1) = (
        get_elements(rect_loc.get(0).unwrap()),
        get_elements(rect_loc.get(1).unwrap()),
    );
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let (p1x, p1y) = (to_f32(p1.get(0).unwrap()), to_f32(p1.get(1).unwrap()));
    Rectangle::new((p0x, p0y), (p1x, p1y))
}

/// [[x1, y1], r]
fn get_circ_arg(loc: &PyObjectRef) -> Circle {
    let rect_loc = get_elements(loc);
    let (p0, r) = (
        get_elements(rect_loc.get(0).unwrap()),
        rect_loc.get(1).unwrap(),
    );
    let (p0x, p0y) = (to_f32(p0.get(0).unwrap()), to_f32(p0.get(1).unwrap()));
    let r = to_f32(r);
    Circle::new((p0x, p0y), r)
}

fn window(vm: &mut VirtualMachine) -> &mut Window {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("window").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr)
        .to_usize()
        .unwrap();
    let window: &mut Window = unsafe { &mut *(ptr as *mut Window) };
    window
}

fn to_i32(p: &PyObjectRef) -> i32 {
    objint::get_value(p).to_i32().unwrap()
}

fn to_f32(p: &PyObjectRef) -> f32 {
    match &p.borrow().payload {
        PyObjectPayload::Integer { value } => value.to_i32().unwrap() as f32,
        PyObjectPayload::Float { value } => *value as f32,
        f => panic!("TODO {:#?}", f),
    }
}

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, MOD_NAME, {
        "rect" => ctx.new_rustfunc(rect),
        "circ" => ctx.new_rustfunc(circ),
        "triangle" => ctx.new_rustfunc(triangle),
        "line" => ctx.new_rustfunc(line),
    })
}
