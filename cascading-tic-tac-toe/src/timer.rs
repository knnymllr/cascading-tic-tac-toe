use std::time::Duration;
use bevy::prelude::*;



pub const TIME: u16 = 10*60;
pub const TEXT_COLOR: Color = Color::WHITE;

#[derive(Component)]

pub struct Counter{
    pub seconds: Timer,
    pub unit: Timer,
}


impl Counter{
    pub fn new() -> Counter {
        Self {
            seconds: Timer::from_seconds(TIME.into(), TimerMode::Repeating),
            unit: Timer::from_seconds(1.,TimerMode::Repeating)
        }
    }
    pub fn pause(&mut self){
        self.seconds.pause();
        self.unit.pause();
    }
    pub fn paused(&self) -> bool{
        self.seconds.paused() && self.unit.paused()
    }
    pub fn tick(&mut self, duration: Duration) {
        self.seconds.tick(duration);
        self.unit.tick(duration);
    }

    
    pub fn unit_just_finished(&self) -> bool{
        self.unit.just_finished()
       
    }
    pub fn duration(&self) -> Duration{
        self.seconds.duration()
    }
    pub fn elapsed_secs_round(&self) -> f32{
        self.seconds.elapsed_secs().round()
    }
}

pub fn time(duration: Duration) -> String{
    let mut minute = 0;
    let mut hour = 0;
    let mut seconds = duration.as_secs();
    while seconds >= 60{
        seconds -= 60;
        minute += 1;
    }
    while minute >= 60{
        minute -= 60;
        hour += 1;
    }
    let mut seconds = seconds.to_string();
    if seconds.len() < 2{
        seconds = format!("0{}", seconds);
    }
    let mut minute = minute.to_string();
    if minute.len() < 2 {
        minute = format!("0{}", minute);
    }
    let mut hour = hour.to_string();
    if hour.len() < 2 {
        hour = format!("0{}", hour);
    }
    format!("{}:{}:{}", hour, minute, seconds)
}