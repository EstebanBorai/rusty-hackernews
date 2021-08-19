mod components;

use yew::prelude::*;

use self::components::Stream;

pub struct Index {}

pub enum Msg {}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Index {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <Stream />
        }
    }
}
