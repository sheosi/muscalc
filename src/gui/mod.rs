mod entry_page;
mod header;
mod info_button;
mod results_page;

use crate::calcs;

use relm::Widget;
use gtk::{Inhibit, prelude::*, Window, WindowType};
use relm::{connect, Component, init, Relm, Update};
use relm_derive::Msg;

use Msg::*;

pub struct Model {
}

#[derive(Msg)]
pub enum Msg {
    ShowResults,
    Quit,
    Back
}


pub struct Win {
    model: Model,
    window: Window,
    header: Component<header::Wdg>,
    entry_page: Component<entry_page::Wdg>,
    results_page: Component<results_page::Wdg>,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            ShowResults => {
                self.results_page.widget().show();
                self.entry_page.widget().hide();
            }
            Back => {
                self.entry_page.widget().show();
                self.results_page.widget().hide();
            }
        }
    }
}

impl Widget for Win {
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);
        window.style_context().add_class("mainwin");
        
        let header = init::<header::Wdg>(()).expect("Header");
        window.set_titlebar(Some(header.widget()));

        let grid = gtk::Grid::new();
        let entry_page_wdg = init::<entry_page::Wdg>(()).expect("Entry Page");
        grid.add(entry_page_wdg.widget());

        let results_page = init::<results_page::Wdg>(()).expect("Results Page");
        results_page.widget().set_visible(false);
        results_page.widget().set_no_show_all(true);
        grid.add(results_page.widget());

        window.add(&grid);
        

        /*use entry_page::Msg::ShowResults as EntryPageShowResults;
        use header::Msg::{EnableBack as EnableBack, OnBack as HeaderBack};*/

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        /*connect!(relm, entry_page_wdg, EntryPageShowResults(_) , Msg::ShowResults);
        connect!(relm, header, EnableBack(_), Msg::Back);
        connect!(relm, header, HeaderBack(_) , Msg::Back);*/
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window,
            header,
            entry_page: entry_page_wdg,
            results_page
        }
    }
}