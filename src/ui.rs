use crate::cube::*;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Resource)]
pub struct TimekeepingTimer(pub Instant);

pub fn game_ui(
    mut egui_context: EguiContexts,
    mut cube_settings: ResMut<CubeSettings>,
    mut timekeeping_timer: ResMut<TimekeepingTimer>,
    mut scramble_event: EventWriter<ScrambleEvent>,
    mut reset_event: EventWriter<ResetEvent>,
) {
    egui::Window::new("Game UI").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("ui_grid")
            .num_columns(2)
            .spacing([10.0, 20.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(egui::Label::new("Rotate Speed"));
                ui.add(egui::Slider::new(
                    &mut cube_settings.rotate_speed,
                    0.1..=10.0,
                ));
                ui.end_row();

                ui.add(egui::Label::new("Play Mode"));
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut cube_settings.play_mode,
                        PlayMode::Practice,
                        "Practice",
                    );
                    if ui
                        .selectable_value(
                            &mut cube_settings.play_mode,
                            PlayMode::Timekeeping,
                            "Timekeeping",
                        )
                        .clicked()
                    {
                        // 重置计时器
                        timekeeping_timer.0 = Instant::now();
                    }
                });
                if cube_settings.play_mode == PlayMode::Timekeeping {
                    ui.add(egui::Label::new(format!(
                        "{}s",
                        timekeeping_timer.0.elapsed().as_secs()
                    )));
                }
                ui.end_row();

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Scramble"))
                    .clicked()
                {
                    scramble_event.send_default();
                }

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Reset"))
                    .clicked()
                {
                    reset_event.send_default();
                }

                ui.end_row();
            });
    });
}
