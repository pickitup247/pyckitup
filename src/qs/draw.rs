use crate::prelude::*;
use crate::qs::utils::*;

pub(crate) fn sound(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!( vm, args, required = [ (name, Some(vm.ctx.str_type())) ]);

    let name = objstr::get_value(name);
    let resources = sprites_mut(vm);
    resources.execute(|resources| {
        resources.get_sound(&name).unwrap().play()?;
        Ok(())
    }).unwrap();
    Ok(vm.get_none())
}

pub(crate) fn sprite(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [ (name, Some(vm.ctx.str_type())) ]
    );
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);

    let name = objstr::get_value(name);

    let (window, sprites) = window_sprites_mut(vm);
    match (args.kwargs.get("rect"), args.kwargs.get("p0")) {
        (Some(loc), None) => {
            let coord = get_rect_arg(loc);

            sprites.execute(|resources| {
                let im = resources.get_img(&name).unwrap();
                window.draw_ex(&coord, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, Some(p0)) => {
            let (p0x, p0y) = get_point_arg(&get_elements(p0));
            let pos = Vector::new(p0x, p0y);

            sprites.execute(|resources| {
                let im = resources.get_img(&name).unwrap();
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

pub(crate) fn text(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (text, Some(vm.ctx.str_type()))
        ]
    );
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let pt = args.kwargs.get("pt").map(|z|to_f32(z)).unwrap_or(90.);
    let color = args.kwargs.get("color").map(get_color_arg).unwrap_or(Color::BLACK);

    let font = args.kwargs.get("font").map(objstr::get_value);
    let store_in_cache = args.kwargs.get("store_in_cache").map(objbool::get_value).unwrap_or(true);
    let text = objstr::get_value(text);

    let style = FontStyle::new(pt, color);

    let (window, sprites) = window_sprites_mut(vm);
    match (args.kwargs.get("rect"), args.kwargs.get("p0")) {
        (Some(loc), None) => {
            let coord = get_rect_arg(loc);
            sprites.execute(|resources| {
                let im = resources.render_str(font, &text, style, store_in_cache);
                window.draw_ex(&coord, Img(&im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, Some(p0)) => {
            let (p0x, p0y) = get_point_arg(&get_elements(p0));
            let pos = Vector::new(p0x, p0y);
            sprites.execute(|resources| {
                let im = resources.render_str(font, &text, style, store_in_cache);
                let Rectangle {pos:_, size} = im.area();
                window.draw_ex(&Rectangle{pos, size}, Img(&im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, None) => panic!("sprite() must have either `p0=` or `rect=` named argument"),
        (Some(_), Some(_)) => panic!("sprite() must have either `p0=` or `rect=` named argument, but not both"),
    };
    Ok(vm.get_none())
}

pub(crate) fn anim(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(name, Some(vm.ctx.str_type()))]
    );
    let transform = args.kwargs.get("transform").map(|t| get_tranform_arg(t)).unwrap_or(Transform::IDENTITY);
    let z = args.kwargs.get("z").map(|z|to_i32(z)).unwrap_or(0);
    let name = objstr::get_value(name);

    let (window, sprites) = window_sprites_mut(vm);
    match (args.kwargs.get("rect"), args.kwargs.get("p0")) {
        (Some(loc), None) => {
            let coord = get_rect_arg(loc);
            sprites.execute(|resources| {
                let im = resources.get_anim(&name).expect(&format!("no animation called {}", name))
                    .current_frame();
                window.draw_ex(&coord, Img(im), transform, z);
                Ok(())
            }).unwrap();
        }
        (None, Some(p0)) => {
            let (p0x, p0y) = get_point_arg(&get_elements(p0));
            let pos = Vector::new(p0x, p0y);

            sprites.execute(|resources| {
                let im = resources.get_anim(&name).expect(&format!("no animation called {}", name))
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

pub(crate) fn set_anim_duration(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(name, Some(vm.ctx.str_type())), (dur, Some(vm.ctx.float_type()))]
    );
    let name = objstr::get_value(name);

    let resources = sprites_mut(vm);
    resources.execute(|resources| {
        resources.get_anim_mut(&name)
            .map(|i| i.set_duration(to_f32(dur).into())).unwrap();
        Ok(())
    }).unwrap();

    Ok(vm.get_none())
}