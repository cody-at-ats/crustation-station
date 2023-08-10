use core::panic;

pub struct DrumSequencer {
    segments: Vec<DrumSegment>,
    active_segment: usize,
}

pub struct DrumSegment {
    beat: u32,
    kick: bool,
    snare: bool,
    hi_hat_closed: bool,
    hi_hat_open: bool,
    floor_tom: bool,
    ride: bool,
}

impl Default for DrumSequencer {
    fn default() -> Self {
        Self {
            segments: vec![],
            active_segment: 0,
        }
    }
}

impl DrumSequencer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_slice(&mut self, iteration: usize) -> &DrumSegment {
        if iteration < self.segments.len() {
            self.active_segment = iteration;
            &self.segments[iteration]
        } else {
            panic!("NO")
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let Self {
            segments,
            active_segment,
        } = self;

        let active_index: usize = active_segment.clone();

        const beats: u32 = 4;
        const bars: u32 = 4;

        if segments.len() == 0 {
            for i in 0..beats * bars {
                segments.push(DrumSegment {
                    beat: (i),
                    kick: (false),
                    snare: (false),
                    hi_hat_closed: (false),
                    hi_hat_open: (false),
                    floor_tom: (false),
                    ride: (false),
                })
            }
        }

        ui.vertical(|ui| {
            const KICK_LABEL: &str = "Kick----------";
            const SNARE_LABEL: &str = "Snare---------";
            const HI_HAT_CLOSED_LABEL: &str = "Hi Hat Closed-";
            const HI_HAT_OPEN_LABEL: &str = "Hi Hat Open---";
            const FLOOR_TOM_LABEL: &str = "Floor Tom-----";
            const RIDE_LABEL: &str = "Ride----------";

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(KICK_LABEL).text_style(egui::TextStyle::Monospace));

                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].kick, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(SNARE_LABEL).text_style(egui::TextStyle::Monospace));
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].snare, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(HI_HAT_CLOSED_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].hi_hat_closed, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(HI_HAT_OPEN_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].hi_hat_open, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(FLOOR_TOM_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].floor_tom, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    if active_index > 0 {
                        *active_segment -= 1;
                    }
                }
                if ui.button(">").clicked() {
                    if active_index < segments.len() - 1 {
                        *active_segment += 1;
                    }
                };
            });
        });
    }
}
