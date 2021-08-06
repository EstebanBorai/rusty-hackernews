use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub by: String,
    pub score: u32,
    #[prop_or(None)]
    pub image_url: Option<String>,
    #[prop_or(None)]
    pub url: Option<String>,
}

pub enum Msg {}

pub struct Story {
    props: Props,
}

impl Story {
    fn render_read_more(&self) -> Html {
        if let Some(url) = self.props.url.clone() {
            return html! {
              <a href={url}>{"Read More"}</a>
            };
        }

        Html::default()
    }

    fn render_score(&self) -> Html {
        html! {
          <span class="story-field">
            <figure>
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-arrow-up-circle" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8zm15 0A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-7.5 3.5a.5.5 0 0 1-1 0V5.707L5.354 7.854a.5.5 0 1 1-.708-.708l3-3a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1-.708.708L8.5 5.707V11.5z"/>
              </svg>
            </figure>
            {self.props.score}
          </span>
        }
    }

    fn render_author(&self) -> Html {
        html! {
          <span class="story-field">
            <figure>
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-person" viewBox="0 0 16 16">
                <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z"/>
              </svg>
            </figure>
            {self.props.by.clone()}
          </span>
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
        let title = self.props.title.clone();
        let author = self.props.by.clone();

        html! {
            <li class="story">
                <main>
                    <h2 class="story-title">{title}</h2>
                </main>
                <footer>
                    { self.render_score() }
                    { self.render_author() }
                    { self.render_read_more() }
                </footer>
            </li>
        }
    }
}
