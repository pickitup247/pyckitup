use crate::prelude::*;
use crate::qs::utils::*;

pub(crate) fn rect(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None)]);

    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);

    let coord = get_rect_arg(loc);
    let window = window_mut(vm);
    window.draw_ex(&coord, Col(color), transform, z);
    Ok(vm.get_none())
}

pub(crate) fn circ(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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

pub(crate) fn triangle(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [(loc, None)]);

    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::RED);

    let tri = get_triangle_arg(loc);
    let window = window_mut(vm);
    window.draw_ex(&tri, Col(color), transform, z);
    Ok(vm.get_none())
}

pub(crate) fn line(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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