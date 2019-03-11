use crate::prelude::*;

pub struct Animation {
    imgs: Vec<Image>,
    pub played: bool,
    nframes: usize,
    /// in seconds
    duration: f64,
    pub current_t: f64,
    frame_w: f32,
    frame_h: f32,
}

impl Animation {

    #[allow(unused)]
    pub fn new(src: &'static str, nframes: usize, duration: f64) -> impl Future<Item=Self, Error=Error> {
        load_file(src)
            .map(|data| Image::from_bytes(data.as_slice()))
            .map(move |sheet| {
                Animation::from_image(sheet.unwrap(), nframes, duration)
            })
            // .and_then(result)
    }

    pub fn from_image(image: Image, nframes: usize, duration: f64) -> Animation {
        let frame_w = image.area().width() / nframes as f32;
        let frame_h = image.area().height();

        let mut imgs = Vec::new();
        for i in 0..nframes {
            let region = Rectangle::new(
                Vector { x: i as f32 * frame_w , y: 0. },
                Vector { x: frame_w, y: frame_h  },
            );
            imgs.push(image.subimage(region));
        }

        Animation {
            imgs,
            played: false,
            nframes,
            duration,
            current_t: 0.,
            frame_w,
            frame_h,
        }
    }

    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        self.current_t += window.update_rate() * 0.001;
        if self.current_t >= self.duration {
            self.current_t -= self.duration
        }

        if self.nth() == self.nframes - 1 {
            self.played = true;
        }

        Ok(())
    }

    pub fn nth(&self) -> usize {
        let frame = (self.current_t / self.duration * self.nframes as f64).floor() as usize + 1;
        let nth = frame % self.nframes;
        nth
    }

    pub fn current_frame(&self) -> &Image {
        let src = &self.imgs[self.nth()];
        src
    }

    #[allow(unused)]
    pub fn draw(&self, window: &mut Window, pos_x: f32, pos_y: f32, scale: f32) {
        if self.played {
            return;
        }
        let src = &self.imgs[self.nth()];

        // let dest = Point2::new( pos_x, pos_y );
        // let scale = Point2::new(5., 5.);

        window.draw_ex(&
            Rectangle::new(
                Vector::new(pos_x, pos_y),
                Vector::new(self.frame_w, self.frame_h)
            ),
            Img(&src),
            Transform::scale(Vector::new(scale, scale)),
            0,
        );

        // window.draw(&src.area().with_center((240., 135.)), Img(&src));
    }

    #[allow(unused)]
    pub fn play(&mut self) -> Result<()> {
        self.played = false;
        self.current_t = 0.;
        Ok(())
    }

    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }
}