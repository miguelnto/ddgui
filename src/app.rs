use egui_file_dialog::FileDialog;
use egui_notify::Toasts;
use std::path::PathBuf;
use std::process::Child;
use lsblk::blockdevs::BlockDevice;
use eframe::egui;
use egui_dropdown::DropDownBox;
use crate::cmds::dd;
use std::time::Duration;
use egui::widgets::Spinner;

pub struct MyApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
    items: Vec<String>,
    device_name: String,
    loading: bool,
    child: Option<Child>,
    toasts: Toasts,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let mut itms: Vec<String> = Vec::new();
        for blk in BlockDevice::list().unwrap() {
            itms.push(blk.fullname.display().to_string());
        }
        Self {
            file_dialog: FileDialog::new(),
            selected_file: None,
            items: itms,
            device_name: String::new(),
            loading: false,
            child: None,
            toasts: Toasts::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                ui.label("Choose an ISO: ");
                ui.label(
                    match &self.selected_file {
                        Some(s) => format!("{}",s.to_str().unwrap()),
                        _ => "No file selected.".to_string()
                    }
                );
                if ui.button("Select file").clicked() {
                    // Open the file dialog to select a file.
                    self.file_dialog.select_file();
                }

            });
            ui.horizontal(|ui| {
                ui.label("Select a device: ");
                ui.add(
                    DropDownBox::from_iter(
                        &self.items,
                        "test_dropbox",
                        &mut self.device_name,
                        |ui, text| ui.selectable_label(false, text),
                    )
                    // choose whether to filter the box items based on what is in the text edit already
                    // default is true when this is not used
                    .filter_by_input(true)
                    // choose whether to select all text in the text edit when it gets focused
                    // default is false when this is not used
                    .select_on_focus(true)
                    // passes through the desired width to the text edit
                    // default is None internally, so TextEdit does whatever its default implements
                    .desired_width(250.0),
                );
                if ui.button("Refresh").clicked() {
                    let mut itms: Vec<String> = Vec::new();
                    for blk in BlockDevice::list().unwrap() {
                        itms.push(blk.fullname.display().to_string());
                    }
                    self.items = itms;
                }
            });

            if !self.loading {
                if ui.button("Start").clicked() {
                    let iso_file = match &self.selected_file {
                        Some(s) => s.to_str().unwrap(),
                        _ => "",
                    };
                    if !iso_file.is_empty() && !self.device_name.is_empty() {
                        self.loading = true;
                        self.child = Some(dd(&self.device_name, iso_file));
                    } else {
                        self.toasts.warning("Please select a device and an ISO file").duration(Some(Duration::from_secs(4)));
                    }
                }
            } else {
                ui.add(Spinner::new());
            }
                
            if let Some(ch) = self.child.as_mut() {
                if let Some(status) = ch.try_wait().unwrap() {
                    self.loading = false;
                    if status.success() {
                        ui.label(format!("Process finished successfully."));
                    } else {
                        self.child = None;
                        self.toasts.warning("Insufficient permissions. Please run this program as root.").duration(Some(Duration::from_secs(4)));
                    }
                }
            }

            self.file_dialog.update(ctx);
            self.toasts.show(ctx);

            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());
            }
        });
    }
}

