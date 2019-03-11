extern crate num_traits;
extern crate quicksilver;
#[cfg(not(target_arch = "wasm32"))]
extern crate clap;
#[cfg(not(target_arch = "wasm32"))]
extern crate fs_extra;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rustpython_vm;
#[cfg(not(target_arch = "wasm32"))]
use clap::{Arg, App, SubCommand};
mod prelude;
mod qs;
mod sprites;
mod anim;

use crate::prelude::*;
use rustpython_vm::pyobject::{AttributeProtocol, DictProtocol};
use std::path::Path;

static mut FNAME: Option<String> = None;

struct PickItUp {
    vm: VirtualMachine,
    sprites: Option<Asset<Sprites>>,

    update_fn: Option<PyObjectRef>,
    draw_fn: Option<PyObjectRef>,
    onload_fn: Option<PyObjectRef>,
    event_fn: Option<PyObjectRef>,
    state: Option<PyObjectRef>,

    window_initialized: bool,

    resources: Resources,
    code_loaded: bool,
}

fn handle_err(vm: &mut VirtualMachine, py_err: PyObjectRef) -> Result<()> {
    return Err(Error::ContextError(vm
        .to_pystr(&py_err)
        .unwrap_or_else(|_| "Error, and error getting error message".into())));
}

impl PickItUp {
    fn load_code(&mut self, source: &str, code_path: String) -> Result<()> {
        let mode = compile::Mode::Exec;
        let code =
            compile::compile(&source, &mode, code_path, self.vm.ctx.code_type())
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

        self.state = Some(match scope.get_item("init") {
            Some(init_fn) => self.vm
                .invoke(Rc::clone(&init_fn), PyFuncArgs::new(vec![], vec![]))
                .map_err(|_|Error::ContextError("cannot invoke init function".to_owned()))?,
            None => self.vm.get_none(),
        });
        // create sprites based on resources
        self.sprites = Some(Asset::new(Sprites::new(self.resources.clone())));

        self.update_fn = scope.get_item("update");
        self.onload_fn = scope.get_item("onload");
        self.draw_fn = scope.get_item("draw");
        self.event_fn = scope.get_item("event");

        let sprites_ptr = (self.sprites.as_ref().unwrap() as *const Asset<Sprites>) as usize;
        qs.set_item(&self.vm.ctx, "sprites", self.vm.new_int(sprites_ptr));

        self.code_loaded = true;

        Ok(())
    }

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
            onload_fn: None,
            state: None,
            resources,
            code_loaded: false,
            window_initialized: false,
        };
        ret.setup_module()?;
        let (source, code_path) = if cfg!(target_arch = "wasm32") {
            (
                String::from_utf8(load_raw("test", "run.py")?).unwrap(),
                "<qs>".to_owned(),
            )
        } else {
            use std::io::Read;
            // requires special handling because of complications in static folder of cargo-web
            let dir = {
                let dir = std::env::current_dir().unwrap();
                if dir.ends_with("static") {
                    "..".to_owned()
                } else {
                    dir.as_os_str().to_str().unwrap().to_owned()
                }
            };

            unsafe {
                let code_path = dir.clone() + "/" + FNAME.as_ref().unwrap();
                let mut s = String::new();
                let f = std::fs::File::open(&code_path);
                match f {
                    Err(_) => panic!(format!("File `{}` is not found.", FNAME.as_ref().unwrap())),
                    Ok(mut f) => {
                        f.read_to_string(&mut s).unwrap();
                        (s, code_path.to_owned())
                    }
                }
            }

        };
        ret.load_code(&source, code_path)?;
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
        if !self.code_loaded {return Ok(())}

        self.update_window_ptr(window)?;

        if !self.window_initialized {
            if let (Some(onload_fn), Some(state)) = (&self.onload_fn, &self.state) {
                match self.vm.invoke(
                    Rc::clone(onload_fn),
                    PyFuncArgs::new(vec![Rc::clone(state)], vec![]),
                ) {
                    Err(py_err) => {
                        handle_err(&mut self.vm, py_err)?;
                    }
                    Ok(_) => {}
                };
                self.window_initialized = true;
            }
        }

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
        if !self.code_loaded {return Ok(())}

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

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let matches = App::new("pickitup")
                        .version("0.1")
                        .arg(Arg::with_name("size")
                            .short("s")
                            .long("size")
                            .value_name("SIZE")
                            .help("size, WxH, defaults to 480x270")
                            .takes_value(true))
                        .arg(Arg::with_name("filename")
                            .value_name("FNAME")
                            .help("filename, defaults to run.py")
                            .takes_value(true))
                        .subcommand(SubCommand::with_name("init")
                            .about("initialize a new pyckitup project")
                            .arg(
                                Arg::with_name("project")
                                .help("name of the project")
                            )
                        )
                        .subcommand(SubCommand::with_name("build")
                            .about("deploy for web")
                        )
                        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        pyckitup_init(&matches);
    } else if let Some(matches) = matches.subcommand_matches("build") {
        pyckitup_wasm();
    } else {
        pyckitup_run(&matches);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    run::<PickItUp>("pickitup", Vector::new(800, 600), Settings::default());
}

#[cfg(not(target_arch = "wasm32"))]
fn pyckitup_wasm() {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    fs_extra::dir::copy("./static", "./build", &options);
    std::fs::write("./build/pyckitup.js", include_bytes!("../target/deploy/pyckitup.js").to_vec());
    std::fs::write("./build/pyckitup.wasm", include_bytes!("../target/deploy/pyckitup.wasm").to_vec());
    std::fs::write("./build/index.html", include_bytes!("../target/deploy/index.html").to_vec());
    std::fs::write("./build/server.py", include_bytes!("../server.py").to_vec());
}

#[cfg(not(target_arch = "wasm32"))]
fn pyckitup_init(matches: &clap::ArgMatches) {
    let project_name = matches.value_of("project").unwrap_or("new_pyckitup_project");
    if Path::new(&format!("./{}", project_name)).exists() {
        println!("Path ./{} already exists. Doing nothing.", project_name);
        std::process::exit(1);
    }

    println!("Initializing pyckitup project in directory `./{}`", project_name);
    std::fs::create_dir(&format!("./{}/", project_name));
    std::fs::create_dir(&format!("./{}/static/", project_name));
    std::fs::write(&format!("./{}/static/click.wav", project_name), include_bytes!("../static/click.wav").to_vec());
    std::fs::write(&format!("./{}/run.py", project_name), include_bytes!("../examples/clock.py").to_vec());
    std::fs::write(&format!("./{}/common.py", project_name), include_bytes!("../examples/common.py").to_vec());
    println!("Initialized. To run: `pyckitup`");
}

#[cfg(not(target_arch = "wasm32"))]
fn pyckitup_run(matches: &clap::ArgMatches) {
    let (w, h) = {
        let size = matches.value_of("size").unwrap_or("800x600");
        let ret: Vec<i32> = size.split("x").map(|i| i.parse().unwrap()).collect();
        (ret[0], ret[1])
    };

    let fname = matches.value_of("filename").unwrap_or("run.py");
    unsafe { FNAME = Some(fname.to_owned()); }

    run::<PickItUp>("pickitup", Vector::new(w, h), Settings::default());
}