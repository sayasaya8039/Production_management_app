#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const APP_VERSION: &str = "0.14.0";
const APP_NAME: &str = "Production Manager";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    id: String,
    title: String,
    comment: String,
    order: usize,
    created_at: String,
}

impl Item {
    fn new(title: String, comment: String, order: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            comment,
            order,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Category {
    name: String,
    items: Vec<Item>,
}

impl Category {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, title: String, comment: String) {
        let order = self.items.len();
        self.items.push(Item::new(title, comment, order));
    }

    fn remove_item(&mut self, id: &str) {
        self.items.retain(|item| item.id != id);
        self.reorder_items();
    }

    fn reorder_items(&mut self) {
        for (i, item) in self.items.iter_mut().enumerate() {
            item.order = i;
        }
    }

    fn sort_by_title(&mut self) {
        self.items.sort_by(|a, b| a.title.cmp(&b.title));
        self.reorder_items();
    }

    fn sort_by_date(&mut self) {
        self.items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        self.reorder_items();
    }

    fn move_item(&mut self, from: usize, to: usize) {
        if from < self.items.len() && to <= self.items.len() {
            let item = self.items.remove(from);
            let insert_at = if to > from { to - 1 } else { to };
            self.items.insert(insert_at.min(self.items.len()), item);
            self.reorder_items();
        }
    }

    fn to_markdown(&self) -> String {
        let mut md = format!("# {}\n\n", self.name);
        for item in &self.items {
            md.push_str(&format!("## {}\n\n", item.title));
            if !item.comment.is_empty() {
                md.push_str(&format!("{}\n\n", item.comment));
            }
            md.push_str(&format!("*Created: {}*\n\n---\n\n", item.created_at));
        }
        md
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AppData {
    categories: Vec<Category>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            categories: vec![
                Category::new("拡張機能"),
                Category::new("Webアプリ"),
                Category::new("Windowsアプリ"),
            ],
        }
    }
}

struct ProductionManager {
    data: AppData,
    data_path: PathBuf,
    show_add_popup: bool,
    add_popup_category: usize,
    new_item_title: String,
    new_item_comment: String,
    show_edit_popup: bool,
    edit_category: usize,
    edit_item_id: String,
    edit_item_title: String,
    edit_item_comment: String,
    dragging: Option<(usize, usize)>,
    drag_target: Option<(usize, usize)>,
    status_message: String,
    status_timer: f32,
}

impl ProductionManager {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_fonts(&cc.egui_ctx);
        
        let data_path = Self::get_data_path();
        let data = Self::load_data(&data_path);
        
        Self {
            data,
            data_path,
            show_add_popup: false,
            add_popup_category: 0,
            new_item_title: String::new(),
            new_item_comment: String::new(),
            show_edit_popup: false,
            edit_category: 0,
            edit_item_id: String::new(),
            edit_item_title: String::new(),
            edit_item_comment: String::new(),
            dragging: None,
            drag_target: None,
            status_message: String::new(),
            status_timer: 0.0,
        }
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        
        // Try Noto Sans JP first (bundled), then system fonts
        let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
        let noto_path = exe_dir.as_ref().map(|d| d.join("fonts/NotoSansJP-Regular.ttf"));
        
        let japanese_loaded = if let Some(ref path) = noto_path {
            if let Ok(font_data) = std::fs::read(path) {
                fonts.font_data.insert("noto_jp".to_owned(), FontData::from_owned(font_data).into());
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "noto_jp".to_owned());
                fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "noto_jp".to_owned());
                true
            } else { false }
        } else { false };
        
        if !japanese_loaded {
            if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/YuGothM.ttc") {
                fonts.font_data.insert("yu_gothic".to_owned(), FontData::from_owned(font_data).into());
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "yu_gothic".to_owned());
                fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "yu_gothic".to_owned());
            }
        }
        
        // Add emoji font at the end as fallback
        if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/seguiemj.ttf") {
            fonts.font_data.insert("emoji".to_owned(), FontData::from_owned(font_data).into());
            fonts.families.get_mut(&FontFamily::Proportional).unwrap().push("emoji".to_owned());
            fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("emoji".to_owned());
        }
        
        ctx.set_fonts(fonts);
    }

    fn get_data_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("ProductionManager");
        fs::create_dir_all(&path).ok();
        path.push("data.json");
        path
    }

    fn load_data(path: &PathBuf) -> AppData {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => AppData::default(),
            }
        } else {
            AppData::default()
        }
    }

    fn save_data(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.data) {
            fs::write(&self.data_path, json).ok();
        }
    }

    fn show_status(&mut self, message: &str) {
        self.status_message = message.to_string();
        self.status_timer = 3.0;
    }
}

impl ProductionManager {
    fn render_category(&mut self, ui: &mut egui::Ui, cat_idx: usize, column_height: f32) {
        let cat_name = self.data.categories[cat_idx].name.clone();
        let items_count = self.data.categories[cat_idx].items.len();
        let scroll_height = (column_height - 180.0).max(100.0);

        egui::Frame::default()
            .fill(egui::Color32::from_rgb(40, 40, 45))
            .rounding(10.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(236.0);
                ui.set_max_width(236.0);
                
                // Category header (centered)
                ui.vertical_centered(|ui| {
                    ui.heading(egui::RichText::new(&cat_name).size(18.0));
                    ui.label(egui::RichText::new(format!("{} items", items_count)).small().color(egui::Color32::from_rgb(150, 150, 155)));
                });

                ui.add_space(10.0);

                // Add button (centered)
                ui.vertical_centered(|ui| {
                    let btn_response = egui::Frame::default()
                        .fill(ui.visuals().widgets.inactive.bg_fill)
                        .rounding(4.0)
                        .inner_margin(egui::vec2(16.0, 6.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("➕").size(14.0));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("追加").size(16.0));
                            });
                        }).response.interact(egui::Sense::click());
                    
                    if btn_response.clicked() {
                        self.show_add_popup = true;
                        self.add_popup_category = cat_idx;
                        self.new_item_title.clear();
                        self.new_item_comment.clear();
                    }
                    if btn_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                });

                ui.add_space(8.0);

                // Sort & Export buttons (centered)
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        if ui.small_button("A-Z").clicked() {
                            self.data.categories[cat_idx].sort_by_title();
                            self.save_data();
                        }
                        if ui.small_button("Z-A").clicked() {
                            self.data.categories[cat_idx].sort_by_title();
                            self.data.categories[cat_idx].items.reverse();
                            self.data.categories[cat_idx].reorder_items();
                            self.save_data();
                        }
                        if ui.small_button("📅").clicked() {
                            self.data.categories[cat_idx].sort_by_date();
                            self.save_data();
                        }
                        if ui.small_button("📄").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name(&format!("{}.md", cat_name))
                                .add_filter("Markdown", &["md"])
                                .save_file()
                            {
                                let md = self.data.categories[cat_idx].to_markdown();
                                if fs::write(&path, md).is_ok() {
                                    self.show_status("Exported!");
                                }
                            }
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Items scroll area with unique ID
                let scroll_id = egui::Id::new(format!("cat_scroll_{}", cat_idx));
                egui::ScrollArea::vertical()
                    .id_salt(scroll_id)
                    .max_height(scroll_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(212.0);
                        
                        let items: Vec<_> = self.data.categories[cat_idx]
                            .items.iter().enumerate()
                            .map(|(i, item)| (i, item.id.clone(), item.title.clone(), item.comment.clone()))
                            .collect();

                        for (item_idx, item_id, title, comment) in items {
                            self.render_item(ui, cat_idx, item_idx, &item_id, &title, &comment);
                            ui.add_space(6.0);
                        }

                        // Drop zone at bottom
                        if self.dragging.is_some() {
                            let is_target = self.drag_target == Some((cat_idx, self.data.categories[cat_idx].items.len()));
                            let response = ui.allocate_response(egui::vec2(200.0, 24.0), egui::Sense::hover());
                            if response.hovered() {
                                self.drag_target = Some((cat_idx, self.data.categories[cat_idx].items.len()));
                            }
                            let color = if is_target {
                                egui::Color32::from_rgb(60, 120, 60)
                            } else {
                                egui::Color32::from_rgb(50, 50, 55)
                            };
                            ui.painter().rect_filled(response.rect, 4.0, color);
                        }
                    });
            });
    }


    fn render_item(&mut self, ui: &mut egui::Ui, cat_idx: usize, item_idx: usize, item_id: &str, title: &str, comment: &str) {
        let is_dragging = self.dragging == Some((cat_idx, item_idx));
        let is_target = self.drag_target == Some((cat_idx, item_idx));

        let frame_color = if is_dragging {
            egui::Color32::from_rgb(70, 70, 90)
        } else if is_target {
            egui::Color32::from_rgb(50, 90, 50)
        } else {
            egui::Color32::from_rgb(55, 55, 60)
        };

        let response = egui::Frame::default()
            .fill(frame_color)
            .rounding(6.0)
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width() - 8.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("☰").weak());
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.strong(title);
                        if !comment.is_empty() {
                            ui.label(egui::RichText::new(comment).small().color(egui::Color32::from_rgb(180, 180, 185)));
                        }
                    });
                });

                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    let item_id_owned = item_id.to_string();
                    let title_owned = title.to_string();
                    let comment_owned = comment.to_string();
                    
                    if ui.small_button("✏️").clicked() {
                        self.show_edit_popup = true;
                        self.edit_category = cat_idx;
                        self.edit_item_id = item_id_owned.clone();
                        self.edit_item_title = title_owned;
                        self.edit_item_comment = comment_owned;
                    }
                    if ui.small_button("🗑️").clicked() {
                        self.data.categories[cat_idx].remove_item(&item_id_owned);
                        self.save_data();
                        self.show_status("削除しました");
                    }
                });
            })
            .response;

        let response = response.interact(egui::Sense::drag());

        if response.drag_started() {
            self.dragging = Some((cat_idx, item_idx));
        }

        if response.hovered() && self.dragging.is_some() {
            self.drag_target = Some((cat_idx, item_idx));
        }

        if response.drag_stopped() {
            if let (Some((from_cat, from_idx)), Some((to_cat, to_idx))) = (self.dragging, self.drag_target) {
                if from_cat == to_cat && from_idx != to_idx {
                    self.data.categories[from_cat].move_item(from_idx, to_idx);
                    self.save_data();
                    self.show_status("移動しました");
                }
            }
            self.dragging = None;
            self.drag_target = None;
        }
    }

    fn render_add_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("新規追加")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let cat_name = self.data.categories[self.add_popup_category].name.clone();
                ui.label(format!("カテゴリ: {}", cat_name));

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    ui.label("タイトル:");
                    ui.add_sized([250.0, 20.0], egui::TextEdit::singleline(&mut self.new_item_title));
                });

                ui.add_space(8.0);

                ui.label("コメント:");
                ui.add(egui::TextEdit::multiline(&mut self.new_item_comment)
                    .desired_width(320.0)
                    .desired_rows(4));

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("キャンセル").clicked() {
                        self.show_add_popup = false;
                    }
                    ui.add_space(20.0);
                    let can_add = !self.new_item_title.trim().is_empty();
                    if ui.add_enabled(can_add, egui::Button::new("追加")).clicked() {
                        self.data.categories[self.add_popup_category].add_item(
                            self.new_item_title.trim().to_string(),
                            self.new_item_comment.trim().to_string(),
                        );
                        self.save_data();
                        self.show_add_popup = false;
                        self.show_status("追加しました");
                    }
                });
            });
    }

    fn render_edit_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("編集")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("タイトル:");
                    ui.add_sized([250.0, 20.0], egui::TextEdit::singleline(&mut self.edit_item_title));
                });

                ui.add_space(8.0);

                ui.label("コメント:");
                ui.add(egui::TextEdit::multiline(&mut self.edit_item_comment)
                    .desired_width(320.0)
                    .desired_rows(4));

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("キャンセル").clicked() {
                        self.show_edit_popup = false;
                    }
                    ui.add_space(20.0);
                    let can_save = !self.edit_item_title.trim().is_empty();
                    if ui.add_enabled(can_save, egui::Button::new("保存")).clicked() {
                        if let Some(item) = self.data.categories[self.edit_category]
                            .items.iter_mut()
                            .find(|i| i.id == self.edit_item_id)
                        {
                            item.title = self.edit_item_title.trim().to_string();
                            item.comment = self.edit_item_comment.trim().to_string();
                        }
                        self.save_data();
                        self.show_edit_popup = false;
                        self.show_status("保存しました");
                    }
                });
            });
    }
}

impl eframe::App for ProductionManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.status_timer > 0.0 {
            self.status_timer -= ctx.input(|i| i.unstable_dt);
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(format!("🎨 {} v{}", APP_NAME, APP_VERSION));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !self.status_message.is_empty() && self.status_timer > 0.0 {
                        ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::from_rgb(100, 200, 100)));
                    }
                });
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_height = ui.available_height();
            
            egui::ScrollArea::horizontal()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        for cat_idx in 0..self.data.categories.len() {
                            ui.push_id(cat_idx, |ui| {
                                ui.allocate_ui(egui::vec2(268.0, panel_height - 8.0), |ui| {
                                    self.render_category(ui, cat_idx, panel_height - 8.0);
                                });
                            });
                        }
                    });
                });
        });

        if self.show_add_popup {
            self.render_add_popup(ctx);
        }

        if self.show_edit_popup {
            self.render_edit_popup(ctx);
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title(format!("{} v{}", APP_NAME, APP_VERSION)),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(ProductionManager::new(cc)))),
    )
}
