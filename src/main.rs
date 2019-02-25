#![feature(pin)]

use std::pin::Pin;

extern crate quicksilver;
#[macro_use]extern crate rustpython_vm;
mod prelude;
use crate::prelude::*;

struct PickItUp {
    vm: VirtualMachine,
    source: Rc<RefCell<Asset<String>>>,
    update_fn: Option<PyObjectRef>,
    draw_fn: Option<PyObjectRef>,
    init_fn: Option<PyObjectRef>,
    state: Option<PyObjectRef>,
}

use rustpython_vm::pyobject::DictProtocol;

fn handle_err(vm: &mut VirtualMachine, py_err: PyObjectRef) {
    let res = vm.to_pystr(&py_err)
        .unwrap_or_else(|_| "Error, and error getting error message".into());
    dbg!(&res);
}

impl PickItUp {
    fn load_code(&mut self) -> Result<()> {
        self.source.clone().borrow_mut().execute(|source| {
            let mode = compile::Mode::Exec;
            let code =
                compile::compile(&source, &mode, "<qs>".to_string(), self.vm.ctx.code_type())
                    .map_err(|err| {
                        dbg!(&err);
                        format!("Error parsing Python code: {}", err)
                    }).unwrap();

            let builtin = self.vm.get_builtin_scope();
            let scope = self.vm.context().new_scope(Some(builtin));
            let result = self.vm.run_code_obj(code, scope.clone());
            match result {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(res) => {
                    dbg!(&res);
                    let init_fn = res.get_item("init").unwrap();
                    self.state = Some(self.vm.invoke(Rc::clone(&init_fn), PyFuncArgs::new(vec![], vec![])).unwrap());
                    self.init_fn = Some(init_fn);

                    self.update_fn = Some(res.get_item("update").unwrap());
                    self.draw_fn = Some(res.get_item("draw").unwrap());
                }
            };

            Ok(())
        });
        Ok(())
    }

    fn reload(&mut self) -> Result<()> {
        self.source = Rc::new(RefCell::new(Asset::new(load_file("run.py").map(|v8| String::from_utf8(v8).unwrap()))));
        self.load_code()
    }


    fn setup_module(&mut self, window: &mut Window) -> Result<()>{

        let test = |vm: &mut VirtualMachine, args: PyFuncArgs| {println!("test");  Ok(vm.get_none())};
        let mk_module = Pin::new(Box::new(|ctx: &PyContext| -> PyObjectRef {
            py_module!(ctx, "qs", {
                "test" => ctx.new_rustfunc(test),
            })
        }));
        let mk_module = Pin::get_mut(mk_module);
        unsafe {
            let mk_module = std::mem::transmute::<_, &'static fn(&PyContext) -> PyObjectRef>(mk_module);
            self.vm.stdlib_inits.insert("test".to_string(), *mk_module);
        }
        Ok(())
    }

}


    fn draw_square(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
        println!("square!");
        Ok(vm.get_none())
    }

    fn hello(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
        // arg_check!(
        //     vm,
        //     args,
        //     required = [
        //         (url, Some(vm.ctx.str_type())),
        //         (handler, Some(vm.ctx.function_type()))
        //     ],
        //     optional = [(reject_handler, Some(vm.ctx.function_type()))]
        // );
        println!("HELLO!");
        Ok(vm.get_none())
    }

    pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
        py_module!(ctx, "qs", {
            "hello" => ctx.new_rustfunc(hello),
            "draw_square" => ctx.new_rustfunc(draw_square)
        })
    }

    pub fn setup_qs_module(vm: &mut VirtualMachine) {
        vm.stdlib_inits.insert("qs".to_string(), mk_module);
    }

impl State for PickItUp {
    fn new() -> Result<Self> {
        let mut vm = VirtualMachine::new();
        let source = Rc::new(RefCell::new(Asset::new(load_file("run.py").map(|v8| String::from_utf8(v8).unwrap()))));
        let mut ret = PickItUp {
            vm,
            source,
            update_fn: None,
            draw_fn: None,
            init_fn: None,
            state: None,
        };
        ret.load_code()?;
        Ok(ret)
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::R, ButtonState::Released) => {
                self.reload()?;
            }
            _ => {}
        };

        Ok(())
    }


    fn update(&mut self, window: &mut Window) -> Result<()> {
        if let (Some(update_fn), Some(state)) = (&self.update_fn, &self.state) {
            match self.vm.invoke(Rc::clone(update_fn), PyFuncArgs::new(vec![Rc::clone(state)], vec![])){
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_) => {
                }
            };
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.setup_module(window);
        if let (Some(draw_fn), Some(state)) = (&self.draw_fn, &self.state) {
            match self.vm.invoke(Rc::clone(draw_fn), PyFuncArgs::new(vec![Rc::clone(state)], vec![])) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_) => {
                }
            }
        }
        Ok(())
    }
}


fn main() {
    run::<PickItUp>("set-cursor", Vector::new(800, 600), Settings::default());
}
