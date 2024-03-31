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
    plot_states: std::vec::Vec<PlotState>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            select: Selector::Memory,
            plot_states: Vec::new(),
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
enum Selector { Memory, DiskSpace}

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

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }


            ui.radio_value(&mut self.select, Selector::Memory, "Memory");
            ui.radio_value(&mut self.select, Selector::DiskSpace, "Disk space");

            if ui.button("New Window").clicked() {
                self.plot_states.push(PlotState{label: self.label.clone(), select: self.select, open:true});
            }

            ui.vertical(|ui| {
                for ps in self.plot_states.iter_mut() {
                    egui::Window::new(ps.label.clone()).title_bar(true).open(&mut ps.open).show(ctx, |win_ui| {
                        win_ui.label("Hello World!");
                        win_ui.radio_value(&mut ps.select, Selector::Memory, "Memory");
                        win_ui.radio_value(&mut ps.select, Selector::DiskSpace, "Disk space");
                    if ps.select == Selector::Memory {
                        let sin: egui_plot::PlotPoints = (0..1000).map(|i| {
                            let x = i as f64 * 0.01+self.value as f64;
                            [x, x.sin()]
                        }).collect();
                        let line = egui_plot::Line::new(sin);
                        egui_plot::Plot::new("my_plot").view_aspect(1.0).width(640.0).height(240.0).show(win_ui, |plot_ui| plot_ui.line(line));
                    } else {
                        let cos: egui_plot::PlotPoints = (0..1000).map(|i| {
                            let x = i as f64 * 0.01-self.value as f64;
                            [x, x.cos()]
                        }).collect();
                        let line = egui_plot::Line::new(cos);
                        egui_plot::Plot::new("my_plot").view_aspect(1.0).width(640.0).height(240.0).show(win_ui, |plot_ui| plot_ui.line(line));
                    }
                });
                }
            });


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
