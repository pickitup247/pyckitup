pub use std::cell::RefCell;
pub use std::rc::Rc;


// imgs, anims, sounds
pub type Resources = (Vec<(String, String)>, Vec<(String, String, (usize,usize,f64))>, Vec<(String, String)>);
pub use crate::sprites::Sprites;
pub use crate::anim::Animation;
pub use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::{run, Asset, Settings, State, Window},
};

pub use quicksilver::{
    combinators::{join_all, result},
    geom::Shape,
    graphics::{Background::Img, Drawable, Font, FontStyle},
    input::{ButtonState, Key, MouseButton},
    lifecycle::Event,
    load_file,
    sound::Sound,
    Error, Future, Result,
};

pub use rustpython_vm::{
    compile,
    obj::objstr,
    obj::{
        objfloat,
        objint::{self, PyInt},
        objsequence::get_elements,
    },
    pyobject::PyObjectPayload,
    pyobject::{PyContext, PyFuncArgs, PyObjectRef, PyResult},
    stdlib::StdlibInitFunc,
    VirtualMachine,
};

pub const MOD_NAME: &'static str = "qs";
