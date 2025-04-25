use eframe::egui;
use image::{ImageError, ImageFormat, ImageReader};
use rfd::FileDialog;

struct ImageConverterApp {
    image_path: String,
    new_name: String,
    new_format: String,
    status: String,
}

impl Default for ImageConverterApp {
    fn default() -> Self {
        Self {
            image_path: String::new(),
            new_name: String::new(),
            new_format: "png".to_string(), // Varsayılan format
            status: String::from("Welcome to Image Format Changer"),
        }
    }
}

impl eframe::App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Format Changer");
            ui.add_space(10.0);

            // Image path input and file picker
            ui.horizontal(|ui| {
                ui.label("Image path: ");
                ui.text_edit_singleline(&mut self.image_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter(
                            "Image",
                            &[
                                "png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp", "ico", "tga",
                                "dds", "hdr",
                            ],
                        )
                        .pick_file()
                    {
                        self.image_path = path.to_string_lossy().into_owned();
                    }
                }
            });
            ui.add_space(5.0);

            // New image name input
            ui.label("New image name (without extension):");
            ui.text_edit_singleline(&mut self.new_name);
            ui.add_space(5.0);

            // Format selection dropdown
            ui.label("Select new image format:");
            let formats = vec![
                "jpeg", "png", "bmp", "gif", "tiff", "webp", "ico", "pbm", "pgm", "ppm", "tga",
                "dds", "hdr",
            ];
            egui::ComboBox::from_label("")
                .selected_text(self.new_format.clone())
                .show_ui(ui, |ui| {
                    for format in formats {
                        ui.selectable_value(&mut self.new_format, format.to_string(), format);
                    }
                });
            ui.add_space(10.0);

            // Convert button
            if ui.button("Convert Image").clicked() {
                match self.convert_image() {
                    Ok(_) => self.status = format!(
                        "Image successfully saved as: {}.{}",
                        self.new_name,
                        self.new_format.to_lowercase()
                    ),
                    Err(e) => self.status = format!("Error: {}", e),
                }
            }

            // Status message
            ui.add_space(10.0);
            ui.label(&self.status);
        });
    }
}

impl ImageConverterApp {
    fn convert_image(&self) -> Result<(), ImageError> {
        if self.image_path.is_empty() || self.new_name.is_empty() {
            return Err(ImageError::Parameter(
                image::error::ParameterError::from_kind(
                    image::error::ParameterErrorKind::Generic(
                        "Image path and name cannot be empty".to_string(),
                    ),
                ),
            ));
        }

        let img = ImageReader::open(self.image_path.trim())?
            .with_guessed_format()?
            .decode()?;

        let format_type = match self.new_format.to_lowercase().as_str() {
            "jpeg" => ImageFormat::Jpeg,
            "jpg" => {
                println!("jpg image replaced with jpeg!!");
                ImageFormat::Jpeg
            }
            "png" => ImageFormat::Png,
            "bmp" => ImageFormat::Bmp,
            "gif" => ImageFormat::Gif,
            "tiff" => ImageFormat::Tiff,
            "webp" => ImageFormat::WebP,
            "ico" => ImageFormat::Ico,
            "pbm" => ImageFormat::Pnm,
            "pgm" => ImageFormat::Pnm,
            "ppm" => ImageFormat::Pnm,
            "tga" => ImageFormat::Tga,
            "dds" => ImageFormat::Dds,
            "hdr" => ImageFormat::Hdr,
            _ => {
                return Err(ImageError::Unsupported(
                    image::error::UnsupportedError::from_format_and_kind(
                        image::error::ImageFormatHint::Unknown,
                        image::error::UnsupportedErrorKind::Format(
                            image::error::ImageFormatHint::Unknown,
                        ),
                    ),
                ));
            }
        };

        let file_name = format!("{}.{}", self.new_name.trim(), self.new_format.to_lowercase());
        img.save_with_format(&file_name, format_type)?;
        Ok(())
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Image Format Changer",
        options,
        Box::new(|_cc| Ok(Box::new(ImageConverterApp::default()))),
    )
}