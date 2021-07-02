extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

use rand::Rng;
use std::{i32, string, usize};

use olc::{BLUE, GREEN, Pixel, WHITE, c_rand, draw, draw_string};
// Very simple example application that prints "Hello, World!" on screen.
struct star{
    x: f32,
    y: f32,
    color: Pixel,
    radius : f32,
    speed_x : f32,
    speed_y : f32,
    tmpframe: f32
}
impl star{
    pub fn updateitself(&mut self,_elapsed_time:f32){
        self.x += 5.0 * (self.speed_x  as f32 * _elapsed_time )  ;
        self.y += 5.0 * (self.speed_y  as f32 * _elapsed_time );
        self.tmpframe+= _elapsed_time ;
        if self.tmpframe > 5.0{
            self.radius += 2.0;
            self.tmpframe = 0.0;
        }
    }
    
}
struct StarField {
    VecStar : Vec<star> 
}
impl StarField{
    pub fn GenerateStar(&self)-> star{
        let mut rng = rand::thread_rng();
        let mut tmpStar = star{
            x: rng.gen_range(10.0..700.0),
            y: rng.gen_range(10.0..700.0),
            speed_x : rng.gen_range(0.0..4.0),
            speed_y : rng.gen_range(0.0..4.0),
            color: olc::WHITE,
            radius : 1.0,
            tmpframe: 0.0,       
        };
        if tmpStar.y <= 600.0{
            tmpStar.speed_y*= -1.0;
        }
        if tmpStar.x <= 360.0{
            tmpStar.speed_x*= -1.0;
        }

        tmpStar
    }
}

impl olc::Application for StarField {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    let tmp_star = self.GenerateStar();
    self.VecStar.push(tmp_star);
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.
    // Clears screen and sets black colour.
    olc::clear(olc::BLACK);

    let tmp_star = self.GenerateStar();
    self.VecStar.push(tmp_star);
    for (index,e) in &mut self.VecStar.iter_mut().enumerate(){
        //只绘制在显示区域内的圆圈
        if e.x > 0.0 && e.x < 720.0 && e.y>0.0 && e.y<720.0 && e.radius <13.0 {
            
            olc::fill_circle(e.x as i32, e.y as i32 , e.radius as i32 , e.color);
            e.updateitself(_elapsed_time * 20.0);
        }
    }
    if self.VecStar.len() > 300{
        //清除不在区域内的圆圈
        self.VecStar.retain(|x| x.x > 0.0 && x.x < 720.0 && x.y>0.0 && x.y<720.0 && x.radius <13.0 );
    }
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }


}

fn main() {
  let mut example = StarField {
      VecStar : Vec::new(), 
  };
  // and starts the main game loop.
  olc::start("Hello, World!", &mut example, 720, 720, 1, 1).unwrap();
}