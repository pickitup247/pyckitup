extern crate num_traits;
extern crate quicksilver;
extern crate clap;
#[macro_use]
extern crate rustpython_vm;
use clap::{Arg, App, SubCommand};
mod prelude;
mod qs;
mod sprites;
mod anim;

use crate::prelude::*;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol};

struct PickItUp {
    vm: VirtualMachine,
    sprites: Option<Asset<Sprites>>,

    update_fn: Option<PyObjectRef>,
    draw_fn: Option<PyObjectRef>,
    event_fn: Option<PyObjectRef>,
    state: Option<PyObjectRef>,

    resources: Resources,
    loaded: bool,
}

fn handle_err(vm: &mut VirtualMachine, py_err: PyObjectRef) -> Result<()> {
    return Err(Error::ContextError(vm
        .to_pystr(&py_err)
        .unwrap_or_else(|_| "Error, and error getting error message".into())));
}

impl PickItUp {
    fn load_code(&mut self, mut source: &str) -> Result<()> {
        let mode = compile::Mode::Exec;
        let code =
            compile::compile(&source, &mode, "<qs>".to_string(), self.vm.ctx.code_type())
                .map_err(|err| Error::ContextError(format!("Error parsing Python code: {}", err)))?;

        let builtin = self.vm.get_builtin_scope();
        let scope = self.vm.context().new_scope(Some(builtin));
        let result = self.vm.run_code_obj(code, scope.clone());
        match result {
            Err(py_err) => {
                handle_err(&mut self.vm, py_err)?;
            }
            Ok(_res) => {

            }
        };

        let resources_ptr = (&self.resources as *const Resources) as usize;
        let modules = self.vm.sys_module.get_attr("modules").ok_or(Error::ContextError("no attr modules".to_owned()))?;
        let qs = modules.get_item(MOD_NAME).ok_or(Error::ContextError("no module called qs".to_owned()))?;
        qs.set_item(&self.vm.ctx, "resources", self.vm.new_int(resources_ptr));

        let init_fn = scope.get_item("init").ok_or(Error::ContextError("no init function".to_owned()))?;
        self.state = Some(
            self.vm
                .invoke(Rc::clone(&init_fn), PyFuncArgs::new(vec![], vec![]))
                .map_err(|_|Error::ContextError("cannot invoke init function".to_owned()))?,
        );
        // create sprites based on resources
        self.sprites = Some(Asset::new(Sprites::new(self.resources.clone())));

        self.update_fn = Some(scope.get_item("update").ok_or(Error::ContextError("no update function".to_owned()))?);
        self.draw_fn = Some(scope.get_item("draw").ok_or(Error::ContextError("no draw function".to_owned()))?);
        self.event_fn = Some(scope.get_item("event").ok_or(Error::ContextError("no event function".to_owned()))?);

        let sprites_ptr = (self.sprites.as_ref().unwrap() as *const Asset<Sprites>) as usize;
        qs.set_item(&self.vm.ctx, "sprites", self.vm.new_int(sprites_ptr));

        self.loaded = true;

        Ok(())
    }

    // fn reload(&mut self) -> Result<()> {
    //     self.source = Asset::new(
    //         load_file("run.py").map(|v8| String::from_utf8(v8).unwrap()),
    //     );
    //     self.load_code()
    // }

    fn setup_module(&mut self) -> Result<()> {
        self.vm
            .stdlib_inits
            .insert(MOD_NAME.to_string(), Box::new(qs::mk_module));

        Ok(())
    }

    fn update_window_ptr(&mut self, window: &mut Window) -> Result<()> {
        let window_ptr = (window as *mut Window) as usize;
        let modules = self.vm.sys_module.get_attr("modules").ok_or(Error::ContextError("modules".to_owned()))?;
        let qs = modules.get_item(MOD_NAME).ok_or(Error::ContextError("MOD_NAME".to_owned()))?;
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
        let sprites = None;
        let resources = (vec![],vec![],vec![]);
        let mut ret = PickItUp {
            vm,
            sprites,

            update_fn: None,
            draw_fn: None,
            event_fn: None,
            state: None,
            resources,
            loaded: false,
        };
        ret.setup_module()?;
        // save_raw("test", "run.py", "import qs\n".as_bytes())?;
        let source = if cfg!(target_arch = "wasm32") {
            String::from_utf8(load_raw("test", "run.py")?).unwrap()
        } else {
            use std::io::Read;
            let mut s = String::new();
            let dir = {
                let dir = std::env::current_dir().unwrap();
                if dir.ends_with("static") {
                    "..".to_owned()
                } else {
                    dir.as_os_str().to_str().unwrap().to_owned()
                }
            };

            dbg!(dir.clone()+"/run.py");

            std::fs::File::open(dir+"/run.py").unwrap().read_to_string(&mut s).unwrap();
            s
        };
        ret.load_code(&source)?;
        Ok(ret)
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        // match event {
        //     Event::Key(Key::R, ButtonState::Released) => {
        //         self.reload()?;
        //     }
        //     _ => {}
        // };


        if let (Some(event_fn), Some(state)) = (&self.event_fn, &self.state) {
            let evt = to_pyobjref(&mut self.vm, event);
            match self.vm.invoke(
                Rc::clone(event_fn),
                PyFuncArgs::new(vec![Rc::clone(state), evt], vec![]),
            ) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err)?;
                }
                Ok(_) => {}
            }
        }

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if !self.loaded {return Ok(())}

        self.update_window_ptr(window)?;


        if let Some(ref mut sprites) = &mut self.sprites {
            sprites.execute(|spr| {
                spr.update_anim(window)?;
                Ok(())
            })?;
        }


        if let (Some(update_fn), Some(state)) = (&self.update_fn, &self.state) {
            match self.vm.invoke(
                Rc::clone(update_fn),
                PyFuncArgs::new(vec![Rc::clone(state)], vec![]),
            ) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err)?;
                }
                Ok(_) => {}
            };
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        if !self.loaded {return Ok(())}

        if let (Some(draw_fn), Some(state)) = (&self.draw_fn, &self.state) {
            match self.vm.invoke(
                Rc::clone(draw_fn),
                PyFuncArgs::new(vec![Rc::clone(state)], vec![]),
            ) {
                Err(py_err) => {
                    handle_err(&mut self.vm, py_err)?;
                }
                Ok(_) => {}
            }
        }
        Ok(())
    }
}

fn to_pyobjref(vm: &mut VirtualMachine, event: &Event) -> PyObjectRef {
    let d = vm.new_dict();
    macro_rules! set {
        ($d:ident, $key:expr, $val:ident) => {
            d.set_item(&vm.ctx, stringify!($key), vm.new_str(stringify!($val).to_owned()));
        }
    };
    macro_rules! set_str {
        ($d:ident, $key:expr, $val:expr) => {
            d.set_item(&vm.ctx, stringify!($key), vm.new_str($val.to_owned()));
        }
    };
    match event {
        Event::Closed => { set!(d, event, closed); },
        Event::Focused => {set!(d, event, focused);},
        Event::Unfocused => { set!(d, event, unfocused); }
        Event::Key(key, state) => {
            set!(d, event, key);
            set_str!(d, key, format!("{:?}", key));
            set_str!(d, state, format!("{:?}", state));
        },
        Event::Typed(c) => {
            set!(d, event, typed);
            set_str!(d, char, format!("{:?}", c));
        },
        Event::MouseMoved(v) => {
            set!(d, event, mouse_moved);
            d.set_item(&vm.ctx, "x", vm.new_int(v.x));
            d.set_item(&vm.ctx, "y", vm.new_int(v.y));
        },
        Event::MouseEntered => { set!(d, event, mouse_entered); }
        Event::MouseExited => { set!(d, event, mouse_exited); }
        Event::MouseWheel(v) => {
            set!(d, event, mouse_wheel);
            d.set_item(&vm.ctx, "x", vm.new_int(v.x));
            d.set_item(&vm.ctx, "y", vm.new_int(v.y));
        } ,
        Event::MouseButton(button, state) => {
            set!(d, event, mouse_button);
            set_str!(d, button, format!("{:?}", button));
            set_str!(d, state, format!("{:?}", state));
        },
        // Event::GamepadAxis(i32, GamepadAxis, f32),
        // Event::GamepadButton(i32, GamepadButton, ButtonState),
        // Event::GamepadConnected(i32),
        // Event::GamepadDisconnected(i32)
        t => panic!("TODO  {:#?}",  t),
    }
    d
}

fn main() {
    let matches = App::new("pickitup")
                        .version("0.1")
                        .arg(Arg::with_name("size")
                            .short("s")
                            .long("size")
                            .value_name("SIZE")
                            .help("size, WxH, defaults to 480x270")
                            .takes_value(true))
                        .get_matches();
    let (w, h) = {
        let size = matches.value_of("size").unwrap_or("480x270");
        let ret: Vec<i32> = size.split("x").map(|i| i.parse().unwrap()).collect();
        (ret[0], ret[1])
    };

    run::<PickItUp>("pickitup", Vector::new(w, h), Settings::default());
}