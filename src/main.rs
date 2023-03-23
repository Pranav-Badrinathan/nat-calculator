use eframe::{run_native, egui, };
use egui::{FontId, TextStyle::*, FontFamily};

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

		// Set a name for the primary font key.
		let primary_font: String = String::from("primaryfont");

		let mut fonts = egui::FontDefinitions::default();

		fonts.font_data.insert(
			primary_font.to_owned(),
			egui::FontData::from_static(include_bytes!(
				"../assets/fonts/CascadiaCode-Regular.otf"
			)),
		);

		fonts
			.families
			.entry(egui::FontFamily::Proportional)
			.or_default()
			.insert(0, primary_font.to_owned());

		// Put font as last fallback for monospace:
		fonts
			.families
			.entry(egui::FontFamily::Monospace)
			.or_default()
			.push(primary_font.to_owned());

		cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }
}

impl eframe::App for Calc {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			// Create a new style variable and override things that we want to change (font size).
			let mut style = (*ctx.style()).clone();	
			style.text_styles = [(Monospace, FontId::new(15.0, FontFamily::Proportional)),
								 (Body, FontId::new(15.0, FontFamily::Monospace))].into();
			ctx.set_style(style);

			ui.add_sized(ui.available_size(), 
						egui::TextEdit::code_editor(egui::TextEdit::multiline(&mut self.input)));
       });
   }
}

fn main () {
	let native_options = eframe::NativeOptions::default();
	
	if let Err(_) =  run_native("NatCalc", native_options, Box::new(|cc| Box::new(Calc::new(cc)))) {
		eprintln!("Egui error launching the application: failed to set up a graphics context.");
	}

	println!("Hello World!");
	nat_calc::hello();
}
