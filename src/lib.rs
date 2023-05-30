pub mod ember {

    pub struct Fireplace {
        settings: Settings,
        embers: Vec<Ember>,
        heatmap: [[f32; 10]; 10],
    }

    struct Heatmap {}

    struct Settings {
        w: f32,
        h: f32,
        sigma: f32,
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
        pub fn find_heatmap(&mut self) {
            for (i, row) in &mut self.heatmap.iter_mut().enumerate() {
                for (j, space) in row.iter_mut().enumerate() {
                    for ember in &mut self.embers.iter_mut() {
                        let distance_squared =
                            ((i as f32 - ember.x).powf(2.0) + (j as f32 - ember.y).powf(2.0));
                        *space = (-distance_squared / 2.0).exp()
                    }
                }
            }
        }
    }

    impl Settings {
        pub fn new() -> Settings {
            Settings {
                w: 10.0,
                h: 10.0,
                sigma: 1.0,
            }
        }
    }

    impl Ember {
        pub fn new() -> Ember {
            Ember {
                heat: 10.0,
                x: 0.0,
                y: 0.0,
            }
        }
    }
}
