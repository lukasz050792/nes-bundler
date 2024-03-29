use egui::{
    epaint::ImageDelta, Color32, ColorImage, Context, ImageData, Order, TextureHandle,
    TextureOptions, Image,
};

use crate::{
    input::{gamepad::GamepadEvent, KeyEvent},
    HEIGHT, WIDTH,
};

use super::Settings;
pub trait ToGuiEvent {
    /// Convert the struct to a GuiEvent
    fn to_gui_event(&self) -> Option<GuiEvent>;
}

#[derive(Clone, Debug)]
pub enum GuiEvent {
    Keyboard(KeyEvent),
    Gamepad(GamepadEvent),
}
pub trait GuiComponent {
    fn ui(&mut self, ctx: &Context, ui_visible: bool, name: String, settings: &mut Settings);
    fn event(&mut self, event: &GuiEvent, settings: &mut Settings);
    fn name(&self) -> Option<String>;
    fn open(&mut self) -> &mut bool;
}

pub struct Gui {
    visible: bool,
    egui_glow: egui_glow::EguiGlow,
    nes_texture: TextureHandle,
    nes_texture_options: TextureOptions,
    no_image: ImageData,
}

impl Gui {
    pub fn new(egui_glow: egui_glow::EguiGlow) -> Self {
        let nes_texture_options = TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
        };

        Self {
            visible: true,
            nes_texture: egui_glow.egui_ctx.load_texture(
                "nes",
                ImageData::Color(ColorImage::new(
                    [WIDTH as usize, HEIGHT as usize],
                    Color32::BLACK,
                ).into()),
                nes_texture_options,
            ),
            egui_glow,
            nes_texture_options,
            no_image: ImageData::Color(ColorImage::new([0, 0], Color32::TRANSPARENT).into()),
        }
    }

    pub fn handle_events(
        &mut self,
        event: &GuiEvent,
        guis: &mut [Option<&mut dyn GuiComponent>],
        settings: &mut Settings,
    ) {
        for gui in guis.iter_mut().flatten() {
            gui.event(event, settings);
        }
    }

    pub fn ui(
        &mut self,
        window: &winit::window::Window,
        guis: &mut [Option<&mut dyn GuiComponent>],
        settings: &mut Settings,
    ) {
        self.egui_glow.run(window, |ctx| {
            egui::Area::new("game_area")
                .fixed_pos([0.0, 0.0])
                .order(Order::Background)
                .show(ctx, |ui| {
                    let texture_handle = &self.nes_texture;
                    if let Some(t) = ctx.tex_manager().read().meta(texture_handle.id()) {
                        if t.size[0] != 0 {
                            ui.centered_and_justified(|ui| {
                                ui.add(Image::new(texture_handle).shrink_to_fit());
                            });
                        }
                    }
                });
            egui::Area::new("window_area")
                .fixed_pos([0.0, 0.0])
                .show(ctx, |ui| {
                    if self.visible {
                        egui::TopBottomPanel::top("menubar_container").show_inside(ui, |ui| {
                            egui::menu::bar(ui, |ui| {
                                ui.menu_button("Settings", |ui| {
                                    for gui in guis.iter_mut().flatten() {
                                        if let Some(name) = gui.name() {
                                            if ui.button(name).clicked() {
                                                *gui.open() = !*gui.open();
                                                ui.close_menu();
                                            };
                                        }
                                    }
                                })
                            });
                        });
                    }
                    for gui in guis.iter_mut().flatten() {
                        if let Some(name) = gui.name() {
                            gui.ui(ctx, self.visible, name, settings);
                        }
                    }
                });
        });
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub(crate) fn update_nes_texture(&self, new_image_data: Option<ImageData>) {
        self.egui_glow.egui_ctx.tex_manager().write().set(
            self.nes_texture.id(),
            ImageDelta::full(
                new_image_data.unwrap_or_else(|| self.no_image.clone()),
                self.nes_texture_options,
            ),
        );
    }

    pub(crate) fn on_event(&mut self, event: &winit::event::WindowEvent<'_>) -> bool {
        self.egui_glow.on_event(event).consumed
    }
    pub fn destroy(&mut self) {
        self.egui_glow.destroy();
    }

    pub fn paint(&mut self, window: &winit::window::Window) {
        self.egui_glow.paint(window);
    }
}
