mod entry_page;
mod header;
mod info_button;
mod results_page;

use relm::Widget;
use gtk::{Inhibit, prelude::*};
use relm::{Channel, connect, Component, init, Relm};
use relm_derive::{Msg, widget};

use Msg::*;

pub struct Model {
    header: Component<header::Wdg>,
    r: Relm<Win>,
    _channel_calc: Option<Channel<entry_page::CalcData>>
}

#[derive(Msg)]
pub enum Msg {
    ShowResults,
    Quit,
    Back
}

use entry_page::Wdg as EntryPage;
use results_page::Wdg as ResultsPage;
use entry_page::Msg::{ShowResults as EntryPageShowResults, ReceiveSender as EntryPageReceiveSender};
use results_page::Msg::Update as ResultsPageUpdateResults;
use header::Msg::{EnableBack as EnableBack, OnBack as HeaderBack};

#[widget]
impl Widget for Win {
    fn model(r: &Relm<Self>, _: ()) -> Model {
        let header = init::<header::Wdg>(()).expect("Header");        
        
        Model {
            header,
            r:r.clone(),
            _channel_calc: None
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            ShowResults => {
                self.widgets.results_page.show();
                self.widgets.entry_page.hide();
            }
            Back => {
                self.widgets.entry_page.show();
                self.widgets.results_page.hide();
            }
        }
    }

    view! {
        #[style_class="mainwin"]
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.header.widget()),

            gtk::Grid {
                #[name="entry_page"]
                EntryPage() {
                    EntryPageShowResults => ShowResults,
                },

                #[name="results_page"]
                ResultsPage() {
                    visible: false,
                    no_show_all: true
                }
            },

            delete_event(_, _)=> return (Some(Msg::Quit), Inhibit(false))
        }
    }

    fn init_view(&mut self) {
        let e = &self.components.entry_page;
        connect!(e@EntryPageShowResults, self.model.header, EnableBack);

        let stream = self.components.results_page.stream();
        let (c,s) = Channel::new(move |d|stream.emit(ResultsPageUpdateResults(d)));       
        self.components.entry_page.stream().emit(EntryPageReceiveSender(s));
        self.model._channel_calc = Some(c);

        let h = &self.model.header;
        connect!(h@HeaderBack, self.model.r , Msg::Back);
    }
}