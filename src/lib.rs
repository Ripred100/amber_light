pub mod ember {
    use canvas::digital_canvas;
    use colorgrad::{Color, CustomGradient};
    use rand::prelude::*;

    pub struct Fireplace {
        pub settings: Settings,
        embers: Vec<Ember>,
        pub heatmap: [[f32; 10]; 10],
    }

    struct Heatmap {}

    pub struct Settings {
        w: f32,
        h: f32,
        sigma: f32,
        alpha: f32,
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
                embers: vec![Ember::new()],
                heatmap: [[0.0; 10]; 10],
            }
        }
        // FIND_HEATMAP()
        // Uses the position of embers in the Vec<embers to generate a map of "Heat" that later gets turned into RGB and displayed
        pub fn find_heatmap(&mut self) {
            let sigma = self.settings.sigma;
            for (j, row) in &mut self.heatmap.iter_mut().enumerate() {
                for (i, space) in row.iter_mut().enumerate() {
                    for ember in &mut self.embers.iter_mut() {
                        let distance_squared =
                            ((i as f32 - ember.x).powf(2.0) + (j as f32 - ember.y).powf(2.0));
                        *space = (-distance_squared / (sigma + (ember.heat).powf(0.5))).exp();
                    }
                }
            }
        }

        pub fn update_embers(&mut self) {
            let mut rng = rand::thread_rng();
            for ember in self.embers.iter_mut() {
                if ember.y < 0.0 || ember.heat < 0.0 {
                    *ember = Ember::new();
                    continue;
                }
                let jitter: f32 = rng.gen();
                ember.y = ember.y - jitter;
                ember.x = ember.x + jitter - 0.5;
                ember.heat = ember.heat - 2.0 * (ember.heat + 1.0).ln();
            }
        }
    }

    impl Settings {
        pub fn new() -> Settings {
            Settings {
                w: 10.0,
                h: 10.0,
                sigma: 1.0,
                alpha: 0.0,
                g: colorgrad::CustomGradient::new()
                    .colors(&[
                        Color::from_rgba8(0, 0, 0, 255),
                        Color::from_rgba8(161, 1, 0, 255),
                        Color::from_rgba8(218, 31, 5, 255),
                        Color::from_rgba8(243, 60, 4, 255),
                        Color::from_rgba8(254, 101, 13, 255),
                        Color::from_rgba8(255, 193, 31, 255),
                        Color::from_rgba8(255, 247, 93, 255),
                    ])
                    //.domain(&[0.0, 0.5, 1.0])
                    .build()
                    .unwrap(),
            }
        }
    }

    impl Ember {
        pub fn new() -> Ember {
            Ember {
                heat: 100.0,
                x: 5.0,
                y: 10.0,
            }
        }
    }
}
