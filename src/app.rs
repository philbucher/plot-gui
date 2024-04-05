use std::hash::{DefaultHasher, Hash, Hasher};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    select: Selector,

    #[serde(skip)]
    plot_states: std::vec::Vec<PlotState>, // should be map if this should be the unique Id of the window. Also, should be ordered. Or, drop the others
    #[serde(skip)]
    picked_path: Option<String>,

    link_cursor: bool,
    link_axis: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            select: Selector::Memory,
            plot_states: Vec::new(),
            picked_path: None,
            link_cursor: false,
            link_axis: false,
        }
    }
}

struct PlotState {
    label: String,
    select: Selector,
    open: bool,
}
impl Default for PlotState {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            select: Selector::Memory,
            open: true,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Selector {
    Memory,
    DiskSpace,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            // ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true))
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
            ui.label(format!("Number of PlotStates: {}", self.plot_states.len()));

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            if ui.button("Open folder…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked folder:");
                    ui.monospace(picked_path);
                });
            }

            if ui
                .button("New Window")
                .on_hover_text("this is the tooltip")
                .clicked()
                && !self.plot_states.iter().any(|ps| ps.label == self.label)
            {
                self.plot_states.push(PlotState {
                    label: self.label.clone(),
                    select: self.select,
                    open: true,
                });
            }

            // delete the closed windows
            self.plot_states.retain(|ps| ps.open);

            let person2 = Person {
                id: 5,
                name: "Bob".to_string(),
                phone: 555_666_7777,
            };

            fn calculate_hash<T: Hash>(t: &T) -> u64 {
                let mut s = DefaultHasher::new();
                t.hash(&mut s);
                s.finish()
            }

            // this is only needed to get a random hash value
            // needs to be refactored/improved
            let plot_link_id = egui::Id::new(calculate_hash(&person2));

            ui.checkbox(&mut self.link_cursor, "Link cursor");
            ui.checkbox(&mut self.link_axis, "Link axis");

            for ps in self.plot_states.iter_mut() {
                egui::Window::new(ps.label.clone())
                    .title_bar(true)
                    .open(&mut ps.open)
                    .show(ctx, |win_ui| {
                        win_ui.label("Hello World!");
                        win_ui.horizontal(|win_ui_hor| {
                            win_ui_hor.radio_value(&mut ps.select, Selector::Memory, "Memory");
                            win_ui_hor.radio_value(
                                &mut ps.select,
                                Selector::DiskSpace,
                                "Disk space",
                            );
                        });
                        if ps.select == Selector::Memory {
                            let sin: egui_plot::PlotPoints = (0..1000)
                                .map(|i| {
                                    let x = i as f64 * 0.01 + self.value as f64;
                                    [x, x.sin()]
                                })
                                .collect();
                            let line = egui_plot::Line::new(sin);
                            egui_plot::Plot::new("my_plot")
                                .view_aspect(1.0)
                                .width(640.0)
                                .height(240.0)
                                .link_cursor(plot_link_id, self.link_cursor, self.link_cursor)
                                .link_axis(plot_link_id, self.link_axis, self.link_axis)
                                .show(win_ui, |plot_ui| plot_ui.line(line));
                        } else {
                            let cos: egui_plot::PlotPoints = (0..1000)
                                .map(|i| {
                                    let x = i as f64 * 0.01 - self.value as f64;
                                    [x, x.cos()]
                                })
                                .collect();
                            let line = egui_plot::Line::new(cos);
                            egui_plot::Plot::new("my_plot")
                                .view_aspect(1.0)
                                .width(640.0)
                                .height(240.0)
                                .link_cursor(plot_link_id, self.link_cursor, self.link_cursor)
                                .link_axis(plot_link_id, self.link_axis, self.link_axis)
                                .show(win_ui, |plot_ui| {
                                    plot_ui.line(line);
                                    plot_ui.vline(egui_plot::VLine::new(1.234));
                                });
                        }
                    });
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
