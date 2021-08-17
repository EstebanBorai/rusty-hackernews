use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::pages::home::Home;
use crate::pages::story::Story;

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
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Story(id) => html!{<Story id=id />}
                        }
                    })
                />
            </main>
          </>
        }
    }
}
