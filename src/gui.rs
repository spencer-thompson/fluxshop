use eframe::egui;

pub fn start() -> anyhow::Result<String> {
    let _ = gui();
    Ok("yea".to_string())
}

pub fn gui() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 1024.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fluxshop",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<FluxApp>::default())
        }),
    )
}

#[derive(Default)]
struct FluxApp {}

// This is where the app gets repainted and updated
impl eframe::App for FluxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut prompt = "Prompt here";
            ui.add(
                egui::Image::new(egui::include_image!("/home/sthom/pics/flux-gen.png"))
                    .corner_radius(5),
            );

            ui.separator();

            let response = ui.add(egui::TextEdit::multiline(&mut prompt));
            if response.changed() {
                println!("Response changed: {:?}", response);
            }

            // ui.add(egui::TextEdit::multiline(&mut prompt));

            ui.add(egui::Button::new("Generate"))
        });
    }
}
