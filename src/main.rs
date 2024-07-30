#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

// mod processes;

mod processes;

use crate::processes::{CPUsageCalculationValues, ProcessInfo};
use eframe::egui;
use eframe::epaint::text::TextWrapMode;
use egui::{RichText, TextStyle, Ui};
use egui_extras::{Column, TableBuilder};
use std::ops::Deref;
use std::time::{Duration, SystemTime};
use sysinfo::{Pid, Process, System};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "Rust Injector",
        options,
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
    .expect("Failed to start eframe");
}
#[derive(PartialEq)]
enum DisplayMode {
    Table,
    ComboBox
}
struct MyApp {
    process_name_search: String,
    age: u32,
    vector_select: Vec<String>,
    selected: usize,
    processes: Vec<ProcessInfo>,
    selected_process: usize,
    update_poll: f32,
    last_updated: SystemTime,
    run_mode: RunMode,
    display_mode: DisplayMode
}
enum RunMode {
    Reactive,
    Continuous,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            process_name_search: "None".to_owned(),
            age: 42,
            vector_select: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            processes: get_all_processes(),
            selected: 0,
            selected_process: 0,
            update_poll: 1000f32,
            last_updated: SystemTime::now(),
            run_mode: RunMode::Continuous,
            display_mode: DisplayMode::Table
        }
    }
}

enum Enum {
    First,
    Second,
    Third,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.run_mode {
            RunMode::Continuous => {
                ctx.request_repaint();
            }
            RunMode::Reactive => {}
        }
        if (SystemTime::now()
            .duration_since(self.last_updated)
            .unwrap()
            .as_millis()
            > 1000)
        {
            self.processes = get_all_processes();
            self.last_updated = SystemTime::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Processes");
            ui.separator();
            let current_process = &self.processes[self.selected_process];
            egui::Grid::new("process_info_grid").show(ui, |ui| {
                ui.end_row();
                let binding = current_process.expand().ok().unwrap();
                let sorted: Vec<_> = binding.iter().collect();
                for (key, value) in sorted {
                    ui.label(format!("{}:", key.to_string()));
                    ui.label(format!("{}", value.to_string()));
                    ui.end_row();
                }
            });
            let kill = ui.button("End Process");
            if(kill.clicked()){
                let system = sysinfo::System::new_all();
                if let Some(process) = system.process(current_process.pid){
                    sysinfo::Process::kill(process);
                }
            }
            egui::Grid::new("process_selection_grid").min_col_width(64f32).show(ui, |ui|{
                ui.radio_value(&mut self.display_mode, DisplayMode::Table, "Classic");
                ui.radio_value(&mut self.display_mode, DisplayMode::ComboBox, "ComboBox");
                ui.end_row();

            });
            match self.display_mode {
                DisplayMode::Table => {
                    self.create_process_table(ui);
                }
                DisplayMode::ComboBox => {
                    self.create_process_combo_box(ui);
                }
            }
        });
    }
}

impl MyApp {
    fn list_processes(&mut self, ui: &mut Ui) {
        for i in 0..self.processes.len() {
            let value = ui.selectable_value(
                &mut &self.processes[i].name,
                &self.processes[self.selected_process].name,
                format!("{} - {}", &self.processes[i].name, &self.processes[i].pid),
            );
            if (value.clicked()) {
                self.selected_process = i;
            }
        }
    }
    fn create_process_table(&mut self, ui: &mut Ui) {
        let text_style = egui::TextStyle::Body;
        let text_height = ui.text_style_height(&text_style);
        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("#");
                });
                header.col(|ui| {
                    ui.strong("Process Name");
                });
                header.col(|ui| {
                    ui.strong("PID");
                });
            })
            .body(|mut body| {
                body.rows(text_height+4f32, self.processes.len(), |mut row| {
                    let row_index = row.index();
                    row.col(|ui| {
                        ui.label(RichText::new(row_index.to_string()).strong());
                    });

                    let current_process = self.processes.get(row_index);
                    if let Some(mut current_process) = self.processes.get(row_index) {
                        row.col(|ui| {
                            let value = ui.selectable_label(
                                self.selected_process == row_index,
                                format!("{}", current_process.name),
                            );
                            if (value.clicked()) {
                                self.selected_process = row_index;
                            }
                        });
                        row.col(|ui| {
                            ui.label(current_process.pid.to_string());
                        });
                    }
                });
            });
    }

    fn create_process_combo_box(&mut self, ui: &mut Ui) {
            ui.label("Processes");
            egui::ComboBox::new("Processes", "")
                .selected_text(format!(
                    "{} - {}",
                    &self.processes[self.selected_process].name,
                    &self.processes[self.selected_process].pid
                ))
                .width(250f32)
                .show_ui(ui, |ui| {
                    self.list_processes(ui);
                });

            ui.end_row();
    }
}
fn sort_processes_by_name_pid(mut process_list: Vec<ProcessInfo>) -> Vec<ProcessInfo> {
    process_list.sort_by(|a, b| {
        if (a.name == b.name) {
            a.pid.partial_cmp(&b.pid).unwrap()
        } else {
            a.name.partial_cmp(&b.name).unwrap()
        }
    });
    return process_list;
}

fn get_all_processes() -> Vec<ProcessInfo> {
    let system = System::new_all();
    let mut guy = CPUsageCalculationValues {
        old_process_sys_cpu: 123,
        old_process_user_cpu: 133,
        old_system_sys_cpu: 133,
        old_system_user_cpu: 133,
    };
    let unsorted_list = system
        .processes()
        .values()
        .map(|p| ProcessInfo {
            name: p.name().to_string(),
            cmd: p.cmd().to_owned(),
            pid: p.pid(),
            user_id: p.user_id().to_owned().cloned(),
            environ: p.environ().to_vec(),
            memory: p.memory(),
            virtual_memory: p.virtual_memory(),
            parent: p.parent(),
            status: p.status(),
            start_time: p.start_time(),
            run_time: p.run_time(),
            cpu_usage: p.cpu_usage(),
            updated: false,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
        })
        .collect();
    sort_processes_by_name_pid(unsorted_list)
}
