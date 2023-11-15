use rand::prelude::*;
use rand_distr::{Distribution, Normal};

pub struct Ember {
    pub status: EmberStatusKind,
    pub heat: f32,
    pub x: f32,
    pub y: f32,
}

pub enum EmberStatusKind {
    Active,
    Inactive,
    Primed(u16),
}

#[derive(Copy, Clone)]
pub struct EmberSettings {
    pub respawn_enabled: bool,
    pub sigma: f32,
    pub heat_decay: f32,
    pub max_heat: f32,
}

impl Ember {
    pub fn new() -> Ember {
        let mut rng = rand::thread_rng();
        let jitter: f32 = rng.gen();
        let jitter2: f32 = rng.gen();
        let jitter3: f32 = rng.gen();
        Ember {
            heat: jitter * 100.0,
            x: jitter2 * 10.0,
            y: 10.0,
            status: EmberStatusKind::Primed((jitter3 * 10.0).round() as u16),
        }
    }
    pub fn step(&mut self, settings: EmberSettings) {
        let mut rng = rand::thread_rng();
        match self.status {
            //If the ember is inactive, we check if it is due for activation. Otherwise, decrement timer
            EmberStatusKind::Primed(0) => {
                self.activate();
            }
            EmberStatusKind::Primed(counter) => {
                self.status = EmberStatusKind::Primed(counter - 1);
            }
            EmberStatusKind::Active => {
                let jitter: f32 = rng.gen();
                let jitter2: f32 = rng.gen();

                self.y = self.y - jitter;
                self.x = self.x + jitter2 - 0.5;
                if self.y < 11.0 {
                    self.heat = self.heat - settings.heat_decay * (self.heat + 1.0).ln();
                }

                if self.y < 0.0 || self.heat < 0.0 {
                    if settings.respawn_enabled {
                        self.prime(settings)
                    } else {
                        self.deactivate();
                    }
                    //continue;
                }
            }
            EmberStatusKind::Inactive => {}
        }
    }
    pub fn delay(&mut self, delay_ticks: u16) {
        match self.status {
            EmberStatusKind::Primed(counter) => {
                self.status = EmberStatusKind::Primed(counter + delay_ticks);
            }
            _ => {}
        }
    }
    pub fn activate(&mut self) {
        self.status = EmberStatusKind::Active;
    }
    pub fn deactivate(&mut self) {
        self.status = EmberStatusKind::Inactive;
    }
    pub fn prime(&mut self, settings: EmberSettings) {
        let mut rng = rand::thread_rng();
        let jitter: f32 = rng.gen();
        let _jitter2: f32 = rng.gen();
        let _jitter3: f32 = rng.gen();

        let normal2 = Normal::new(0.0, 1.0).unwrap();
        let jitter2 = normal2.sample(&mut rand::thread_rng());

        let normal3 = Normal::new(0.4, 0.2).unwrap();
        let jitter3 = normal3.sample(&mut rand::thread_rng());

        self.status = EmberStatusKind::Primed((jitter * 20.0).round() as u16);
        self.y = 15.0;
        //self.x = 4.0 + 4.0 * (jitter2 - 0.5);
        self.x = 4.0 + jitter2;
        self.heat = settings.max_heat * (0.3 + jitter3);
    }
}