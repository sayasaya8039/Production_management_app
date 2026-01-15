#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const APP_VERSION: &str = "0.1.0";
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
                Category::new("Extensions"),
                Category::new("Web Apps"),
                Category::new("Windows Apps"),
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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

    fn export_category_markdown(&self, category_index: usize) {
        if let Some(category) = self.data.categories.get(category_index) {
            let markdown = category.to_markdown();
            let filename = format!("{}.md", category.name.replace(" ", "_"));
            
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&filename)
                .add_filter("Markdown", &["md"])
                .save_file()
            {
                if fs::write(&path, &markdown).is_ok() {
                    println!("Exported to {:?}", path);
                }
            }
        }
    }
}

impl eframe::App for ProductionManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.status_timer > 0.0 {
            self.status_timer -= ctx.input(|i| i.predicted_dt);
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(format!("{} v{}", APP_NAME, APP_VERSION));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !self.status_message.is_empty() && self.status_timer > 0.0 {
                        ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::GREEN));
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();
            let column_count = self.data.categories.len();
            let column_width = (available_width / column_count as f32) - 10.0;

            ui.horizontal(|ui| {
                for cat_idx in 0..self.data.categories.len() {
                    ui.vertical(|ui| {
                        ui.set_width(column_width);
                        self.render_category(ui, cat_idx, column_width);
                    });
                    ui.add_space(5.0);
                }
            });
        });

        if ctx.input(|i| i.pointer.any_released()) {
            if let (Some((from_cat, from_idx)), Some((to_cat, to_idx))) = (self.dragging, self.drag_target) {
                if from_cat == to_cat {
                    self.data.categories[from_cat].move_item(from_idx, to_idx);
                    self.save_data();
                    self.show_status("Item reordered");
                }
            }
            self.dragging = None;
            self.drag_target = None;
        }

        self.render_add_popup(ctx);
        self.render_edit_popup(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_data();
    }
}

impl ProductionManager {
    fn render_category(&mut self, ui: &mut egui::Ui, cat_idx: usize, column_width: f32) {
        let category_name = self.data.categories[cat_idx].name.clone();

        ui.group(|ui| {
            ui.set_width(column_width - 10.0);

            ui.horizontal(|ui| {
                ui.heading(&category_name);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("MD").on_hover_text("Export to Markdown").clicked() {
                        self.export_category_markdown(cat_idx);
                    }

                    ui.menu_button("Sort", |ui| {
                        if ui.button("By Title").clicked() {
                            self.data.categories[cat_idx].sort_by_title();
                            self.save_data();
                            ui.close_menu();
                        }
                        if ui.button("By Date").clicked() {
                            self.data.categories[cat_idx].sort_by_date();
                            self.save_data();
                            ui.close_menu();
                        }
                    });
                });
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 50.0)
                .show(ui, |ui| {
                    let mut action: Option<(String, bool)> = None;
                    let items_len = self.data.categories[cat_idx].items.len();

                    for item_idx in 0..items_len {
                        let is_dragging = self.dragging == Some((cat_idx, item_idx));
                        let is_drag_target = self.drag_target == Some((cat_idx, item_idx));

                        if is_drag_target {
                            ui.colored_label(egui::Color32::YELLOW, ">> Drop here <<");
                        }

                        let item = &self.data.categories[cat_idx].items[item_idx];
                        let item_id = item.id.clone();
                        let item_title = item.title.clone();
                        let item_comment = item.comment.clone();
                        let item_created = item.created_at.clone();

                        let frame = egui::Frame::none()
                            .fill(if is_dragging {
                                egui::Color32::from_rgb(70, 70, 90)
                            } else {
                                egui::Color32::from_rgb(45, 45, 55)
                            })
                            .rounding(5.0)
                            .inner_margin(8.0);

                        frame.show(ui, |ui| {
                            ui.set_width(column_width - 30.0);

                            let response = ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("=").monospace());
                                    ui.strong(&item_title);
                                });

                                if !item_comment.is_empty() {
                                    ui.label(egui::RichText::new(&item_comment).small().color(egui::Color32::GRAY));
                                }

                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&item_created).small().color(egui::Color32::DARK_GRAY));

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.small_button("X").on_hover_text("Delete").clicked() {
                                            action = Some((item_id.clone(), true));
                                        }
                                        if ui.small_button("E").on_hover_text("Edit").clicked() {
                                            action = Some((item_id.clone(), false));
                                        }
                                    });
                                });
                            });

                            let response = response.response.interact(egui::Sense::drag());

                            if response.drag_started() {
                                self.dragging = Some((cat_idx, item_idx));
                            }

                            if response.hovered() && self.dragging.is_some() && self.dragging != Some((cat_idx, item_idx)) {
                                self.drag_target = Some((cat_idx, item_idx));
                            }
                        });

                        ui.add_space(5.0);
                    }

                    if let Some((id, is_delete)) = action {
                        if is_delete {
                            self.data.categories[cat_idx].remove_item(&id);
                            self.save_data();
                            self.show_status("Item deleted");
                        } else {
                            if let Some(item) = self.data.categories[cat_idx].items.iter().find(|i| i.id == id) {
                                self.show_edit_popup = true;
                                self.edit_category = cat_idx;
                                self.edit_item_id = id;
                                self.edit_item_title = item.title.clone();
                                self.edit_item_comment = item.comment.clone();
                            }
                        }
                    }
                });

            ui.separator();
            if ui.button("+ Add Item").clicked() {
                self.show_add_popup = true;
                self.add_popup_category = cat_idx;
                self.new_item_title.clear();
                self.new_item_comment.clear();
            }
        });
    }

    fn render_add_popup(&mut self, ctx: &egui::Context) {
        if !self.show_add_popup {
            return;
        }

        egui::Window::new("Add New Item")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Category:");
                    ui.label(egui::RichText::new(&self.data.categories[self.add_popup_category].name).strong());
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.new_item_title);
                });

                ui.add_space(5.0);

                ui.label("Comment:");
                ui.add(egui::TextEdit::multiline(&mut self.new_item_comment).desired_width(300.0).desired_rows(4));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_add_popup = false;
                    }

                    ui.add_space(10.0);

                    let can_add = !self.new_item_title.trim().is_empty();
                    if ui.add_enabled(can_add, egui::Button::new("Add")).clicked() {
                        self.data.categories[self.add_popup_category].add_item(
                            self.new_item_title.trim().to_string(),
                            self.new_item_comment.trim().to_string(),
                        );
                        self.save_data();
                        self.show_add_popup = false;
                        self.show_status("Item added");
                    }
                });
            });
    }

    fn render_edit_popup(&mut self, ctx: &egui::Context) {
        if !self.show_edit_popup {
            return;
        }

        egui::Window::new("Edit Item")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.edit_item_title);
                });

                ui.add_space(5.0);

                ui.label("Comment:");
                ui.add(egui::TextEdit::multiline(&mut self.edit_item_comment).desired_width(300.0).desired_rows(4));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_edit_popup = false;
                    }

                    ui.add_space(10.0);

                    let can_save = !self.edit_item_title.trim().is_empty();
                    if ui.add_enabled(can_save, egui::Button::new("Save")).clicked() {
                        if let Some(item) = self.data.categories[self.edit_category]
                            .items
                            .iter_mut()
                            .find(|i| i.id == self.edit_item_id)
                        {
                            item.title = self.edit_item_title.trim().to_string();
                            item.comment = self.edit_item_comment.trim().to_string();
                        }
                        self.save_data();
                        self.show_edit_popup = false;
                        self.show_status("Item updated");
                    }
                });
            });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
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
