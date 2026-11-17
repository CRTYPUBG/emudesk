//! EmuDesk main egui application: navigation, connection form, and screens.

use std::path::PathBuf;

use eframe::egui;

use crate::github;
use crate::help;
use crate::profile::{Profile, ProfileStore, Resolution};
use crate::rdp;
use crate::theme::{self, ACCENT_BLUE, ACCENT_CYAN, TEXT_MUTED};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Devices,
    Settings,
    Help,
}

pub struct EmuDeskApp {
    screen: Screen,

    target: String,
    port: u16,
    username: String,
    desktop_size: String,
    resolution: Resolution,
    multi_monitor: bool,
    fullscreen: bool,
    profile_name: String,

    status: String,
    status_error: bool,

    profiles: Vec<Profile>,
    store: ProfileStore,

    identity: String,
    server_address: String,
}

impl EmuDeskApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_style(theme::style());

        let data_root = emudesk_core::config::default_data_dir();
        let identity = load_identity(&data_root.join("identity.json"));
        let store = ProfileStore::new(data_root.join("profiles.json")).unwrap_or_else(|_| {
            ProfileStore::new(PathBuf::from("emudesk_profiles.json")).expect("profile store")
        });
        let profiles = store.load_all().unwrap_or_default();

        Self {
            screen: Screen::Home,
            target: String::new(),
            port: 3389,
            username: String::new(),
            desktop_size: "1280x800".to_string(),
            resolution: Resolution::Windowed,
            multi_monitor: false,
            fullscreen: false,
            profile_name: String::new(),
            status: String::new(),
            status_error: false,
            profiles,
            store,
            identity,
            server_address: String::new(),
        }
    }

    fn form_profile(&self) -> Profile {
        Profile {
            name: self.profile_name.clone(),
            target: self.target.clone(),
            port: self.port,
            username: if self.username.trim().is_empty() {
                None
            } else {
                Some(self.username.trim().to_string())
            },
            desktop_size: self.desktop_size.clone(),
            resolution: self.resolution,
            multi_monitor: self.multi_monitor,
            fullscreen: self.fullscreen,
        }
    }

    fn set_status(&mut self, msg: impl Into<String>) {
        self.status = msg.into();
        self.status_error = false;
    }

    fn set_error(&mut self, msg: impl Into<String>) {
        self.status = msg.into();
        self.status_error = true;
    }

    fn do_connect(&mut self) {
        let profile = self.form_profile();
        if let Err(e) = profile.validate() {
            self.set_error(e);
            return;
        }
        match rdp::launch(&profile) {
            Ok(()) => self.set_status(format!(
                "Bağlantı başlatıldı: {}",
                rdp::host_arg(&profile.target, profile.port)
            )),
            Err(e) => self.set_error(e.to_string()),
        }
    }

    fn do_save_profile(&mut self) {
        let profile = self.form_profile();
        if let Err(e) = profile.validate() {
            self.set_error(e);
            return;
        }
        match self.store.upsert(&mut self.profiles, profile) {
            Ok(()) => self.set_status("Profil kaydedildi."),
            Err(e) => self.set_error(format!("Profil kaydedilemedi: {e}")),
        }
    }

    fn delete_profile(&mut self, name: &str) {
        let name = name.to_string();
        match self.store.remove(&mut self.profiles, &name) {
            Ok(()) => self.set_status(format!("Profil silindi: {name}")),
            Err(e) => self.set_error(format!("Profil silinemedi: {e}")),
        }
    }

    fn load_profile_into_form(&mut self, name: &str) {
        if let Some(p) = self.profiles.iter().find(|p| p.name == name).cloned() {
            self.profile_name = p.name;
            self.target = p.target;
            self.port = p.port;
            self.username = p.username.unwrap_or_default();
            self.desktop_size = p.desktop_size;
            self.resolution = p.resolution;
            self.multi_monitor = p.multi_monitor;
            self.fullscreen = p.fullscreen;
            self.screen = Screen::Home;
            self.set_status(format!("Profil yüklendi: {}", name));
        }
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading("EmuDesk");
        ui.add_space(8.0);

        let nav = [
            (Screen::Home, "Bağlan"),
            (Screen::Devices, "Kayıtlı Cihazlar"),
            (Screen::Settings, "Ayarlar"),
            (Screen::Help, "Yardım"),
        ];
        for (screen, label) in nav {
            let selected = self.screen == screen;
            if ui.selectable_label(selected, label).clicked() {
                self.screen = screen;
            }
        }
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(egui::RichText::new("v0.1.0").small().color(TEXT_MUTED));
        });
    }

    fn status_line(&mut self, ui: &mut egui::Ui) {
        if self.status.is_empty() {
            return;
        }
        let color = if self.status_error {
            egui::Color32::from_rgb(0xe0, 0x5b, 0x5b)
        } else {
            ACCENT_BLUE
        };
        ui.colored_label(color, &self.status);
    }

    fn home_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Uzak bilgisayara bağlan").color(ACCENT_CYAN));
        ui.add_space(6.0);

        ui.label("Hedef (IP adresi, bilgisayar adı veya EmuDesk ID)");
        ui.text_edit_singleline(&mut self.target);

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.add(
                egui::DragValue::new(&mut self.port)
                    .speed(1)
                    .range(1..=65535),
            );
            ui.add_space(16.0);
            ui.label("Kullanıcı adı (isteğe bağlı):");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Çözünürlük:");
            egui::ComboBox::from_id_salt("resolution_combo")
                .selected_text(self.resolution.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.resolution, Resolution::Windowed, "Pencere");
                    ui.selectable_value(&mut self.resolution, Resolution::Fullscreen, "Tam ekran");
                    ui.selectable_value(
                        &mut self.resolution,
                        Resolution::MultiMonitor,
                        "Çoklu monitör",
                    );
                });
            ui.add_space(8.0);
            ui.label("Masaüstü boyutu:");
            ui.text_edit_singleline(&mut self.desktop_size);
        });

        ui.add_space(4.0);
        ui.checkbox(&mut self.multi_monitor, "Çoklu monitör");
        ui.checkbox(&mut self.fullscreen, "Tam ekran");

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Profil adı:");
            ui.text_edit_singleline(&mut self.profile_name);
        });

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .add_sized([160.0, 34.0], egui::Button::new("Bağlan"))
                .clicked()
            {
                self.do_connect();
            }
            if ui
                .add_sized([160.0, 34.0], egui::Button::new("Kaydet"))
                .clicked()
            {
                self.do_save_profile();
            }
        });

        ui.add_space(8.0);
        self.status_line(ui);
    }

    fn devices_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Kayıtlı Cihazlar").color(ACCENT_CYAN));
        ui.add_space(6.0);

        if self.profiles.is_empty() {
            ui.label("Henüz kayıtlı cihaz yok. Bağlan sayfasında bir profil adı girip 'Kaydet' kullanın.");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut to_delete: Option<String> = None;
            for p in self.profiles.clone() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}  •  {}:{}", p.name, p.target, p.port));
                    if ui.button("Yükle").clicked() {
                        let name = p.name.clone();
                        self.load_profile_into_form(&name);
                    }
                    if ui.button("Bağlan").clicked() {
                        match rdp::launch(&p) {
                            Ok(()) => self.set_status(format!("Bağlantı başlatıldı: {}", p.target)),
                            Err(e) => self.set_error(e.to_string()),
                        }
                    }
                    if ui.button("Sil").clicked() {
                        to_delete = Some(p.name.clone());
                    }
                });
            }
            if let Some(name) = to_delete {
                self.delete_profile(&name);
            }
        });

        ui.add_space(6.0);
        self.status_line(ui);
    }

    fn settings_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Ayarlar").color(ACCENT_CYAN));
        ui.add_space(8.0);

        ui.label("Bu cihazın EmuDesk ID'si:");
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(&self.identity).strong().monospace());
        });
        ui.add_space(4.0);
        ui.colored_label(
            TEXT_MUTED,
            "Bu ID tek başına erişim hakkı vermez; yalnızca bir randevu adresidir. Kimlik doğrulama ayrı eşleştirme ve oturum anahtarlarıyla yapılır.",
        );

        ui.add_space(12.0);
        ui.label("Koordinasyon sunucusu adresi (isteğe bağlı):");
        ui.text_edit_singleline(&mut self.server_address);
        ui.colored_label(
            TEXT_MUTED,
            "Henüz bağlı değil — bu alan şimdilik yalnızca gösterim içindir.",
        );

        ui.add_space(12.0);
        ui.separator();
        ui.label(format!("RDP motoru: {}", rdp::describe_backend()));

        ui.add_space(8.0);
        if ui.button("GitHub'da güncelleme kontrolü yap").clicked() {
            self.check_github_update();
        }

        ui.add_space(12.0);
        self.status_line(ui);
    }

    fn check_github_update(&mut self) {
        let current = env!("CARGO_PKG_VERSION");
        let platform = current_platform();
        match github::UpdateChecker::emudesk().check_for_update(current, platform) {
            Ok(Some(u)) => {
                // The stable checker never offers prereleases; keep the guard
                // defensive and surface the release metadata we received.
                if u.is_prerelease {
                    self.set_status("Yalnızca kararlı sürümler önerilir.");
                    return;
                }
                let mut msg = format!("Yeni sürüm mevcut: {}", u.latest_version);
                if !u.published_at.trim().is_empty() {
                    msg.push_str(&format!("  ({})", u.published_at.trim()));
                }
                if let Some(notes) = u.body.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
                    msg.push_str(&format!("  — {notes}"));
                }
                if let Some(url) = u.asset_download_url.as_deref() {
                    msg.push_str(&format!("  İndir: {url}"));
                }
                self.set_status(msg);
            }
            Ok(None) => self.set_status("Güncel sürüm kullanıyorsunuz."),
            Err(e) => self.set_error(format!("Güncelleme kontrolü başarısız: {e}")),
        }
    }

    fn help_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new(help::HELP_TITLE).color(ACCENT_CYAN));
        ui.add_space(6.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in help::HELP_LINES {
                ui.label(*line);
            }
        });
    }
}

impl eframe::App for EmuDeskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nav")
            .resizable(false)
            .exact_width(170.0)
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Home => self.home_screen(ui),
            Screen::Devices => self.devices_screen(ui),
            Screen::Settings => self.settings_screen(ui),
            Screen::Help => self.help_screen(ui),
        });
    }
}

/// Load the local device identity; falls back to a friendly placeholder.
fn load_identity(identity_file: &PathBuf) -> String {
    let store = match emudesk_core::identity::IdentityStore::new(identity_file) {
        Ok(s) => s,
        Err(_) => return "belirsiz".to_string(),
    };
    match store.load_or_create() {
        Ok(id) => id.id,
        Err(_) => "belirsiz".to_string(),
    }
}

/// Active platform used to select a GitHub release asset.
#[cfg(windows)]
fn current_platform() -> github::Platform {
    github::Platform::WindowsX64
}

/// Active platform used to select a GitHub release asset.
#[cfg(target_os = "linux")]
fn current_platform() -> github::Platform {
    github::Platform::LinuxX64
}
