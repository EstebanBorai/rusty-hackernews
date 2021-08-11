use anyhow::Error;
use common::LinkPreview;
use js_sys::encode_uri_component;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::RequestMode;

#[cfg(not(debug_assertions))]
const PREVIEWS_V1_ENDPOINT: &str = "https://fluxcap.herokuapp.com/api/v1/previews";

#[cfg(debug_assertions)]
const PREVIEWS_V1_ENDPOINT: &str = "http://0.0.0.0:3000/api/v1/previews";

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

pub enum Msg {
    FetchPreviews,
    FetchSucced(LinkPreview),
    FetchFailed(String),
}

pub struct Story {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    previews: Option<LinkPreview>,
    is_loading: bool,
    error_message: Option<String>,
}

impl Story {
    fn render_read_more(&self) -> Html {
        if let Some(url) = self.props.url.clone() {
            return html! {
                <a href={url} class="story-field read-more-button action-button" target="_blank">
                    <figure>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-box-arrow-up-right" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5z"/>
                            <path fill-rule="evenodd" d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0v-5z"/>
                        </svg>
                    </figure>
                    {"Read More"}
                </a>
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

    fn render_description(&self) -> Html {
        if self.is_loading {
            return html! {
              <p>{"Loading Description"}</p>
            };
        }

        if let Some(preview) = self.previews.clone() {
            if let Some(description) = preview.description {
                if description.is_empty() {
                    return html! {
                      <p class="empty-description-label">{"No description available"}</p>
                    };
                }

                if let Some(image_url) = preview.image_url {
                    return html! {
                        <p>
                            { self.render_image(image_url.clone()) }
                            {description}
                        </p>
                    };
                }

                return html! {
                  <p>{description}</p>
                };
            }
        }

        Html::default()
    }

    fn render_image(&self, image_url: String) -> Html {
        html! {
          <figure>
            <img src=image_url alt=format!("{} story image", self.props.title) />
          </figure>
        }
    }
}

impl Component for Story {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            fetch_task: None,
            previews: None,
            is_loading: true,
            error_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchPreviews => {
                // Even when "is_loading" is set to true on "create"
                // we want to make sure is settled again when fetching
                // for previews. This way we don't have to worry about
                // retries behaviors for instance
                self.is_loading = true;
                self.error_message = None;

                // TODO: Replace "js_sys" approach with a Rust-Native approach
                if let Some(story_url) = self.props.url.clone() {
                    #[allow(unused_unsafe)]
                    let url_query = unsafe { encode_uri_component(story_url.as_str()) };
                    let url_query = String::from(url_query);
                    let mut final_url = String::from(PREVIEWS_V1_ENDPOINT);

                    final_url.push_str("?url=");
                    final_url.push_str(url_query.as_str());

                    let request = Request::get(final_url).body(Nothing).unwrap();
                    let callback =
                        self.link
                            .callback(|res: Response<Json<Result<LinkPreview, Error>>>| {
                                let Json(data) = res.into_body();

                                match data {
                                    Ok(previews) => Msg::FetchSucced(previews),
                                    Err(err) => Msg::FetchFailed(err.to_string()),
                                }
                            });

                    let mut options = FetchOptions::default();

                    options.mode = Some(RequestMode::Cors);

                    let task =
                        FetchService::fetch_with_options(request, options, callback).unwrap();

                    self.fetch_task = Some(task);
                }
            }
            Msg::FetchFailed(error_message) => {
                self.is_loading = false;
                self.error_message = Some(error_message);
            }
            Msg::FetchSucced(previews) => {
                self.is_loading = false;
                self.error_message = None;
                self.previews = Some(previews);
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(Msg::FetchPreviews);
        }
    }

    fn view(&self) -> Html {
        let title = self.props.title.clone();

        html! {
            <li class="story">
                <main>
                    <h2 class="story-title">{title}</h2>
                    <article>
                        { self.render_description() }
                    </article>
                </main>
                <footer>
                    <div class="story-meta">
                        { self.render_score() }
                        { self.render_author() }
                    </div>
                    { self.render_read_more() }
                </footer>
            </li>
        }
    }
}
