extern crate olc_pixel_game_engine;

use rand::Rng;
use std::{i32, string, usize};

use olc::{BLUE, GREEN, WHITE, c_rand, draw, draw_string};

use crate::olc_pixel_game_engine as olc;

struct point{
    pub x:f32,
    pub y:f32,
}
struct Points {
    pub points :Vec<point>,
    pub startPoint: point,
    pub endPoint:point,
    pub needFindNext:bool,
    pub tmpFrame : f32,
    pub Maxspeed :f32
}
impl Points{
    //generate the path of ball 
    pub fn GeneratePath(&self)->point{
        
        let x1 = self.startPoint.x;
        let y1 = self.startPoint.y;
        let x2 = self.endPoint.x;
        let y2 = self.endPoint.y;
        //TODO 初始化必须赋值？？？
        let mut Rpoint =point 
        {
             x:0.0,
             y:0.0
        };
        let direction_x : f32;
        if self.startPoint.x  < self.endPoint.x{
            direction_x = 1.0;
        }else{
            direction_x = -1.0;
        }
        Rpoint.x = self.startPoint.x + direction_x;
        //Rpoint.y = ((y2-y1)/(x2-x1) * self.startPoint.x + y2 - (y2-y1)/(x2-x1)* x2) as f32;
        Rpoint.y = self.startPoint.y + (y2-y1) / (x2-x1) * direction_x;
        Rpoint
    }
    //generate next random number
    pub fn GenerateNumber(&self)->usize{
        let mut rng = rand::thread_rng();
        let mut pointNumber = rng.gen_range(0..self.points.len()-1);
        if self.points[pointNumber].x == self.startPoint.x && self.points[pointNumber].y == self.startPoint.y {
            pointNumber+=1;
            if pointNumber == self.points.len(){
                pointNumber = 0;
            }
        }
        pointNumber as usize
    }
}

impl olc::Application for Points {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.points.push(point { x: 10.0, y: 10.0 });
    self.points.push(point { x: 20.0, y: 40.0 });
    self.points.push(point { x: 30.0, y: 20.0 });
    self.points.push(point { x: 40.0, y: 44.0 });
    self.points.push(point { x: 50.0, y: 34.0 });
    self.points.push(point { x: 60.0, y: 65.0 });
    self.points.push(point { x: 70.0, y: 34.0 });

    self.startPoint.x = self.points[0].x;
    self.startPoint.y = self.points[0].y;   
    
    let pointNumber = self.GenerateNumber();

    self.endPoint.x = self.points[pointNumber].x;
    self.endPoint.y = self.points[pointNumber].y;   

    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.
    // Clears screen and sets black colour.
    olc::clear(olc::BLACK);
    // set the value of frame  设置每一帧的时间间隔 
    self.tmpFrame += _elapsed_time * 1000.0 as f32;
    if self.tmpFrame > self.Maxspeed{
    // find next point 
        if self.needFindNext {
            let pointNumber = self.GenerateNumber();
            self.endPoint.x = self.points[pointNumber].x;
            self.endPoint.y = self.points[pointNumber].y;
            self.needFindNext = false;
        }
        else {
            self.startPoint  = self.GeneratePath();

            if self.endPoint.x == self.startPoint.x && self.startPoint.y == self.endPoint.y{
                self.startPoint.x = self.endPoint.x;
                self.startPoint.y = self.endPoint.y;
                self.needFindNext = true;
            }
        }
        self.tmpFrame = 0.0;
    }
    // draw the points
    for (pos,e) in self.points.iter().enumerate(){
        olc::draw(e.x as i32, e.y as i32, olc::RED);
        olc::draw_string(e.x as i32, (e.y + 4.0) as i32, &pos.to_string()[..] , olc::GREEN);
    }
    olc::draw(self.startPoint.x as i32, self.startPoint.y as i32, olc::BLUE);
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }
}

fn main() {
  let mut example = Points {
      points:Vec::new(),
      startPoint: point{x:0.0,y:0.0},
      endPoint: point{x:0.0,y:0.0},
      needFindNext:false,
      tmpFrame : 0.0,
      Maxspeed :20.0,
  };
  // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
  // and starts the main game loop.
  olc::start("BALL COLLISION!", &mut example, 160, 80, 10, 10).unwrap();
}