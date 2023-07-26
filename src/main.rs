use fltk::{
    app,
    button::Button,
    dialog::{self, FileChooser, FileChooserType, FileDialog, FileDialogType},
    prelude::*,
    window::Window,
};
use image::{open, GenericImageView};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Debug, Deserialize)]
struct Config {
    padding: usize,
    page_format: PageFormat,
    rows: usize,
    columns: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            padding: 20,
            page_format: PageFormat::IN4X6,
            rows: 2,
            columns: 4,
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
enum PageFormat {
    A4,
    IN4X6,
}

fn parse_config(config_path: PathBuf) -> Config {
    match std::fs::read_to_string(config_path.clone()) {
        Ok(config) => match toml::from_str(&config) {
            Ok(config) => config,
            Err(e) => {
                warn!("{}", e);
                info!("Creating a new config file at ");
                let config = Config::default();
                match std::fs::write(config_path.clone(), toml::to_string_pretty(&config_path.clone()).unwrap()) {
                    Err(err) => {
                        error!("{}", err);
                    }
                    Ok(()) => {}
                }
                config
            }
        },
        Err(e) => {
            warn!("{}", e);
            info!("Creating a new config file at ");
            let config = Config::default();
            std::fs::write(config_path, toml::to_string_pretty(&config).unwrap()).unwrap();
            config
        }
    }
}

fn main() {
    femme::start();
    let config = parse_config("photoGrid.toml".into());
    let app = app::App::default();
    let mut windo = Window::new(100, 100, 400, 300, "Photo Grid");
    let mut print_btn = Button::new(160, 80, 80, 40, "Print");
    let mut image_btn = Button::new(160, 210, 80, 40, "Select File");
    windo.end();
    windo.show();
    print_btn.set_callback(move |e| e.set_label("You Clicked Me!!"));
    image_btn.set_callback(move |_| {
        let mut fd = FileDialog::new(FileDialogType::BrowseFile);
        fd.show();
        let filename = fd.filename();
        println!("{:?}", filename);
        match handle_img(filename, config.clone()) {
            None => error!("No image found"),
            Some(img) => img.save("output.png").unwrap()
        };
    });
    app.run().unwrap();
}

fn handle_img(file_name: PathBuf, config: Config) -> Option<image::RgbaImage> {
    let img = match open(file_name) {
        Ok(img) => img,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let (img_width, img_height) = (img.width(), img.height());
    let (total_width, total_height) = (
        (img_width as usize + config.padding) * config.columns,
        (img_height as usize + config.padding) * config.rows,
    );
    let mut big_img = image::RgbaImage::new(total_width as u32, total_height as u32);
    println!("{:?}", config);
    for row in 0..(config.rows as u32) {
        for column in 0..(config.columns as u32) {
            for (x, y, pixel) in img.pixels() {
                big_img.put_pixel(
                    x + column * (img_width + config.padding as u32),
                    y + row * (img_height + config.padding as u32),
                    pixel,
                )
            }
        }
    }
    Some(big_img)
}
