use std::fs;
use std::sync::Arc;
use time::{OffsetDateTime, Duration};
use egui::{Context, Ui};
use serde::{Serialize, Deserialize};
use serde_json;
use eframe::egui::{self, pos2, vec2, Button, Color32, Layout, Pos2, RichText, Stroke, UiBuilder, Vec2, epaint::{Shadow, Shape, CornerRadius}, Rect, Label, FontDefinitions, FontFamily, TextureHandle, Rounding};
use eframe::egui::color_picker::color_picker_color32;
use eframe::egui::scroll_area::State;
use crate::models::{UserInformation, AccountData, AllWorkoutData, MacroData, States};
use crate::muscles::{workout_tracker_widget_front, workout_tracket_widget_behind};
use crate::tools::weekday_iso;

pub struct AppRuntime {
    pub app: App,
    pub image_texture: Option<TextureHandle>,
    pub calory_add_modal: bool,
}

impl AppRuntime {
    pub fn new(ctx: &egui::Context) -> Self {
        let image = load_png(ctx, include_bytes!("blue.png"));
        let app = App::new(); // или другой способ загрузки состояния

        // let image = load_image_from_path("decor.png");
        // let texture = ctx.load_texture("decor", image, Default::default());

        Self {
            app,
            image_texture: Some(image),
            calory_add_modal: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct App {
    user_inf: UserInformation,
    user_dt: AccountData,
    workout_dt: AllWorkoutData,
    calory_dt: MacroData,
    all_states: States,
    selected_tab: usize,

    #[serde(skip)]
    image_texture: Option<TextureHandle>,
}

fn load_png(ctx: &egui::Context, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes).unwrap();
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let egui_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    ctx.load_texture("background", egui_image, egui::TextureOptions::LINEAR)
}

fn draw_stretched_background_x(ui: &mut egui::Ui, texture: &egui::TextureHandle, offset: f32) {
    let texture_size = texture.size_vec2(); 
    let available_rect = ui.max_rect(); 

    let available_width = available_rect.width();
    let scale = available_width / texture_size.x;

    let scaled_height = texture_size.y * scale;

    // 🧭 Вариант 1: Центрирование по вертикали
    // let y_centered = available_rect.center().y - scaled_height / 2.0;

    // 🧭 Вариант 2: Приподнять на 50px от верха (можно изменить на любое значение)
    let y_top = available_rect.min.y - offset;

    let rect = egui::Rect::from_min_size(
        egui::pos2(available_rect.min.x, y_top), 
        egui::vec2(available_width, scaled_height),
    );

    let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

    ui.painter().add(egui::Shape::image(
        texture.id(),
        rect,
        uv,
        egui::Color32::LIGHT_BLUE,
    ));
}


impl App {
    pub fn new() -> Self {
        let appdata_path = "appdata.json";

        let mut app_data: App = {
            let content = fs::read_to_string(&appdata_path).expect("failed to read appdata");
            serde_json::from_str(&content).expect("failed to parse appdata")
        };

        app_data
    }

    fn home_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        // ui.add_space(10.0);
        let available_width = ui.available_width();


        let default_pp = egui::include_image!("../user.jpg");

        let total_width = 450.0;
        let screen_width = ui.available_width();
        let padding = ((screen_width - total_width) / 2.0).max(0.0);

        let pen = egui::include_image!("../pen.png");

        ui.horizontal(|ui| {
            ui.add_space(padding);

            ui.vertical(|ui| {
                ui.add_sized([100.0, 100.0], egui::Image::new(default_pp).corner_radius(5.0));
                // ui.add(egui::Image::new(default_pp.clone()).size(egui::vec2(100.0, 100.0))); // Указываем размер картинки через `size()`
                ui.label(egui::RichText::new(format!("{}", self.user_inf.name)).size(20.0).strong());
                ui.label(egui::RichText::new(format!("@{}", self.user_inf.username)).size(15.0));
            });

            ui.add_space(5.0);

            ui.vertical(|ui| {
                ui.add_space(12.5);
                fn row(ui: &mut egui::Ui, left: String, right: String) {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(160.0);
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                                ui.label(RichText::new(left).size(13.0));
                            });
                        });

                        ui.label(RichText::new(right).size(13.0));
                    });
                }

                row(ui, format!("age: {}", self.user_inf.age), format!("lifted weight: {}", self.user_dt.lifted_weight));
                row(ui, format!("weight: {}", self.user_inf.weight), format!("registered cardio: {}", self.user_dt.registrated_cals));
                row(ui, format!("height: {}", self.user_inf.height), format!("registered calories: {}", self.user_dt.registrated_cals));
                row(ui, format!("registration date: {}", self.user_inf.registration_date), format!("registered meals: {}", self.user_dt.registrated_meals));

                ui.add_space(15.0);
                ui.add(egui::Button::image_and_text(pen, "edit profile").corner_radius(10).min_size(egui::vec2(300.0, 30.0)));
            });
        });

        ui.vertical_centered(|ui| {
            ui.add_space(5.0);
            ui.separator();

            ui.add_space(5.0);
            ui.add_sized(egui::vec2(ui.available_width() - 90.0, 10.0), egui::ProgressBar::new(0.0).show_percentage());

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("calory tracker").size(17.5).strong());
                let fs = egui::include_image!("../fs.png");
                if ui.add(egui::Button::image_and_text(fs, "")).clicked() {
                    self.selected_tab = 3;
                };
            });
            ui.add_space(5.0);
        });

        ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0);

        let spacing = 3.0;
        let rect_size = 10.0;
        let cols = 25;

        let total_width = (rect_size * cols as f32) + (spacing * (cols as f32 - 1.0));
        let available_width = ui.available_width();

        let mut percent = 0_u32;
        ui.vertical_centered(|ui| {
            percent = ((self.calory_dt.calory_registered as f32 / self.calory_dt.calory_goal as f32) * 100.0) as u32;
            ui.label(RichText::new(format!("{} %", &percent)).size(18.0));
        });

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width((available_width - total_width - 30.0) / 2.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(25.0);
                    ui.label(RichText::new("REGISTERED").size(15.0).strong());
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("proteins: {}", self.calory_dt.protein_registered)).size(13.0));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("carbs: {}", self.calory_dt.carb_registered)).size(13.0));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("fats: {}", self.calory_dt.fat_registered)).size(13.0));
                })
            });

            ui.vertical(|ui| {
                ui.set_width(total_width + 25.0);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("cals registered: {}", self.calory_dt.calory_registered)).size(13.0));
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("your goal: {}", self.calory_dt.calory_goal)).size(13.0));
                        });
                    });

                    ui.add_space(5.0);

                    self.calory_tracker_bar(ctx, frame, ui, spacing, rect_size, cols, percent);

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("cals remains: {}", self.calory_dt.calory_goal.saturating_sub(self.calory_dt.calory_registered))).size(13.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("meals registered: {}", self.calory_dt.meal_registered)).size(13.0));
                        });
                    });
                });
            });

            ui.vertical(|ui| {
                ui.set_width((available_width - total_width - 30.0) / 2.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(25.0);
                    ui.label(RichText::new("REMAINS").size(15.0).strong());
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!(
                        "proteins: {}",
                        self.calory_dt.protein_goal.saturating_sub(self.calory_dt.protein_registered)
                    )).size(13.0));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!(
                        "carbs: {}",
                        self.calory_dt.carb_goal.saturating_sub(self.calory_dt.carb_registered)
                    )).size(13.0));
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!(
                        "fats: {}",
                        self.calory_dt.fat_goal.saturating_sub(self.calory_dt.fat_registered)
                    )).size(13.0));
                });

            });
        });


        ui.vertical_centered(|ui| {
            if ui.add(egui::Button::image_and_text(egui::include_image!("../plus.png"), "add cals").corner_radius(15.0).min_size(egui::vec2(70.0, 30.0)).wrap()).clicked() {
                self.all_states.calory_add_modal = true;
            }

            let modal_size = egui::vec2(300.0, 200.0);
            let modal_pos = ctx.screen_rect().center() - modal_size / 4.0;

            if self.all_states.calory_add_modal {
                let screen_rect = ctx.screen_rect();
                let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("dark_backdrop")));

                painter.rect_filled(
                    screen_rect,
                    0.0,
                    egui::Color32::from_black_alpha(180),
                );

                egui::Window::new("fast add calories")
                    .fixed_pos(modal_pos)
                    .fixed_size(modal_size)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("enter calories:");
                        ui.add(egui::DragValue::new(&mut self.all_states.calories));

                        ui.horizontal(|ui| {
                            ui.set_width(100.0);
                            ui.vertical(|ui| {
                                ui.set_width(30.0);
                                ui.label("p");
                                ui.add(egui::DragValue::new(&mut self.all_states.proteins));
                            });

                            ui.vertical(|ui| {
                                ui.set_width(30.0);
                                ui.label("c");
                                ui.add(egui::DragValue::new(&mut self.all_states.carbs));
                            });

                            ui.vertical(|ui| {
                                ui.set_width(30.0);
                                ui.label("f");
                                ui.add(egui::DragValue::new(&mut self.all_states.fats));
                            })
                        });

                        ui.horizontal(|ui| {
                            if ui.button("ok").clicked() {
                                self.calory_dt.calory_registered += self.all_states.calories;
                                self.all_states.calories = 0;
                                self.calory_dt.protein_registered += self.all_states.proteins;
                                self.all_states.proteins = 0;
                                self.calory_dt.carb_registered += self.all_states.carbs;
                                self.all_states.carbs= 0;
                                self.calory_dt.fat_registered += self.all_states.fats;
                                self.all_states.fats= 0;

                                self.all_states.calory_add_modal = false;
                            }

                            if ui.button("cancel").clicked() {
                                self.all_states.calories = 0;
                                self.all_states.calory_add_modal = false;
                            }
                        });
                    });
            }

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("workout tracker").size(17.5).strong());
                ui.add_space(5.0);
                let fs = egui::include_image!("../fs.png");
                if ui.add(egui::Button::image_and_text(fs, "")).clicked() {
                    self.selected_tab = 2;
                };
            });
            ui.add_space(10.0);
            ui.label(RichText::new(format!("{} % of muscles worked out this week", self.workout_dt.worked_out)).size(15.0));
            ui.add_space(5.0);

            let available_width = ui.available_width();
            let column_width = available_width / 4.0;
            let spacing = 40.0;

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.add_space(spacing);
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new("PRS").strong().size(18.0));
                        ui.add_space(spacing / 5.0);
                        ui.label(RichText::new(format!("{}", self.workout_dt.prs)).size(16.0));
                        ui.add_space(spacing * 5.0);
                        ui.label(RichText::new("SETS").strong().size(18.0));
                        ui.add_space(spacing / 5.0);
                        ui.label(RichText::new(format!("{}", self.workout_dt.week_sets)).size(16.0));
                    });
                });

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.vertical_centered(|ui| {
                        workout_tracker_widget_front(ctx, frame, ui);
                    });
                });

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.vertical_centered(|ui| {
                        workout_tracket_widget_behind(ctx, frame, ui);
                    });
                });

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.add_space(spacing);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("VOLUME").strong().size(18.0));
                        ui.add_space(spacing / 5.0);
                        ui.label(RichText::new(format!("{} Kgs", self.workout_dt.week_volume)).size(16.0));
                        ui.add_space(spacing * 5.0);
                        ui.label(RichText::new("REPS").strong().size(18.0));
                        ui.add_space(spacing / 5.0);
                        ui.label(RichText::new(format!("{}", self.workout_dt.week_reps)).size(16.0));
                    });
                });
            });

            ui.add_space(spacing / 2.0);

            ui.label(RichText::new("TIME SPENT").strong().size(18.0));
            ui.add_space(spacing / 5.0);
            ui.label(RichText::new(format!("{} Hrs", self.workout_dt.week_time)).size(16.0));
        });
    }

    fn friends_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.heading("FRIENDS");
    }

    fn workouts_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, image_texture: Option<&TextureHandle>) {
        // ui.heading("WORKOUTS");

        // let bg_color = if is_dark {
        //     egui::Color32::from_rgb(20, 20, 20) // тёмный фон
        // } else {
        //     egui::Color32::from_rgb(240, 240, 240) // светлый фон
        // };
        let is_dark = ctx.style().visuals.dark_mode;

        let elements_color = if is_dark {
            egui::Color32::from_rgb(27, 27, 27)
        } else {
            egui::Color32::from_rgb(217, 217, 217)
        };

        let side_rect =egui::Rect::from_min_size(
            // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
            ctx.screen_rect().left_top() + vec2(50.0, 125.0),
            egui::vec2(ui.available_width() - 100.0, 600.0),
        );

        ui.vertical_centered(|ui| {
            let id = egui::Id::new("drag_x_rect");

            let mut pos = ctx.data(|data| {
                data.get_temp::<Pos2>(id).unwrap_or(side_rect.left_top())
            });

            let response = ui.allocate_rect(side_rect, egui::Sense::drag());

            if response.dragged() {
                let delta = response.drag_delta();
                pos.y += delta.y; 

                ctx.data_mut(|data| {
                    data.insert_temp(id, pos);
                });
            }

            ui.painter().rect_filled(
                Rect::from_min_size(pos, vec2(ui.available_width() - 100.0, 600.0)), 
                egui::epaint::Rounding {
                    nw: 24,
                    ne: 24,
                    sw: 24,
                    se: 24,
                },
                elements_color,
            );

            ui.allocate_ui_at_rect(Rect::from_min_size(pos, vec2(ui.available_width() - 100.0, 600.0)),|ui| {
                // ui.vertical_centered(|ui| {
                //     ui.add_space(280.0);
                //     ui.add(Label::new(RichText::new("not planned").size(24.0)).selectable(false));
                //     ui.add_space(200.0);
                // });
                // 
                // ui.horizontal(|ui| {
                //     ui.add_space(side_rect.width() / 13.0);
                // 
                //     ui.add(
                //         Button::new(RichText::new("rest").size(22.0).color(Color32::WHITE))
                //             //     egui::Color32::from_rgb(91, 0, 113),
                //             .fill(Color32::from_rgb(91, 0, 113)) // цвет фона кнопки
                //             .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                //             .rounding(10),
                //             // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), // рамка
                //     );
                // 
                //     let padding = side_rect.width() - (((side_rect.width() / 13.0) * 2.0) + ((side_rect.width() / 2.5) * 2.0)) - 8.0;
                //     ui.add_space(padding);
                // 
                //     ui.add(
                //         Button::new(RichText::new("add workout").size(22.0).color(Color32::WHITE))
                //             //     egui::Color32::from_rgb(91, 0, 113),
                //             .fill(Color32::from_rgb(0, 75, 141)) // цвет фона кнопки
                //             .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                //             .rounding(10),
                //         // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), // рамка
                //     );
                // });

                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.label(RichText::new("legs & shoulders").size(27.0).strong());
                    ui.add_space(50.0);
                
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(side_rect.width() / 2.0);
                            ui.vertical_centered(|ui| {
                                workout_tracker_widget_front(ctx, frame, ui);
                            });
                        });
                
                        ui.vertical(|ui| {
                            ui.set_width(side_rect.width() / 2.0);
                            ui.vertical_centered(|ui| {
                                workout_tracket_widget_behind(ctx, frame, ui);
                            });
                        });
                    });
                
                    ui.add_space(50.0);
                
                    ui.add(
                        Button::new(RichText::new("start").size(22.0).strong().color(Color32::WHITE))
                            //     egui::Color32::from_rgb(91, 0, 113),
                            .fill(Color32::from_rgb(21, 141, 0)) // цвет фона кнопки
                            .min_size(Vec2::new(side_rect.width() / 4.0, 40.0))
                            .rounding(10),
                    );
                
                    ui.add_space(12.0);
                
                    ui.horizontal(|ui| {
                        let button_width = side_rect.width() / 4.0;
                        let spacing = 6.0;
                        let total_width = button_width * 3.0 + spacing * 2.0;
                
                        let left_padding = (side_rect.width() - total_width) / 2.1;
                        ui.add_space(left_padding);
                
                        // Левая кнопка
                        ui.add(
                            Button::new(RichText::new("change workout").size(15.0).strong().color(Color32::WHITE))
                                .fill(Color32::from_rgb(0, 75, 141))
                                .min_size(Vec2::new(button_width, 30.0))
                                .rounding(8),
                        );
                
                        ui.add_space(spacing);
                
                        // Центральная кнопка
                        ui.add(
                            Button::new(RichText::new("rest").size(18.0).strong().color(Color32::WHITE))
                                .fill(Color32::from_rgb(91, 0, 113))
                                .min_size(Vec2::new(button_width, 30.0))
                                .rounding(8),
                        );
                
                        ui.add_space(spacing);
                
                        // Правая кнопка
                        ui.add(
                            Button::new(RichText::new("skip").size(18.0).color(Color32::WHITE))
                                .fill(Color32::from_rgb(141, 0, 19))
                                .min_size(Vec2::new(button_width, 30.0))
                                .rounding(8),
                        );
                    });
                
                    //
                    // let padding = side_rect.width() - (((side_rect.width() / 13.0) * 2.0) + ((side_rect.width() / 2.5) * 2.0)) - 8.0;
                    // ui.add_space(padding);
                    //
                    // ui.add(
                    //     Button::new(RichText::new("add workout").size(22.0))
                    //         //     egui::Color32::from_rgb(91, 0, 113),
                    //         .fill(Color32::from_rgb(0, 75, 141)) // цвет фона кнопки
                    //         .min_size(Vec2::new(side_rect.width() / 2.5, 40.0))
                    //         .rounding(10),
                    //     // .stroke(egui::Stroke::new(1.0, Color32::WHITE)), // рамка
                    // );
                });

                // let rest_button = egui::Rect::from_min_size(
                //     side_rect.left_top() + vec2(30.0, 480.0),
                //     vec2(ui.available_width() - 400.0, 40.0),
                // );
                
                // ui.painter().rect_filled(
                //     rest_button,
                //     egui::epaint::Rounding {
                //         nw: 12,
                //         ne: 12,
                //         sw: 12,
                //         se: 12,
                //     },
                //     egui::Color32::from_rgb(91, 0, 113),
                // );

            });

        });

        let top_rect = egui::Rect::from_min_size(
            ctx.screen_rect().left_top(),
            egui::vec2(ui.available_width(), 100.0),
        );

        Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
            nw: 0,
            ne: 0,
            sw: 24,
            se: 24,
        });

        let now = OffsetDateTime::now_local().unwrap();

        ui.allocate_ui_at_rect(top_rect, |ui| {

            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.add(Label::new(RichText::new(format!("{} {}", now.month(), now.day())).size(25.0).strong()).selectable(false));
                ui.add(Label::new(RichText::new(format!("{}", now.weekday())).size(15.0).strong()).selectable(false));
            });
        });

        ui.add_space(25.0);

        let bot_rect = egui::Rect::from_min_size(
            ctx.screen_rect().left_bottom() - vec2(0.0, 150.0),
            ctx.screen_rect().right_bottom().to_vec2(),
        );

        Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 24, elements_color, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
            nw: 24,
            ne: 24,
            sw: 0,
            se: 0,
        });
        
        let weekdays = ["mo", "tu", "we", "th", "fr", "sa", "su"];

        ui.allocate_ui_at_rect(bot_rect, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.set_width(side_rect.width());
                ui.add_space(top_rect.height() / 4.0);

                let rect_width = side_rect.width() / 8.0;

                ui.horizontal(|ui| {
                    ui.add_space((side_rect.width() - rect_width * 7.0) / 9.0);
                    let today = weekday_iso(now.weekday());

                    for i in 1..=7 {
                        let offset = i as i32 - today as i32;
                        let date = now + Duration::days(offset.into());

                        ui.vertical(|ui| {
                            ui.add(Label::new(format!(" {}", weekdays[i - 1])).selectable(false));

                            ui.add(Button::new(RichText::new(format!("{}", date.day()))
                                .size(18.0)
                                .color(Color32::WHITE))
                                .fill(if i as u8 == today { Color32::from_rgb(0, 75, 142) } else { Color32::from_rgb(96, 96, 96) })
                                .min_size(Vec2::new(rect_width, 60.0))
                                .rounding(8))
                        });
                    }
                });
            });
        });

        draw_stretched_background_x(ui, image_texture.unwrap(), -70.0);
    }

    fn calory_tracker_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, image_texture: Option<&TextureHandle>) {

        let is_dark = ctx.style().visuals.dark_mode;

        let elements_color = if is_dark {
            egui::Color32::from_rgb(27, 27, 27)
        } else {
            egui::Color32::from_rgb(217, 217, 217)
        };

        let top_rect = egui::Rect::from_min_size(
            ctx.screen_rect().left_top(),
            egui::vec2(ui.available_width(), 100.0),
        );

        Self::draw_rect_with_black_shadow(ui.painter(), top_rect, 24, elements_color, 6.0, [(5.0, 20), (3.0, 25), (2.0, 30),], Rounding {
            nw: 0,
            ne: 0,
            sw: 24,
            se: 24,
        });

        let now = OffsetDateTime::now_local().unwrap();

        ui.allocate_ui_at_rect(top_rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.add(Label::new(RichText::new(format!("{} {}", now.month(), now.day())).size(25.0).strong()).selectable(false));
                ui.add(Label::new(RichText::new(format!("{}", now.weekday())).size(15.0).strong()).selectable(false));
            });
        });

        ui.add_space(40.0);

        ui.vertical_centered(|ui| {
            ui.add(Label::new(RichText::new("CALORIES").size(25.0).strong()).selectable(false));

            let calory_rect =egui::Rect::from_min_size(
                // top_rect.left_top() + egui::vec2(50.0, top_rect.height() + 25.0),
                ctx.screen_rect().left_top() + vec2(150.0, 155.0),
                egui::vec2(ui.available_width() - 300.0, 100.0),
            );

            ui.painter().rect_filled(
                calory_rect,
                egui::epaint::Rounding {
                    nw: 24,
                    ne: 24,
                    sw: 24,
                    se: 24,
                },
                elements_color,
            );

            ui.allocate_ui_at_rect(calory_rect, |ui| {
                ui.add_space(calory_rect.width() / 10.0);

                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(format!("{}/{}", self.calory_dt.calory_registered, self.calory_dt.calory_goal)).size(35.0).strong());
                });
            });
        });

        ui.add_space(50.0);

        let spacing = 3.0;
        let rect_size = 10.0;
        let cols = 25;

        let percent = ((self.calory_dt.calory_registered as f32 / self.calory_dt.calory_goal as f32) * 100.0) as u32;

        let total_width = (rect_size * cols as f32) + (spacing * (cols as f32 - 1.0));
        let available_width = ui.available_width();

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width((available_width - total_width - 30.0) / 2.0);
                });
            });

            ui.vertical(|ui| {
                ui.set_width(total_width + 25.0);
                ui.vertical_centered(|ui| {
                    ui.set_width(total_width + 25.0);
                    ui.vertical_centered(|ui| {
                        self.calory_tracker_bar(ctx, frame, ui, spacing, rect_size, cols, percent);
                    });
                });
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {

            let rect_length = 130.0;

            let available_width = ui.available_width();

            let carbs_rect = Rect::from_min_size(
                Pos2::new((available_width/ 2.0 ) - (rect_length / 2.0), ctx.screen_rect().min.y + 405.0),
                Vec2::new(rect_length, ctx.screen_rect().min.y  + 50.0)
            );

            let proteins_rect = Rect::from_min_size(
                carbs_rect.left_top() - Vec2::new(rect_length + 10.0, 0.0),
                Vec2::new(rect_length, ctx.screen_rect().min.y + 50.0),
            );

            let fats_rect = Rect::from_min_size(
                carbs_rect.right_top() + Vec2::new(10.0, 0.0),
                Vec2::new(rect_length, ctx.screen_rect().min.y + 50.0),
            );

            ui.add(Label::new(RichText::new("MACROS").size(25.0).strong()).selectable(false));

            ui.horizontal(|ui| {
                ui.painter().rect_filled(
                    carbs_rect,
                    egui::epaint::Rounding {
                        nw: 14,
                        ne: 14,
                        sw: 14,
                        se: 14,
                    },
                    elements_color,
                );

                ui.allocate_ui_at_rect(carbs_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.label(RichText::new("carbs").strong().color(egui::Color32::from_rgb(141, 54, 0)).size(14.0));
                        ui.label(RichText::new(format!("{}/{}", self.calory_dt.carb_registered, self.calory_dt.carb_goal)).size(23.0).strong());
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(31.0);
                            self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.calory_dt.carb_registered, self.calory_dt.carb_goal);
                        });
                    });
                });

                ui.painter().rect_filled(
                    proteins_rect,
                    egui::epaint::Rounding {
                        nw: 14,
                        ne: 14,
                        sw: 14,
                        se: 14,
                    },
                    elements_color,
                );

                ui.allocate_ui_at_rect(proteins_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.label(RichText::new("proteins").strong().color(egui::Color32::from_rgb(0, 75, 140)).size(14.0));
                        ui.label(RichText::new(format!("{}/{}", self.calory_dt.protein_registered, self.calory_dt.protein_goal)).size(23.0).strong());
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(31.0);
                            self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.calory_dt.protein_registered, self.calory_dt.protein_goal);
                        });
                    });
                });

                ui.painter().rect_filled(
                    fats_rect,
                    egui::epaint::Rounding {
                        nw: 14,
                        ne: 14,
                        sw: 14,
                        se: 14,
                    },
                    elements_color,
                );

                ui.allocate_ui_at_rect(fats_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.label(RichText::new("fats").strong().color(egui::Color32::from_rgb(141, 0, 19)).size(14.0));
                        ui.label(RichText::new(format!("{}/{}", self.calory_dt.fat_registered, self.calory_dt.fat_goal)).size(23.0).strong());
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(31.0);
                            self.mini_tracker_bar(ctx, frame, ui, spacing, rect_size, 5, self.calory_dt.fat_registered, self.calory_dt.fat_goal);
                        });
                    });
                });
            });

            ui.add_space(5.0);

            ui.add(Label::new(RichText::new("HISTORY").size(25.0).strong()).selectable(false));

            let history_rect = Rect::from_min_size(
                Pos2::new((ui.available_width() / 2.0) - 240.0, ctx.screen_rect().min.y + 590.0),
                Vec2::new(480.0, 150.0),
            );

            ui.painter().rect_filled(
                history_rect,
                egui::epaint::Rounding {
                    nw: 28,
                    ne: 28,
                    sw: 28,
                    se: 28,
                },
                elements_color,
            );

            let bot_rect = egui::Rect::from_min_size(
                ctx.screen_rect().left_bottom() - vec2(0.0, 130.0),
                ctx.screen_rect().right_bottom().to_vec2(),
            );

            Self::draw_rect_with_black_shadow(ui.painter(), bot_rect, 110, elements_color, -4.0, [(2.0, 20), (3.0, 25), (5.0, 30)], Rounding {
                nw: 110,
                ne: 110,
                sw: 0,
                se: 0,
            });

            ui.allocate_ui_at_rect(bot_rect, |ui| {
                ui.add_space(30.0);
                ui.add(
                    Button::new(RichText::new("add calories").size(18.0).strong().color(Color32::WHITE))
                        //     egui::Color32::from_rgb(91, 0, 113),
                        .fill(Color32::from_rgb(21, 141, 0)) // цвет фона кнопки
                        .min_size(Vec2::new(120.0, 40.0))
                        .rounding(12),
                );
            });

            draw_stretched_background_x(ui, image_texture.unwrap(), 300.0);
        });
    }

    fn water_tracker_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.heading("WATER TRACKER");
    }

    fn statistics_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.heading("STATISTICS");
    }

    fn calory_tracker_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, spacing: f32, rect_size: f32, cols: i32, percent: u32) {
        ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0);

        let mut green_rects = {
            if self.calory_dt.calory_goal> self.calory_dt.calory_registered{
                (1.25 * percent as f32).round() as u32
            } else {
                125
            }
        };

        ui.vertical_centered(|ui| {
            for _ in 0..5 {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(rect_size, rect_size),
                            egui::Sense::hover(),
                        );

                        let color =  if green_rects > 0 {
                            green_rects -= 1;
                            Color32::from_rgb(0, 136, 255)
                            // Color32::DARK_BLUE
                        } else {
                            Color32::GRAY
                        };
                        
                        ui.painter().rect_filled(rect, 1.0, color);

                        if col < cols - 1 {
                            ui.add_space(spacing);
                        }
                    }
                });
            }
            ui.add_space(5.0);
        });
    }

    fn mini_tracker_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, spacing: f32, rect_size:f32, cols: i32, registered: u32, goal: u32) {
        // let prev_spacing = ui.spacing().item_spacing; // сохраняем
        ui.spacing_mut().item_spacing = egui::vec2(1.0, -3.0); // временно меняем

        let ROWS = 5;
        let COLUMNS = 5;

        let percent = ((registered as f32 / goal as f32) * 100.0) as u32;

        let mut remaining = if goal > registered {
            (((ROWS * COLUMNS) as f32 / 100.0) * percent as f32).round() as u32
        } else {
            ROWS * COLUMNS
        };

        ui.vertical(|ui| {
            for _ in 0..ROWS {
                ui.horizontal(|ui| {
                    for col in 0..COLUMNS {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(rect_size, rect_size),
                            egui::Sense::hover(),
                        );

                        let color = if remaining > 0 {
                            remaining -= 1;
                            Color32::from_rgb(0, 136, 255)
                        } else {
                            Color32::GRAY
                        };

                        ui.painter().rect_filled(rect, 1.0, color);

                        if col < COLUMNS - 1 {
                            ui.add_space(spacing);
                        }
                    }
                });
            }
        });
    }

    fn draw_rect_with_black_shadow(painter: &egui::Painter, rect: egui::Rect, rounding: u8, fill: Color32, offset_y: f32, layer: [(f32, u8); 3], corners: Rounding) {
        let shadow_color = |alpha: u8| Color32::from_rgba_unmultiplied(0, 0, 0, alpha);

        let shadow_offset = Vec2::new(0.0, offset_y);

        for (inflate_by, alpha) in layer {
            let shadow_rect = rect
                .translate(shadow_offset)
                .expand(inflate_by);
            painter.rect_filled(shadow_rect, egui::Rounding::same(rounding + inflate_by as u8), shadow_color(alpha));
        }

        painter.rect_filled(rect, corners
        ,fill);
    }


    fn testing(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        // ui.vertical_centered_justified(|ui| {
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width() / 2.0);
            ui.vertical_centered_justified(|ui| {
                workout_tracker_widget_front(ctx, frame, ui);
            });

            ui.add_space(50.0);

            ui.vertical_centered_justified(|ui| {
                workout_tracket_widget_behind(ctx, frame, ui);
            });
        });
    }

    fn ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui, image_texture: Option<&TextureHandle>, // 👈 добавили
        ) {
        match self.selected_tab {
            // 0 => self.testing(ctx, frame, ui),
            0 => self.home_ui(ctx, frame, ui),
            1 => self.friends_ui(ctx, frame, ui),
            2 => self.workouts_ui(ctx, frame, ui, image_texture),
            3 => self.calory_tracker_ui(ctx, frame, ui, image_texture),
            4 => self.water_tracker_ui(ctx, frame, ui),
            5 => self.statistics_ui(ctx, frame, ui),
            _ => {ui.label("empty");},
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!("../SF-Pro-Display-Semibold.otf"))), // или from_owned()
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    ctx.set_fonts(fonts);
}

impl eframe::App for AppRuntime {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_dark = ctx.style().visuals.dark_mode;

        let bg_color = if is_dark {
            egui::Color32::from_rgb(20, 20, 20) // тёмный фон
        } else {
            egui::Color32::from_rgb(240, 240, 240) // светлый фон
        };

        let elements_color = if is_dark {
            egui::Color32::from_rgb(50, 100, 200) // синий для dark
        } else {
            egui::Color32::from_rgb(200, 100, 50) // оранжевый для light
        };
        
        setup_custom_fonts(ctx);

        // рисуем на всю область
        ctx.layer_painter(egui::LayerId::background())
            .rect_filled(ctx.screen_rect(), egui::epaint::Rounding::ZERO, bg_color);

        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE,
            )
            // .frame(egui::Frame::default().inner_margin(egui::Margin::same(0)).outer_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                self.app.ui(ctx, frame, ui, self.image_texture.as_ref());
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(20.0)
            .show(ctx, |ui| {
                // ui.horizontal(|ui| {
                //     ui.selectable_label("home");
                //     ui.selectable_label(, text)
                // })
                // let desired_width = 300.0;
                // let available_width = ui.available_width();
                // let offset_x = (available_width - desired_width) / 2.0;

                // ui.add_space(offset_x.max(0.0));

                // ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| { //     ui.add(Button::image_and_text(home,"home"));
                // ui.allocate_ui_with_layout(egui::vec2(desired_width, 60.0), Layout::left_to_right(egui::Align::Center), |ui|{
                // let total_width = ui.available_width();
                // let button_width = 80.0;
                // let spacing = 40.0;
                // let total_buttons_width = button_width * 3.0 + spacing * 2.0;
                // let left_padding = (total_width - total_buttons_width) / 2.0;

                // ui.add_space(left_padding.max(0.0));

                // ui.horizontal(|ui| {
                //     if ui.add_sized([button_width, 30.0], egui::SelectableLabel::new(self.selected_tab == 0, "home")).clicked() {
                //         self.selected_tab = 0;
                //     }

                //     ui.add_space(spacing);

                //     if ui.add_sized([button_width, 30.0], egui::SelectableLabel::new(self.selected_tab == 1, "session")).clicked() {
                //         self.selected_tab = 1;
                //     }

                //     ui.add_space(spacing);

                //     if ui.add_sized([button_width, 30.0], egui::SelectableLabel::new(self.selected_tab == 2, "stats")).clicked() {
                //         self.selected_tab = 2;
                //     }
                // });
                // ui.horizontal_centered(|ui|{

                //     unsafe {
                //         if ui.add(egui::SelectableLabel::new(self.selected_tab == 0, "home")).clicked() {
                //             self.selected_tab = 0;
                //         }

                //         if ui.add(egui::SelectableLabel::new(self.selected_tab == 1, "friends")).clicked() {
                //             self.selected_tab = 1;
                //         }

                //         if ui.add(egui::SelectableLabel::new(self.selected_tab == 2, "workouts")).clicked() {
                //             self.selected_tab = 2;
                //         }

                //         if ui.add(egui::SelectableLabel::new(self.selected_tab == 3, "calory tracker")).clicked() {
                //             self.selected_tab = 3;
                //         }

                //         if ui.add(egui::SelectableLabel::new(self.selected_tab == 4, "statistics")).clicked() {
                //             self.selected_tab = 4;
                //         }
                //     }
                // });
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());

                    let button_count = 6;
                    let button_width = 85.0;
                    let total_buttons_width = button_count as f32 * button_width;
                    let left_padding = (ui.available_width() - total_buttons_width).max(0.0) / 2.0 - 20.0;

                    ui.add_space(left_padding);

                    for (i, label) in ["🏠", "🌍", "💪", "🍴", "💧", "📈"].iter().enumerate() {
                        let selected = self.app.selected_tab == i;
                        let button = egui::SelectableLabel::new(selected, *label);
                        if ui.add_sized(egui::vec2(button_width, 30.0), button).clicked() {
                            self.app.selected_tab = i;
                        }
                    }
                });
            });
    }
}