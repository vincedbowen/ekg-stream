use std::time::{Duration, Instant};

use eframe::{App, Frame, NativeOptions, egui};
use egui::Color32;
use egui_extras::{Size, StripBuilder};
use egui_plot::{Line, Plot, PlotPoints};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Electrocardiogram (EKG)",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    start_time: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let heartRate: u16 = 65;
        // App Header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.heading(egui::RichText::new("Electrocardiogram (EKG)").strong());
            });
            ui.add_space(6.0);
        });

        egui::TopBottomPanel::top("info_boxes")
            .resizable(false)
            .exact_height(100.0)
            .show(ctx, |ui| {
                StripBuilder::new(ui)
                    .sizes(Size::relative(1.0 / 3.0), 3)
                    .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(80.0, 80.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new("Session Information").strong(),
                                        );
                                        ui.separator();
                                        ui.small("Stuff about the session");
                                    });
                                });
                            });
                            strip.cell(|ui| {
                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(80.0, 80.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new("Device Information").strong(),
                                        );
                                        ui.separator();
                                        ui.small("Stuff about the connected device (producer)");
                                    });
                                });
                            });
                            strip.cell(|ui| {
                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(80.0, 80.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new("Extra").strong(),
                                        );
                                        ui.separator();
                                        ui.small("Probably gonna want another box of sorts");
                                    });
                                });
                            });
                    });
            });

        // raw ekg plot
        egui::CentralPanel::default().show(ctx, |ui| {
            let points = PlotPoints::from_iter((0..200).map(|i| {
                let x = i as f64 * 0.05;
                [x, (x).sin() + 0.2 * (2.0 * x).cos()]
            }));

            Plot::new("raw ekg")
                .allow_drag(false)
                .allow_zoom(true)
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new("", points));
                });
        });

        egui::TopBottomPanel::bottom("bottom panel")
            .resizable(false)
            .default_height(160.0)
            .show(ctx, |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.65))
                    .size(Size::relative(0.35))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                ui.set_min_size(egui::vec2(100.0, 140.0));
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Random Info").strong());
                                    ui.separator();
                                    ui.label("Surely gonna need to add some stuff later (logs, etc.)");
                                });
                            });
                        });
                        strip.cell(|ui| {
                            let bpm = heartRate as f32;
                            let period_secs = 60.0_f32 / bpm.max(1.0);
                            let elapsed = self.start_time.elapsed().as_secs_f32();
                            let phase = elapsed % period_secs;
                            let duty_fraction = 0.15_f32; 
                            let is_on = phase < (period_secs * duty_fraction);
                            let light_color = if is_on {
                                Color32::from_rgb(0xff, 0x44, 0x44)
                            } else {
                                Color32::from_gray(100)
                            };
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                ui.set_min_size(egui::vec2(100.0, 140.0));
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Heartrate").strong());
                                    ui.separator();
                                    ui.horizontal(|ui| {
                                        ui.label(heartRate.to_string());
                                        ui.label(egui::RichText::new("•").color(light_color));
                                    });
                                });
                            });
                            // keep animating
                            ctx.request_repaint_after(Duration::from_millis(50));
                        });
                    });
            });
    }
}
