extern crate quicksilver;
// #[macro_use]extern crate rustpython_vm;

// use rustpython_vm::obj::objstr;
// use rustpython_vm::pyobject::{PyContext, PyFuncArgs, PyObjectRef, PyResult};
// use rustpython_vm::{compile, VirtualMachine};

use quicksilver::{
    Result,
    geom::{Shape, Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::MouseCursor,
    lifecycle::{Event, Settings, State, Window, run}
};

struct RectangleState {
    grab_rect: Rectangle,
    crosshair_rect: Rectangle,
    // vm: VirtualMachine,
}

impl State for RectangleState {
    fn new() -> Result<Self> {
        // let mut vm = VirtuaGlMachine::new();
        // setup_qs_module(&mut vm);

        Ok(RectangleState {
            grab_rect: Rectangle::new((0, 0), (200, 100)).with_center((400, 100)),
            crosshair_rect: Rectangle::new((0, 0), (200, 100)).with_center((400, 400)),
            // vm,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseMoved(vector) => {
                if vector.overlaps_rectangle(&self.crosshair_rect) {
                    window.set_cursor(MouseCursor::Crosshair);
                } else if vector.overlaps_rectangle(&self.grab_rect) {
                    window.set_cursor(MouseCursor::Grab);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            Event::Key(_,_) => {
                // let source = "from qs import hello\nhello()\n";
                // let mode = compile::Mode::Exec;
                // let code =
                //     compile::compile(&source, &mode, "<qs>".to_string(), self.vm.ctx.code_type())
                //         .map_err(|err| {
                //             dbg!(&err);
                //             format!("Error parsing Python code: {}", err)
                //         }).unwrap();

                // let builtin = self.vm.get_builtin_scope();
                // let scope = self.vm.context().new_scope(Some(builtin));

                // let result = self.vm.run_code_obj(code, scope.clone());
                // if let Err(py_err) = result {
                //     let res = self.vm.to_pystr(&py_err)
                //         .unwrap_or_else(|_| "Error, and error getting error message".into());
                //     dbg!(&res);
                // }

            }
            _ => {}
        };

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        window.draw(&self.grab_rect, Col(Color::RED));
        window.draw(&self.crosshair_rect, Col(Color::GREEN));

        Ok(())
    }
}

// fn hello(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
//     // arg_check!(
//     //     vm,
//     //     args,
//     //     required = [
//     //         (url, Some(vm.ctx.str_type())),
//     //         (handler, Some(vm.ctx.function_type()))
//     //     ],
//     //     optional = [(reject_handler, Some(vm.ctx.function_type()))]
//     // );
//     println!("HELLO!");
//     Ok(vm.get_none())
// }
// const BROWSER_NAME: &str = "qs";

// pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
//     py_module!(ctx, BROWSER_NAME, {
//         "hello" => ctx.new_rustfunc(hello)
//     })
// }

// pub fn setup_qs_module(vm: &mut VirtualMachine) {
//     vm.stdlib_inits.insert(BROWSER_NAME.to_string(), mk_module);
// }

fn main() {
    run::<RectangleState>("set-cursor", Vector::new(800, 600), Settings::default());
}
