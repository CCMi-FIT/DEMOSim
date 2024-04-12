pub struct Product {
    pub name: String,
    pub children: Vec<Product>,
}

pub struct ProductView {
    pos: egui::Pos2,
}