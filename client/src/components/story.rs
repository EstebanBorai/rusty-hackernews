use anyhow::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use common::LinkPreview;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::RequestMode;
use yew_router::components::RouterAnchor;

use crate::router::AppRoute;
use crate::utils::url::make_link_preview_url;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u64,
    pub title: String,
    pub by: String,
    pub score: u32,
    pub time: u64,
    #[prop_or(None)]
    pub image_url: Option<String>,
    #[prop_or(None)]
    pub url: Option<String>,
    #[prop_or(None)]
    pub kids: Option<Vec<u64>>,
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
    fn render_comments(&self) -> Html {
        if let Some(kids) = self.props.kids.clone() {
            return html! {
                <span class="story-field action-button">
                    <RouterAnchor<AppRoute> route=AppRoute::Story(self.props.id) classes="router,active">
                        <figure>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 16">
                                <path d="M2.678 11.894a1 1 0 0 1 .287.801 10.97 10.97 0 0 1-.398 2c1.395-.323 2.247-.697 2.634-.893a1 1 0 0 1 .71-.074A8.06 8.06 0 0 0 8 14c3.996 0 7-2.807 7-6 0-3.192-3.004-6-7-6S1 4.808 1 8c0 1.468.617 2.83 1.678 3.894zm-.493 3.905a21.682 21.682 0 0 1-.713.129c-.2.032-.352-.176-.273-.362a9.68 9.68 0 0 0 .244-.637l.003-.01c.248-.72.45-1.548.524-2.319C.743 11.37 0 9.76 0 8c0-3.866 3.582-7 8-7s8 3.134 8 7-3.582 7-8 7a9.06 9.06 0 0 1-2.347-.306c-.52.263-1.639.742-3.468 1.105z"/>
                            </svg>
                        </figure>
                        {kids.len()}
                    </RouterAnchor<AppRoute>>
                </span>
            };
        }

        Html::default()
    }

    fn render_read_more(&self) -> Html {
        if let Some(url) = self.props.url.clone() {
            return html! {
                <span class="story-field action-button">
                    <a href={url} class="read-more-button" target="_blank">
                        <figure>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-box-arrow-up-right" viewBox="0 0 16 16">
                                <path fill-rule="evenodd" d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5z"/>
                                <path fill-rule="evenodd" d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0v-5z"/>
                            </svg>
                        </figure>
                        {"Read More"}
                    </a>
                </span>
            };
        }

        Html::default()
    }

    fn render_score(&self) -> Html {
        html! {
          <span class="story-field story-score">
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

    fn render_time(&self) -> Html {
        let naive = NaiveDateTime::from_timestamp(self.props.time as i64, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let newdate = datetime.format("%Y-%m-%d at %H:%M");
        let time = newdate.to_string();

        html! {
            <span class="story-field">
                <figure>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-clock" viewBox="0 0 16 16">
                        <path d="M8 3.5a.5.5 0 0 0-1 0V9a.5.5 0 0 0 .252.434l3.5 2a.5.5 0 0 0 .496-.868L8 8.71V3.5z"/>
                        <path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm7-8A7 7 0 1 1 1 8a7 7 0 0 1 14 0z"/>
                    </svg>
                </figure>
                {time.clone()}
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
                    let request = Request::get(make_link_preview_url(story_url))
                        .body(Nothing)
                        .unwrap();
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
                <header>
                    { self.render_author() }
                    { self.render_time() }
                </header>
                <main>
                    <h2 class="story-title">{title}</h2>
                    <article>
                        { self.render_description() }
                    </article>
                </main>
                <footer>
                    <div class="story-meta">
                        { self.render_score() }
                        { self.render_comments() }
                    </div>
                    { self.render_read_more() }
                </footer>
            </li>
        }
    }
}
