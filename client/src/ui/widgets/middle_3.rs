use super::*;

pub struct Middle3 {
    width: f32,
    side_align: Align,
}

impl Default for Middle3 {
    fn default() -> Self {
        Self {
            width: 150.0,
            side_align: Align::Center,
        }
    }
}

impl Middle3 {
    pub fn width(mut self, value: f32) -> Self {
        self.width = value;
        self
    }
    pub fn side_align(mut self, align: Align) -> Self {
        self.side_align = align;
        self
    }
    pub fn ui_mut(
        self,
        ui: &mut Ui,
        world: &mut World,
        center: impl FnOnce(&mut Ui, &mut World),
        left: impl FnOnce(&mut Ui, &mut World),
        right: impl FnOnce(&mut Ui, &mut World),
    ) {
        let mut full_rect = Rect::from_min_size(
            ui.cursor().left_top(),
            egui::vec2(ui.available_width(), 0.0),
        );
        let side_width = (full_rect.width() - self.width) * 0.5;
        let rect_center = Rect::from_min_max(
            full_rect.center_top() - egui::vec2(self.width * 0.5, 0.0),
            pos2(
                full_rect.center_top().x + self.width * 0.5,
                full_rect.height(),
            ),
        );

        let ui_center = &mut ui.new_child(
            UiBuilder::new()
                .max_rect(rect_center.shrink(8.0))
                .layout(Layout::top_down_justified(Align::Center)),
        );
        center(ui_center, world);
        let height = ui_center.cursor().top() - ui.cursor().top();

        let rect_left = Rect::from_min_size(full_rect.min, egui::vec2(side_width, height));
        let rect_right =
            Rect::from_min_size(rect_center.right_top(), egui::vec2(side_width, height));
        left(
            &mut ui.new_child(
                UiBuilder::new()
                    .max_rect(rect_left)
                    .layout(Layout::right_to_left(Align::Center)),
            ),
            world,
        );
        right(
            &mut ui.new_child(
                UiBuilder::new()
                    .max_rect(rect_right)
                    .layout(Layout::left_to_right(Align::Center)),
            ),
            world,
        );
        full_rect.set_height(full_rect.height().max(height));
        ui.advance_cursor_after_rect(full_rect);
    }
    pub fn ui(
        self,
        ui: &mut Ui,
        world: &World,
        center: impl FnOnce(&mut Ui, &World),
        left: impl FnOnce(&mut Ui, &World),
        right: impl FnOnce(&mut Ui, &World),
    ) {
        let mut full_rect = Rect::from_min_size(
            ui.cursor().left_top(),
            egui::vec2(ui.available_width(), 0.0),
        );
        let side_width = (full_rect.width() - self.width) * 0.5;
        let rect_center = Rect::from_min_max(
            full_rect.center_top() - egui::vec2(self.width * 0.5, 0.0),
            pos2(
                full_rect.center_top().x + self.width * 0.5,
                full_rect.height(),
            ),
        );
        let ui_center = &mut ui.new_child(
            UiBuilder::new()
                .max_rect(rect_center.shrink(8.0))
                .layout(Layout::top_down_justified(Align::Center)),
        );
        center(ui_center, world);
        let height = ui_center.cursor().top() - ui.cursor().top();

        let rect_left = Rect::from_min_size(full_rect.min, egui::vec2(side_width, height));
        let rect_right =
            Rect::from_min_size(rect_center.right_top(), egui::vec2(side_width, height));
        left(
            &mut ui.new_child(
                UiBuilder::new()
                    .max_rect(rect_left)
                    .layout(Layout::right_to_left(Align::Center)),
            ),
            world,
        );
        right(
            &mut ui.new_child(
                UiBuilder::new()
                    .max_rect(rect_right)
                    .layout(Layout::left_to_right(Align::Center)),
            ),
            world,
        );
        full_rect.set_height(full_rect.height().max(height));
        ui.advance_cursor_after_rect(full_rect);
    }
}
