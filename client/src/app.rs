use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::modules::home;
use crate::modules::story;

use super::router::AppRoute;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            <Header />
            <main id="app-main">
                <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html!{<home::Index />},
                            AppRoute::Story(id) => html!{<story::Index id=id />}
                        }
                    })
                />
            </main>
          </>
        }
    }
}
