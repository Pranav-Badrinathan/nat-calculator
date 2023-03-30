use eframe::{run_native, egui, epaint::Color32, };
use egui::{FontId, TextStyle::*, FontFamily, Margin};

#[derive(Default)]
struct Calc {
	input: String,
	output: String,
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
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		let properties = egui::Frame::none()
			.inner_margin(Margin::same(0.0))
			.fill(Color32::from_rgb(20, 20, 20));
		
		egui::CentralPanel::default()
			.frame(properties)
			.show(ctx, |ui| {
				// Create a new style variable and override things that we want to change (font size).
				let mut style = (*ctx.style()).clone();	
				style.text_styles = [(Monospace, FontId::new(14.0, FontFamily::Proportional)),
									 (Body, FontId::new(14.0, FontFamily::Monospace))].into();
				ctx.set_style(style);
				frame.set_window_title("Calculator?");

				// Wrap the text area in a scroll area.
				egui::ScrollArea::vertical().show(ui, |ui| {
					egui::Frame::none()
							.inner_margin(Margin::symmetric(30.0, 10.0))
							.fill(Color32::TRANSPARENT)
						.show(ui, |ui| {
							
							ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
								
								// The "Answers" panel. the "&*" in front of the output re-references it. String -> &str 
								ui.add_sized([100.0, ui.available_size().y],
										egui::TextEdit::multiline(&mut &*self.output)
												.frame(false));

								// The main writing space.
								ui.add_sized(ui.available_size(), 
										egui::TextEdit::multiline(&mut self.input)
												.code_editor()
												.frame(false));
							});
					});
				});
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
