use crate::prelude::*;
use num_traits::ToPrimitive;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol};

pub fn draw_rect(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    let modules = vm.sys_module.get_attr("modules").unwrap();
    let qs = modules.get_item(MOD_NAME).unwrap();
    let ptr = qs.get_item("window").unwrap();
    let ptr = rustpython_vm::obj::objint::get_value(&ptr).to_usize().unwrap();
    let window: &mut Window = unsafe { &mut *(ptr as *mut Window) };
    dbg!(1);
    // TODO:
    window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
    Ok(vm.get_none())
}
