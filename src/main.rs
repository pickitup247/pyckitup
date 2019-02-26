extern crate num_traits;
extern crate quicksilver;
#[macro_use]
extern crate rustpython_vm;
mod prelude;
mod qs;
mod sprites;
mod anim;

use crate::prelude::*;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol};

struct PickItUp {
    vm: VirtualMachine,
    source: Rc<RefCell<Asset<String>>>,

    sprites: Option<Asset<Sprites>>,

    update_fn: Option<PyObjectRef>,
    draw_fn: Option<PyObjectRef>,
    state: Option<PyObjectRef>,

    /// images, anims, sounds
    resources: Resources,
}

fn handle_err(vm: &mut VirtualMachine, py_err: PyObjectRef) {
    let res = vm
        .to_pystr(&py_err)
        .unwrap_or_else(|_| "Error, and error getting error message".into());
    panic!(res);
}

impl PickItUp {
    fn load_code(&mut self) -> Result<()> {
        Rc::clone(&self.source).borrow_mut().execute(|source| {
            let mode = compile::Mode::Exec;
            let code =
                compile::compile(&source, &mode, "<qs>".to_string(), self.vm.ctx.code_type())
                    .map_err(|err| format!("Error parsing Python code: {}", err))
                    .expect("cannot compile");

            let builtin = self.vm.get_builtin_scope();
            let scope = self.vm.context().new_scope(Some(builtin));
            let result = self.vm.run_code_obj(code, scope.clone());
            match result {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err);
                }
                Ok(_res) => {}
            };

            let resources_ptr = (&self.resources as *const Resources) as usize;
            let modules = self.vm.sys_module.get_attr("modules").expect("no attr modules");
            let qs = modules.get_item(MOD_NAME).expect("no module called qs");
            qs.set_item(&self.vm.ctx, "resources", self.vm.new_int(resources_ptr));

            let init_fn = scope.get_item("init").expect("no init function");
            self.state = Some(
                self.vm
                    .invoke(Rc::clone(&init_fn), PyFuncArgs::new(vec![], vec![]))
                    .expect("cannot invoke init function"),
            );
            dbg!(&self.resources);
            // create sprites based on resources
            self.sprites = Some(Asset::new(Sprites::new(self.resources.clone())));

            self.update_fn = Some(scope.get_item("update").expect("no update function"));
            self.draw_fn = Some(scope.get_item("draw").expect("no draw function"));

            let sprites_ptr = (self.sprites.as_ref().unwrap() as *const Asset<Sprites>) as usize;
            qs.set_item(&self.vm.ctx, "sprites", self.vm.new_int(sprites_ptr));

            Ok(())
        })?;
        Ok(())
    }

    fn reload(&mut self) -> Result<()> {
        self.source = Rc::new(RefCell::new(Asset::new(
            load_file("run.py").map(|v8| String::from_utf8(v8).expect("cannot load run.py")),
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
        let window_ptr = (window as *mut Window) as usize;
        let modules = self.vm.sys_module.get_attr("modules").expect("modules");
        let qs = modules.get_item(MOD_NAME).expect("MOD_NAME");
        qs.set_item(&self.vm.ctx, "window", self.vm.new_int(window_ptr));

        if self.sprites.is_some() {
            let sprites_ptr = (self.sprites.as_ref().unwrap() as *const Asset<Sprites>) as usize;
            qs.set_item(&self.vm.ctx, "sprites", self.vm.new_int(sprites_ptr));
        }

        Ok(())
    }
}

impl State for PickItUp {
    fn new() -> Result<Self> {
        let vm = VirtualMachine::new();
        let source = Rc::new(RefCell::new(Asset::new(
            load_file("run.py").map(|v8| String::from_utf8(v8).expect("cannot convert from utf8")),
        )));
        let sprites = None;
        let resources = (vec![],vec![],vec![]);
        let mut ret = PickItUp {
            vm,
            source,
            sprites,

            update_fn: None,
            draw_fn: None,
            state: None,
            resources,
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
