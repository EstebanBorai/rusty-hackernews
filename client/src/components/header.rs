use yew::prelude::*;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <header id="app-header">
            <div id="header-wrapper">
                <h1>{"FluxCap"}</h1>
                <small>{"Powered by HackerNews and Firebase"}</small>
            </div>
          </header>
        }
    }
}
