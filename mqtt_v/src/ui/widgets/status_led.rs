use eframe::egui::{Response, Sense, Ui, Widget};
use eframe::emath::{lerp, Vec2};
use eframe::epaint::{Color32, Rgba, Stroke};

pub struct StatusLed {
    value: bool,
    size: f32,
    animated: bool,
}

impl StatusLed {
    pub fn new(value: &bool) -> Self {
        Self {
            value: *value,
            size: 20.0,
            animated: true,
        }
    }
}

impl Widget for StatusLed {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let end = if self.value { 1.0 } else { 0.0 };
            let v = if self.animated {
                ui.ctx().animate_value_with_time(response.id, end, 0.2)
            } else {
                end
            };

            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::TRANSPARENT,
                Stroke::none(),
            );

            ui.painter().circle(
                rect.center(),
                5.0,
                Color32::from(lerp(
                    Rgba::from(Color32::BLACK)..=Rgba::from(Color32::GREEN),
                    v,
                )),
                Stroke::none(),
            );
        }

        response
    }
}
