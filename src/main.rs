use adw::prelude::*;
use adw::Application;

fn main() {
    let app = Application::builder()
        .application_id("com.example.AdwaitaApp")
        .build();

    app.connect_activate(|app| {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(400)
            .default_height(300)
            .build();

        let header_bar = adw::HeaderBar::builder().build();

        let title_label = gtk::Label::new(Some("Ninitarch"));
        header_bar.set_title_widget(Some(&title_label));

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let image = gtk::Image::from_file("assets/images/ninitarch.webp");
        image.set_width_request(128);
        image.set_height_request(128);
        vbox.append(&image);

        let heading = gtk::Label::new(Some(
            "NinitArch: Ninite for Arch Linux
",
        ));
        heading.add_css_class("heading");
        vbox.append(&heading);

        let paragraph1 = gtk::Label::new(Some(
            "Select the apps you want, NinitArch will create a script to install them all for you.",
        ));
        paragraph1.set_wrap(true);
        vbox.append(&paragraph1);

        let paragraph2 = gtk::Label::new(Some(
            "We assume that you have already installed an AUR helper, such as yay, paru or pamac.",
        ));
        paragraph2.set_wrap(true);
        vbox.append(&paragraph2);

        let paragraph3 = gtk::Label::new(Some("Created with ❤️ by Furkan Baytekin."));
        paragraph3.set_wrap(true);
        vbox.append(&paragraph3);

        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&header_bar);
        content.append(&vbox);

        window.set_content(Some(&content));

        window.set_visible(true);
    });

    app.run();
}
