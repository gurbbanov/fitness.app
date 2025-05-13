mod muscles;
mod app;
mod models;
mod tools;

use std::{fs, time::{Duration, Instant}};
use time::{OffsetDateTime};
use std::thread::sleep;
use eframe::egui::{self, pos2, vec2, Button, Color32, Layout, Pos2, RichText, Shape, Stroke, Vec2};
use serde::{Serialize, Deserialize};
use serde_json;
use std::path::Path;
// use usvg::{Tree, Options};
use image::GenericImageView;
use egui::{Ui};

fn load_png(ctx: &egui::Context, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes).unwrap();
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let egui_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    ctx.load_texture("background", egui_image, egui::TextureOptions::LINEAR)
}
//
//
fn draw_stretched_background_x(ui: &mut egui::Ui, texture: &egui::TextureHandle) {
    let texture_size = texture.size_vec2();
    let available_rect = ui.max_rect(); // вся доступная область без отступов

    // Высота картинки остаётся оригинальной
    let target_height = texture_size.y;

    // По X растягиваем картинку на всю ширину панели
    let target_width = available_rect.width();

    // Строим rect без отступов, сверху окна
    let rect = egui::Rect::from_min_size(
        available_rect.min,                    // верхний левый угол окна
        egui::vec2(target_width, target_height), // ширина окна, оригинальная высота картинки
    );

    // Настраиваем UV: по X растянуть, по Y оставить 0–1
    let uv = egui::Rect::from_min_max(
        egui::pos2(0.0, 0.0),
        egui::pos2(target_width / texture_size.x, 1.0),
    );

    ui.painter().add(egui::Shape::image(
        texture.id(),
        rect,
        uv,
        egui::Color32::WHITE,
    ));
}

// fn draw_background(ui: &mut egui::Ui, texture: &egui::TextureHandle) {
//     let rect = ui.max_rect(); // весь доступный прямоугольник
//     let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 0.1));
//     ui.painter().add(egui::epaint::Shape::image(
//         texture.id(),
//         rect,
//         uv,
//         egui::Color32::WHITE,
//     ));
// }



// impl Default for app {
//     fn default() -> Self {
//     }
// }


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_inner_size([500.0, 600.0])
            .with_min_inner_size([550.0, 900.0])
            .with_max_inner_size([600.0, 1000.0]),

        ..Default::default()
    };

    eframe::run_native(
        "fitness app",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::new()))),
    )
    // let mut sess = session::start();

    // sleep(Duration::from_secs(5));

    // sess.stop();

    // println!("{:?}", sess);


    // let mut leg_extens = exercise::new(String::from("Leg extension"), muscle_group::Legs, None);

    // println!("{:?}", leg_extens);

    // leg_extens.add_set();

    // let now = OffsetDateTime::now_local().unwrap();

    // println!("{:?}", leg_extens);
}   