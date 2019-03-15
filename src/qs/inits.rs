use crate::prelude::*;
use crate::qs::utils::*;

pub(crate) fn init_sprites(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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
        resources.imgs.push((name, path));
    }
    Ok(vm.get_none())
}

pub(crate) fn init_anims(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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
        let nframes = to_usize(anim_params.get(2).unwrap());
        let dur = to_f64(anim_params.get(3).unwrap());

        resources.anims.push((name, path, (nframes, dur)));
    }
    Ok(vm.get_none())
}

pub(crate) fn init_sounds(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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
        resources.sounds.push((name, path));
    }
    Ok(vm.get_none())
}

pub(crate) fn init_fonts(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
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
        resources.fonts.push((name, path));
    }
    Ok(vm.get_none())
}

