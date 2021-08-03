use yew::prelude::*;

pub struct Post {}

pub enum Msg {}

impl Component for Post {
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
          <li class="post">
            <main>
              <h2>{"Lorem Ipsum"}</h2>
            </main>
          </li>
        }
    }
}
