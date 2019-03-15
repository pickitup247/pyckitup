use crate::prelude::*;

mod utils;
mod shapes;
mod inits;
mod draw;
mod window;
use crate::qs::shapes::*;
use crate::qs::inits::*;
use crate::qs::draw::*;
use crate::qs::window::*;

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, MOD_NAME, {
        "rect" => ctx.new_rustfunc(rect),
        "circ" => ctx.new_rustfunc(circ),
        "triangle" => ctx.new_rustfunc(triangle),
        "line" => ctx.new_rustfunc(line),

        "sprite" => ctx.new_rustfunc(sprite),
        "anim" => ctx.new_rustfunc(anim),
        "sound" => ctx.new_rustfunc(sound),
        "text" => ctx.new_rustfunc(text),

        "clear" => ctx.new_rustfunc(window_clear),

        "init_sprites" => ctx.new_rustfunc(init_sprites),
        "init_anims" => ctx.new_rustfunc(init_anims),
        "init_sounds" => ctx.new_rustfunc(init_sounds),
        "init_fonts" => ctx.new_rustfunc(init_fonts),

        "mouse_pos" => ctx.new_rustfunc(mouse_pos),
        "mouse_wheel_delta" => ctx.new_rustfunc(mouse_wheel_delta),

        "keyboard" => ctx.new_rustfunc(keyboard),
        "keyboard_bool" => ctx.new_rustfunc(keyboard_bool),


        "set_view" => ctx.new_rustfunc(set_view),
        "update_rate" => ctx.new_rustfunc(update_rate),
        "set_update_rate" => ctx.new_rustfunc(set_update_rate),

        "set_anim_duration" => ctx.new_rustfunc(set_anim_duration),

    })
}
