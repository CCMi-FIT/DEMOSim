pub fn section_label_ui(ui: &mut egui::Ui, text: impl ToString) {
    ui.add_space(20.0);

    let available_width = ui.available_width();
    let height = 20.0;
    let style = ui.style();
    let fg_color = style.visuals.widgets.noninteractive.fg_stroke.color;
    let bg_color = style.visuals.code_bg_color;
    let rect = ui.allocate_space(egui::vec2(available_width, height)).1;

    ui.painter().rect_filled(rect, 0.0, bg_color);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::default(),
        fg_color,
    );
    ui.add_space(10.0);
}
