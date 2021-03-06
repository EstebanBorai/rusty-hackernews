use anyhow::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use common::hacker_news;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::RequestMode;
use yew_router::components::RouterAnchor;

use crate::components::raw_html::RawHtml;
use crate::constants::api;
use crate::router::AppRoute;
use crate::utils::url::make_link_preview_url;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct Comments {
    props: Props,
    link: ComponentLink<Self>,
    is_loading: bool,
    items: Option<Vec<common::hacker_news::Comment>>,
    fetch_task: Option<FetchTask>,
    error_message: Option<String>,
}

pub enum Msg {
    FetchStory,
    FetchSucced(Vec<common::hacker_news::Comment>),
    FetchFailed(String),
}

impl Comments {
    fn render_author(&self, author: String) -> Html {
        html! {
          <span class="story-field">
            <figure>
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-person" viewBox="0 0 16 16">
                <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z"/>
              </svg>
            </figure>
            {author}
          </span>
        }
    }

    fn render_time(&self, time: u64) -> Html {
        let naive = NaiveDateTime::from_timestamp(time as i64, 0);
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

    fn render_comments(&self) -> Html {
        if let Some(comments) = self.items.clone() {
            return html! {
                <ul id="comments">
                {
                    for comments.into_iter().map(|comment| {
                        html! {
                            <li class="comment" id=format!("#{}", comment.id)>
                                <a href=format!("#{}", comment.id)>
                                    <header class="comment-header">
                                        {
                                            if let Some(author) = comment.by {
                                                self.render_author(author)
                                            } else {
                                                Html::default()
                                            }
                                        }
                                        { self.render_time(comment.time) }
                                    </header>
                                </a>
                                {
                                    if comment.text.is_some() {
                                        html! {
                                            <main class="comment-body">
                                                <RawHtml inner_html=comment.text.clone().unwrap() />
                                            </main>
                                        }
                                    } else {
                                        Html::default()
                                    }
                                }

                            </li>
                        }
                    })
                }
                </ul>
            };
        }

        Html::default()
    }
}

impl Component for Comments {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Comments {
            props,
            link,
            is_loading: false,
            items: None,
            fetch_task: None,
            error_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchStory => {
                self.is_loading = true;
                self.error_message = None;

                let request = Request::get(format!("{}/{}/kids", api::v1::STORIES, self.props.id))
                    .body(Nothing)
                    .unwrap();
                let callback = self.link.callback(
                    |res: Response<Json<Result<Vec<common::hacker_news::Comment>, Error>>>| {
                        let Json(data) = res.into_body();

                        match data {
                            Ok(comments) => Msg::FetchSucced(comments),
                            Err(err) => Msg::FetchFailed(err.to_string()),
                        }
                    },
                );

                let mut options = FetchOptions::default();

                options.mode = Some(RequestMode::Cors);

                let task = FetchService::fetch_with_options(request, options, callback).unwrap();

                self.fetch_task = Some(task);
            }
            Msg::FetchSucced(comments) => {
                self.items = Some(comments);
                self.is_loading = false;
            }
            Msg::FetchFailed(error_message) => {
                self.error_message = Some(error_message);
                self.is_loading = false;
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(Msg::FetchStory);
        }
    }

    fn view(&self) -> Html {
        if self.is_loading {
            return html! {
                <h1>{"Fetching story"}</h1>
            };
        }

        if self.error_message.is_some() {
            let error_message = self.error_message.clone();
            let error_message = error_message.unwrap();

            return html! {
                <h1>{format!("Failed to fetch story with id: {}", self.props.id)}<br />{error_message}</h1>
            };
        }

        if self.items.is_some() {
            return self.render_comments();
        }

        Html::default()
    }
}
