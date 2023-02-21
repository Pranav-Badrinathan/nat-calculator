use eframe::{run_native, egui};

#[derive(Default)]
struct Calc {
	input: String,
}

impl Calc {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Calc {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
			ui.text_edit_multiline(&mut self.input);
       });
   }
}

fn main () {
	let native_options = eframe::NativeOptions::default();
	
	run_native("NatCalc", native_options, Box::new(|cc| Box::new(Calc::new(cc))));

	println!("Hello World!");
	nat_calc::hello();
}
