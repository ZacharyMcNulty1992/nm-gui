extern crate gtk;

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{Relm, Update, Widget};

use provider;
use provider::{dbus_provider, interface};

struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    Decrement,
    Increment,
    Quit,
}

// Create the structure that holds the widgets used in the view.
struct Win {
    //counter_label: Label,
    model: Model,
    window: Window,
    bus: dbus_provider,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        return Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        //let label = &self.counter_label;

        match event {
            Msg::Decrement => {
                //self.model.counter -= 1;
                // Manually update the view.
                //label.set_text(&self.model.counter.to_string());
            },
            Msg::Increment => {
                //self.model.counter += 1;
                //label.set_text(&self.model.counter.to_string());
            },
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        //create tab like structure to house different operations.
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        let network_tab_button = Button::new_with_label("Network");
        button_box.add(&network_tab_button);
        vbox.add(&button_box);

        //add label with description of current page
        let desc_label = Label::new("This tab is for configuring network interfaces.");
        vbox.add(&desc_label);
        // TODO: get list of network interfaces via NM DBus.
        // TODO: create content box.

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        // Send the message Increment when the button is clicked.
        //connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
        //connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        let bus = provider::init_provider();
        Win {
            //counter_label: counter_label,
            bus:bus,
            model,
            window: window,
        }
    }
}

pub fn start(){
    Win::run(()).unwrap();
}