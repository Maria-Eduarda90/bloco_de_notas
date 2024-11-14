use gtk::prelude::*;
use gtk::{TextView, TextBuffer, Window, WindowType, Button, FileChooserDialog, FileChooserAction, FileFilter};

fn main() {
    gtk::init().expect("Falha ao iniciar o GTK");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Bloco de notas");
    window.set_default_size(800, 600);

    window.set_resizable(true);
    window.set_decorated(true);
    
    let text_view = TextView::new();
    let buffer = text_view.get_buffer().unwrap();
    text_view.set_wrap_mode(gtk::WrapMode::Word);

    let save_button = Button::with_label("Salvar");
    

    save_button.connect_clicked({
        let window = window.clone();
        move |_| {
            let dialog = FileChooserDialog::new(
                Some("Salvar Arquivo"),
                Some(&window),
                FileChooserAction::Save,
            );

            let filter = FileFilter::new();
            filter.add_mime_type("text/plain");
            dialog.add_filter(&filter);

            dialog.add_button("Cancelar", gtk::ResponseType::Cancel);
            dialog.add_button("Salvar", gtk::ResponseType::Accept);

            if dialog.run() == gtk::ResponseType::Accept {
                if let Some(filename) = dialog.get_filename() {
                    if let Some(content) = buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), true) {
                        let content_str = content.to_string();
                        std::fs::write(filename, content_str).expect("Erro ao salvar arquivo.");
                    }
                }
            }

            dialog.close();
        }
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&text_view, true, true, 0);
    vbox.pack_start(&save_button, false, false, 0);

    window.add(&vbox);

    window.show_all();

    gtk::main();
}
