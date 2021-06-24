extern crate olc_pixel_game_engine;

use std::{i32, string, usize};

use olc::{draw, draw_string, fill_circle, BLUE, WHITE};

use crate::olc_pixel_game_engine as olc;

struct point {
    pub x: f32,
    pub y: f32,
}
struct Splines {
    points: Vec<point>,
    tmp_point: point,
}

impl Splines {
    pub fn GenerateSpline(&mut self, tmp_t: f32) -> point {
        let size  = self.points.len();
        let p1 = (tmp_t as i32 + 1) % size as i32;
        let p2 = (&p1 + 1) % size as i32;
        let p3 = (&p2 + 1) % size as i32;
        let p0: i32;
        if p1 == 0 {
            p0 = size as i32 - 1;
        } else {
            p0 = (p1 - 1) % size as i32;
        }
        let mut t = tmp_t;

        t = &t - t as i32 as f32;
        
        if t > 1.0 {
          return point {
              x: 0.0,
              y: 0.0,
          }        
        }
        let tt = &t * &t;
        let ttt = &t * &t * &t;

        let y0: f32 = -ttt + 2.0 * tt - t;
        let y1: f32 = 3.0 * ttt - 5.0 * tt + 2.0;
        let y2: f32 = -3.0 * ttt + 4.0 * tt + t;
        let y3: f32 = ttt - tt;
        self.tmp_point.x = 0.5
            * (self.points[p0 as usize].x * y0
                + self.points[p1 as usize].x * y1
                + self.points[p2 as usize].x * y2
                + self.points[p3 as usize].x * y3);
        self.tmp_point.y = 0.5
            * (self.points[p0 as usize].y * y0
                + self.points[p1 as usize].y * y1
                + self.points[p2 as usize].y * y2
                + self.points[p3 as usize].y * y3);

        point {
            x: self.tmp_point.x,
            y: self.tmp_point.y,
        }
    }
}

struct Demo {
    Selecter: u32,
    splines: Splines,
    fMaker: f32,
}

impl olc::Application for Demo {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.splines.points.push(point { x: 10.0, y: 10.0 });
        self.splines.points.push(point { x: 20.0, y: 20.0 });
        self.splines.points.push(point { x: 30.0, y: 30.0 });
        self.splines.points.push(point { x: 40.0, y: 40.0 });
        self.splines.points.push(point { x: 50.0, y: 50.0 });
        self.splines.points.push(point { x: 60.0, y: 50.0 });
        self.splines.points.push(point { x: 70.0, y: 50.0 });
        self.splines.points.push(point { x: 80.0, y: 50.0 });

        self.Selecter = 0;
        self.fMaker = 3.0;
        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        olc::clear(olc::BLACK);
        //olc::draw_string(40, 40, "Hell7o, World!", olc::WHITE)?;

        if olc::get_key(olc::Key::D).pressed {
            if self.Selecter <= 0 {
                self.Selecter = self.splines.points.len() as u32 -1 ;
            } else {
                self.Selecter -= 1;
            }
        }
        if olc::get_key(olc::Key::A).pressed {
            if self.Selecter >= self.splines.points.len() as u32 -1 {
                self.Selecter = 0 as u32;
            } else {
                self.Selecter += 1;
            }
        }
        if olc::get_key(olc::Key::UP).held {
            self.splines.points[self.Selecter as usize].y -= 6.0 * _elapsed_time;
        }
        if olc::get_key(olc::Key::DOWN).held {
            self.splines.points[self.Selecter as usize].y += 6.0 * _elapsed_time;
        }
        if olc::get_key(olc::Key::LEFT).held {
            self.splines.points[self.Selecter as usize].x -= 6.0 * _elapsed_time;
        }
        if olc::get_key(olc::Key::RIGHT).held {
            self.splines.points[self.Selecter as usize].x += 6.0 * _elapsed_time;
        }
        //draw line
        let mut t: f32 = 0.0;
        while t < self.splines.points.len() as f32 + 1.0 {
            let point = self.splines.GenerateSpline(t);
            draw(point.x as i32, point.y as i32, WHITE);
            t += 0.005;
        }

        //draw points
        for (pos, e) in self.splines.points.iter().enumerate() {
            fill_circle(
                self.splines.points[pos].x as i32 ,
                self.splines.points[pos].y as i32 ,
                1,
                BLUE,
            );
            draw_string(e.x as i32, e.y as i32, &pos.to_string()[..], olc::RED)?;
        }
        Ok(())
    }
    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

fn main() {
    let mut demo = Demo {
        Selecter: 0,
        splines: Splines {
            points: Vec::new(),
            tmp_point: point { x: 0.0, y: 0.0 },
        },
        fMaker: 0.0,
    };
    olc::start("Hello, World!", &mut demo, 160, 80, 10, 10).unwrap();
}
