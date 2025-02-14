pub mod app;
pub mod parser;
pub mod output;
pub mod ref_command;
pub mod window;

use app::App;
use parser::Opti;
use zix_core::entry::create::{filter_entries, FilterOptions};

fn main() {
    if let Some(app) = App::init() {
        let mut items = app.entries;
        let ops = app.options;

        for dir_entries in items.iter_mut() {
            let mut filtered_entries = if ops.is_empty() {
                dir_entries.clone() // No hay opciones, no filtramos
            } else {
                filter_entries(dir_entries, &ops[0]) // Filtrar según la primera opción
            };

            let entry_names: Vec<String> = filtered_entries
                .iter()
                .map(|entry| entry.name.clone())
                .collect();

            if ops.contains(&Opti::Tree) {
                output::tree::base(&mut filtered_entries, ops.clone());
            } else if ops.contains(&Opti::List) {
                output::list::base(&mut filtered_entries, ops.clone());
            } else if ops.contains(&Opti::Grid) || ops.is_empty() {
                // Se usa Grid si se pasa --grid o si no hay opciones
                output::grid::base(entry_names, &filtered_entries);
            }
        }
    }
}
