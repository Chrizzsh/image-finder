extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{
    Application, 
    ApplicationWindow,
    
    // Search bar
    Button, 
    Entry,
    
    // File view
    TreeView,
    TreeViewColumn,
    CellRendererText,
    ListStore,

    // Layout
    Box,
    Orientation,
};

mod finder;

fn main() {
    let application = Application::new(
        Some("image.finder"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("File finder");
        window.set_default_size(800, 600);

        // Top bar where path is entered
        let path_entry = Entry::new();
        let search_button = Button::with_label("Search");
    
        // Pack entry and button into one
        let search_box = Box::new(Orientation::Horizontal, 2);
        search_box.pack_start(&path_entry, true, true, 1);
        search_box.pack_end(&search_button, false, false, 1);
                
        // List views
        let tree = create_tree();
        let model = create_model();
        tree.set_model(Some(&model));
        
        // Add on clicks
        search_button.connect_clicked(move |_| {
            let entry_text = path_entry.get_text().as_str();
            let path = std::path::Path::new(entry_text);
    
            let file_result = finder::find_files(path);
            match file_result {
                Err(e) => eprintln!("{}", e),
                Ok(files) => {
                    model.clear();
                    for file in files {
                        model.insert_with_values(None, &[0, 1], &[&file.0, &file.1]);
                    }
                }
            }
        });
        
        // Layout stuff
        let v_box = Box::new(Orientation::Vertical, 2);
        v_box.pack_start(&search_box, false, false, 1);
        v_box.pack_end(&tree, true, true, 1);

        window.add(&v_box);
        window.show_all();
    });

    application.run(&[]);
}

fn create_tree() -> TreeView {
    let tree = TreeView::new();
    tree.set_headers_visible(true);

    append_column(&tree, 0, "Filename");
    append_column(&tree, 1, "Size");

    tree
}

fn append_column(tree: &TreeView, id: i32, name: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    cell.set_property_text(Some("Hallo"));

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", id);
    column.set_sort_column_id(id);
    column.set_title(name);
    tree.append_column(&column);
}

fn create_model() -> ListStore {
    let model = ListStore::new(&[String::static_type(), u32::static_type()]);

    model
}