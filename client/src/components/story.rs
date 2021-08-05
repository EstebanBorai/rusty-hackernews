use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    #[prop_or(None)]
    pub image_url: Option<String>,
}

pub enum Msg {}

pub struct Story {
    props: Props,
}

impl Story {
    fn render_with_preview(&self) -> Html {
        let title = self.props.title.clone();
        let image_url = self.props.image_url.clone();
        let image_url = image_url.unwrap();

        html! {
          <li class="story">
            <main>
              <h2>{title}</h2>
              <img src={image_url} />
            </main>
          </li>
        }
    }

    fn render_without_preview(&self) -> Html {
        let title = self.props.title.clone();

        html! {
          <li class="story">
            <main>
              <h2>{title}</h2>
            </main>
          </li>
        }
    }
}

impl Component for Story {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.props.image_url.is_some() {
            return self.render_with_preview();
        }

        self.render_without_preview()
    }
}
