use yew::prelude::*;

use crate::components::post::Post;

pub struct Stream {}

pub enum Msg {}

impl Component for Stream {
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
          <ul id="stream">
            <Post />
            <Post />
            <Post />
            <Post />
          </ul>
        }
    }
}
