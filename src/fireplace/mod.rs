use colorgrad::Color;
use rand::prelude::*;
//use rand_distr::{Distribution, Normal};
use super::ember::*;

pub struct Fireplace {
    state: FireplaceState,
    pub settings: FireplaceSettings,
    embers: Vec<Ember>,
    pub heatmap: [[f32; 10]; 10],
}

pub struct FireplaceSettings {
    _w: f32,
    _h: f32,
    pub ember_settings: EmberSettings,
    pub g: colorgrad::Gradient,
    pub max_embers: usize,
}

pub enum FireplaceState {
    Starting,
    Running,
    Off,
}

impl FireplaceSettings {
    pub fn new() -> FireplaceSettings {
        FireplaceSettings {
            _w: 10.0,
            _h: 10.0,
            ember_settings: EmberSettings {
                respawn_enabled: true,
                sigma: 0.0,
                heat_decay: 1.4,
                max_heat: 100.0,
            },
            max_embers: 10,
            g: colorgrad::CustomGradient::new()
                .colors(&[
                    Color::from_rgba8(0, 0, 0, 255),
                    Color::from_rgba8(161, 10, 0, 255),
                    Color::from_rgba8(218, 31, 5, 255),
                    Color::from_rgba8(243, 60, 4, 255),
                    Color::from_rgba8(254, 131, 13, 255),
                    Color::from_rgba8(255, 183, 31, 255),
                    Color::from_rgba8(255, 227, 93, 255),
                ])
                //.domain(&[0.0, 0.5, 1.0])
                .build()
                .unwrap(),
        }
    }
}

impl Fireplace {
    pub fn new() -> Fireplace {
        Fireplace {
            state: FireplaceState::Starting,
            settings: FireplaceSettings::new(),
            embers: (0..20).map(|_x| Ember::new()).collect(),
            heatmap: [[0.0; 10]; 10],
        }
    }
    pub fn off(&mut self) {
        //TODO Wind down????
        //self.state = FireplaceState::Off;
        self.settings.ember_settings.respawn_enabled = false;
        // for ember in self.embers.iter_mut() {
        //     ember.deactivate()
        // }
    }
    pub fn start(&mut self) {
        self.state = FireplaceState::Starting;
    }

    pub fn step(&mut self) {
        match self.state {
            FireplaceState::Off => {
                //TODO wind down??
                //self.update_embers();
                //self.find_heatmap();
            }
            FireplaceState::Running => {
                self.update_embers();
                self.find_heatmap();
            }
            FireplaceState::Starting => {
                self.settings.ember_settings.respawn_enabled = true;
                let mut rng = rand::thread_rng();
                let mut cum_delay = 0;
                for ember in &mut self.embers.iter_mut() {
                    let jitter: f32 = rng.gen();
                    cum_delay = cum_delay + (jitter * 30.0).round() as u16;
                    ember.prime(self.settings.ember_settings);
                    ember.delay(cum_delay);
                }
                self.state = FireplaceState::Running;
            }
        }
    }
    // FIND_HEATMAP()
    // Uses the x,y position of embers in the Vec<embers to generate a map of "Heat" that later gets turned into RGB and displayed
    fn find_heatmap(&mut self) {
        self.heatmap = [[0.0; 10]; 10];
        let sigma = self.settings.ember_settings.sigma;
        for (j, row) in &mut self.heatmap.iter_mut().enumerate() {
            for (i, space) in row.iter_mut().enumerate() {
                for ember in &mut self.embers.iter_mut() {
                    match ember.status {
                        EmberStatusKind::Active => {
                            //Find distance of the ember to each space
                            let distance_squared =
                                (i as f32 - ember.x).powf(2.0) + (j as f32 - ember.y).powf(2.0);
                                // -------------------------------------------
                                //This section is a disaster. It works visually, but there has to be a cleaner function that works just as well.
                                // -------------------------------------------
                            //Equation to turn distance into the contributed "temperature" of the space
                            *space = *space
                                //Scaling coefficient From 0 to 1. (ember.heat has a nominal range of 100-0)
                                + ((ember.heat).powf(0.5) / 10.0)
                                //exponential function exp   ( -d^2 )
                                //                          -----------
                                //                          s + sqrt(h)
                                    * (-distance_squared / (sigma + (ember.heat).powf(0.5)))
                                        .exp();
                        }
                        _ => {}
                    }
                }
                // Scales temperature to between 0 and 1 using a rational function
                *space = (1.0 / -(0.7 * *space + 1.0).powf(2.0)) + 1.0
            }
        }
    }

    pub fn update_embers(&mut self) {
        //let mut rng = rand::thread_rng();
        //let decay = self.settings.ember_settings.heat_decay;
        for ember in self.embers.iter_mut() {
            ember.step(self.settings.ember_settings)
            // match ember.status {
            //     //If the ember is inactive, we check if it is due for activation. Otherwise, decrement timer
            //     EmberStatusKind::Primed(0) => {
            //         ember.activate();
            //     }
            //     EmberStatusKind::Primed(counter) => {
            //         ember.status = EmberStatusKind::Primed(counter - 1);
            //     }
            //     EmberStatusKind::Active => {
            //         let jitter: f32 = rng.gen();
            //         let jitter2: f32 = rng.gen();

            //         ember.y = ember.y - jitter;
            //         ember.x = ember.x + jitter2 - 0.5;
            //         if ember.y < 11.0 {
            //             ember.heat = ember.heat - decay * (ember.heat + 1.0).ln();
            //         }

            //         if ember.y < 0.0 || ember.heat < 0.0 {
            //             ember.prime(self.settings.ember_settings)
            //             //continue;
            //         }
            //     }
            // }
        }
    }
    // fn off(&mut self) {
    //     for ember in self.embers.iter_mut() {
    //         ember.deactivate()
    //     }
    // }
}


