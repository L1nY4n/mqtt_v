use eframe::egui::{self};

use crate::ui::widgets::docking;

use super::client::Client;



pub struct TreeView {
    title: String,  
}

impl TreeView {
 pub   fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
         
        }
    }
}

impl docking::Tab<Client> for TreeView {
    fn title(&self) -> &str {
        &self.title
    }

    fn ui(&mut self, ui: &mut egui::Ui, _client: &mut Client) {
        ui.push_id("tree_view", |_ui|{
   
        });
    }
}
