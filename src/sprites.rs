use crate::prelude::*;
use std::collections::HashMap;

pub struct Sprites {
    pub items: HashMap<String, Image>,
    pub anims: HashMap<String, Animation>,
    pub sounds: HashMap<String, Sound>,
    strings: HashMap<String, Image>,
}

impl Sprites {

    pub fn new((imgs, anims, sounds): Resources) -> impl Future<Item=Self, Error=Error> {
        let img_futs = imgs.into_iter().map(move |(name, src)| {
            load_file(src.to_owned())
                .map(move |data|
                    (name, Image::from_bytes(data.as_slice()).unwrap())
                )
        });

        let anim_futs = anims.into_iter().map(move |(name, src, dims)| {
            load_file(src.to_owned())
                .map(move |data|
                    (name, (dims, Image::from_bytes(data.as_slice()).unwrap()))
                )
        });

        let fut_anim = join_all(anim_futs)
            .map(|vec| {
                let mut anims = HashMap::new();
                for (src, ((frame_w, frame_h, dur), img)) in vec.into_iter() {
                    let anim = Animation::from_image(img, frame_w, frame_h, dur);
                    anims.insert(src.to_string(), anim);
                }
                anims
            });

        let sound_futs = sounds.into_iter()
            .map(|(name, src)|
                Sound::load(src.to_owned())
                .map(
                    move |sound| (name, sound)
                )
            );

        let fut_sounds = join_all(sound_futs)
            .map(|vec| {
                let mut sounds = HashMap::new();
                for (name, sound) in vec.into_iter() {
                    sounds.insert(name.to_string(), sound);
                }
                sounds
            });


        let fut_items = join_all(img_futs)
            .map(|vec| {
                let mut items = HashMap::new();
                for (src, img) in vec.into_iter() {
                    items.insert(src.to_string(), img);
                }


                items
            });

        let ret = fut_anim.join3(fut_items, fut_sounds)
            .map(|(anims,items,sounds)| Sprites {
                items, anims, sounds, strings: HashMap::new(),
            });
        ret
    }

    pub fn get_img(&self, name: &str) -> Option<&Image> {
        self.items.get(name)
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

}

impl Sprites {

    pub fn get_anim(&self, name: &str) -> Option<&Animation> {
        self.anims.get(name)
    }

    pub fn get_anim_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.anims.get_mut(name)
    }

    pub fn update_anim(&mut self, window: &mut Window) -> Result<()> {
        for i in self.anims.values_mut() {
            i.update(window)?;
        }
        Ok(())
    }

    pub fn set_duration(&mut self, duration: f64) -> Result<()> {
        for i in self.anims.values_mut() {
            i.set_duration(duration);
        }
        Ok(())
    }


    // pub fn draw_anim(&mut self, window: &mut Window, pos_x: f32, pos_y: f32, scale: f32) -> Result<()> {
    //     for i in self.anims.values_mut() {
    //         i.draw(window, pos_x, pos_y, scale);
    //     }
    //     Ok(())
    // }

    // pub fn render_str(&mut self, s: &str) -> Image {
    //     if self.strings.contains_key(s) {
    //         self.strings.get(s).unwrap().to_owned()
    //     } else {
    //         let img = Font::from_slice(include_bytes!("../static/fonts/VGATypewriter.ttf"))
    //                 .and_then(move |font| {
    //                     let style = FontStyle::new(90.0, Color::BLACK);
    //                     font.render(&s, &style)
    //                 }).unwrap();
    //         self.strings.insert(s.to_owned(), img.clone());
    //         img
    //     }
    // }

}