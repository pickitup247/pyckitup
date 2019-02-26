extern crate num_traits;
extern crate quicksilver;
#[macro_use]
extern crate rustpython_vm;
mod prelude;
mod qs;
use crate::prelude::*;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol};

struct PickItUp {
    vm: VirtualMachine,
    source: Rc<RefCell<Asset<String>>>,
    update_fn: Option<PyObjectRef>,
    draw_fn: Option<PyObjectRef>,
    init_fn: Option<PyObjectRef>,
    state: Option<PyObjectRef>,
}

fn handle_err(vm: &mut VirtualMachine, py_err: PyObjectRef) {
    let res = vm
        .to_pystr(&py_err)
        .unwrap_or_else(|_| "Error, and error getting error message".into());
    panic!(res);
}

impl PickItUp {
    fn load_code(&mut self) -> Result<()> {
        self.source.clone().borrow_mut().execute(|source| {
            let mode = compile::Mode::Exec;
            let code =
                compile::compile(&source, &mode, "<qs>".to_string(), self.vm.ctx.code_type())
                    .map_err(|err| format!("Error parsing Python code: {}", err))
                    .unwrap();

            let builtin = self.vm.get_builtin_scope();
            let scope = self.vm.context().new_scope(Some(builtin));
            let result = self.vm.run_code_obj(code, scope.clone());
            match result {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_res) => {}
            };

            let init_fn = scope.get_item("init").unwrap();
            self.state = Some(
                self.vm
                    .invoke(Rc::clone(&init_fn), PyFuncArgs::new(vec![], vec![]))
                    .unwrap(),
            );
            self.init_fn = Some(init_fn);
            self.update_fn = Some(scope.get_item("update").unwrap());
            self.draw_fn = Some(scope.get_item("draw").unwrap());

            Ok(())
        })?;
        Ok(())
    }

    fn reload(&mut self) -> Result<()> {
        self.source = Rc::new(RefCell::new(Asset::new(
            load_file("run.py").map(|v8| String::from_utf8(v8).unwrap()),
        )));
        self.load_code()
    }

    fn setup_module(&mut self) -> Result<()> {
        self.vm
            .stdlib_inits
            .insert(MOD_NAME.to_string(), Box::new(qs::mk_module));
        Ok(())
    }

    fn update_window_ptr(&mut self, window: &mut Window) -> Result<()> {
        let ptr = (window as *mut Window) as usize;
        let modules = self.vm.sys_module.get_attr("modules").unwrap();
        let qs = modules.get_item(MOD_NAME).unwrap();
        qs.set_item(&self.vm.ctx, "window", self.vm.new_int(ptr));
        Ok(())
    }
}

impl State for PickItUp {
    fn new() -> Result<Self> {
        let vm = VirtualMachine::new();
        let source = Rc::new(RefCell::new(Asset::new(
            load_file("run.py").map(|v8| String::from_utf8(v8).unwrap()),
        )));
        let mut ret = PickItUp {
            vm,
            source,
            update_fn: None,
            draw_fn: None,
            init_fn: None,
            state: None,
        };
        ret.setup_module()?;
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
        self.update_window_ptr(window)?;
        if let (Some(update_fn), Some(state)) = (&self.update_fn, &self.state) {
            match self.vm.invoke(
                Rc::clone(update_fn),
                PyFuncArgs::new(vec![Rc::clone(state)], vec![]),
            ) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_) => {}
            };
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        if let (Some(draw_fn), Some(state)) = (&self.draw_fn, &self.state) {
            match self.vm.invoke(
                Rc::clone(draw_fn),
                PyFuncArgs::new(vec![Rc::clone(state)], vec![]),
            ) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_) => {}
            }
        }
        Ok(())
    }
}

fn main() {
    run::<PickItUp>("set-cursor", Vector::new(800, 600), Settings::default());
}
