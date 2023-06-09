pub mod ember {
    use canvas::digital_canvas;
    use colorgrad::{Color, CustomGradient};
    use rand::prelude::*;

    pub struct Fireplace {
        pub settings: Settings,
        embers: Vec<Ember>,
        pub heatmap: [[f32; 10]; 10],
    }

    pub struct Settings {
        w: f32,
        h: f32,
        sigma: f32,
        heat_decay: f32,
        pub g: colorgrad::Gradient,
    }
    struct Ember {
        heat: f32,
        x: f32,
        y: f32,
    }

    impl Fireplace {
        pub fn new() -> Fireplace {
            Fireplace {
                settings: Settings::new(),
                embers: (0..10).map(|_x| Ember::new()).collect(),
                heatmap: [[0.0; 10]; 10],
            }
        }
        // FIND_HEATMAP()
        // Uses the x,y position of embers in the Vec<embers to generate a map of "Heat" that later gets turned into RGB and displayed
        pub fn find_heatmap(&mut self) {
            self.heatmap = [[0.0; 10]; 10];
            let sigma = self.settings.sigma;
            for (j, row) in &mut self.heatmap.iter_mut().enumerate() {
                for (i, space) in row.iter_mut().enumerate() {
                    for ember in &mut self.embers.iter_mut() {
                        let distance_squared =
                            (i as f32 - ember.x).powf(2.0) + (j as f32 - ember.y).powf(2.0);
                        *space = *space
                            + ((ember.heat).powf(0.5) / 10.0)
                                * (-distance_squared / (sigma + (ember.heat).powf(0.5))).exp();
                    }
                    *space = (1.0 / -(0.7 * *space + 1.0).powf(2.0)) + 1.0
                }
            }
        }

        pub fn update_embers(&mut self) {
            let mut rng = rand::thread_rng();
            let decay = self.settings.heat_decay;
            for ember in self.embers.iter_mut() {
                let jitter: f32 = rng.gen();
                let jitter2: f32 = rng.gen();

                ember.y = ember.y - jitter;
                ember.x = ember.x + jitter2 - 0.5;
                ember.heat = ember.heat - decay * (ember.heat + 1.0).ln();
                if ember.y < 0.0 || ember.heat < 0.0 {
                    ember.y = 11.0;
                    ember.x = 4.0 + 4.0 * (jitter - 0.5);
                    ember.heat = 100.0 * jitter2;
                    //continue;
                }
            }
        }
    }

    impl Settings {
        pub fn new() -> Settings {
            Settings {
                w: 10.0,
                h: 10.0,
                sigma: 0.0,
                heat_decay: 1.5,
                g: colorgrad::CustomGradient::new()
                    .colors(&[
                        Color::from_rgba8(0, 0, 0, 255),
                        Color::from_rgba8(161, 10, 0, 255),
                        Color::from_rgba8(218, 31, 5, 255),
                        Color::from_rgba8(243, 60, 4, 255),
                        Color::from_rgba8(254, 101, 13, 255),
                        Color::from_rgba8(255, 183, 31, 255),
                        Color::from_rgba8(255, 227, 93, 255),
                    ])
                    //.domain(&[0.0, 0.5, 1.0])
                    .build()
                    .unwrap(),
            }
        }
    }

    impl Ember {
        pub fn new() -> Ember {
            let mut rng = rand::thread_rng();
            let jitter: f32 = rng.gen();
            let jitter2: f32 = rng.gen();
            Ember {
                heat: jitter * 100.0,
                x: jitter2 * 10.0,
                y: 10.0,
            }
        }
    }
}
