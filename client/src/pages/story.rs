use anyhow::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use common::LinkPreview;
use js_sys::encode_uri_component;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::RequestMode;

use crate::constants::api;
use crate::utils::url::make_link_preview_url;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct Story {
    props: Props,
    link: ComponentLink<Self>,
    is_loading: bool,
    story: Option<common::Story>,
    link_preview: Option<common::LinkPreview>,
    fetch_task: Option<FetchTask>,
    error_message: Option<String>,
}

pub enum Msg {
    FetchStory,
    FetchSucced(common::Story),
    FetchFailed(String),
    FetchLinkPreview,
    FetchLinkPreviewSuccess(common::LinkPreview),
}

impl Story {
    fn render_story_image(&self) -> Html {
        if let Some(link_preview) = self.link_preview.clone() {
            if let Some(image_url) = link_preview.image_url {
                return html! {
                    <figure>
                        <img src={image_url} alt="Story image" />
                    </figure>
                };
            }
        }

        Html::default()
    }
}

impl Component for Story {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Story {
            props,
            link,
            is_loading: false,
            story: None,
            link_preview: None,
            fetch_task: None,
            error_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchStory => {
                self.is_loading = true;
                self.error_message = None;

                let request = Request::get(format!("{}/{}", api::v1::STORIES, self.props.id))
                    .body(Nothing)
                    .unwrap();
                let callback =
                    self.link
                        .callback(|res: Response<Json<Result<common::Story, Error>>>| {
                            let Json(data) = res.into_body();

                            match data {
                                Ok(story) => Msg::FetchSucced(story),
                                Err(err) => Msg::FetchFailed(err.to_string()),
                            }
                        });

                let mut options = FetchOptions::default();

                options.mode = Some(RequestMode::Cors);

                let task = FetchService::fetch_with_options(request, options, callback).unwrap();

                self.fetch_task = Some(task);
            }
            Msg::FetchSucced(story) => {
                self.story = Some(story);
                self.is_loading = false;
                self.update(Msg::FetchLinkPreview);
            }
            Msg::FetchFailed(error_message) => {
                self.error_message = Some(error_message);
                self.is_loading = false;
            }
            Msg::FetchLinkPreview => {
                self.is_loading = true;
                self.error_message = None;

                let story = self.story.clone().unwrap();

                if let Some(url) = story.url {
                    let request = Request::get(make_link_preview_url(url))
                        .body(Nothing)
                        .unwrap();
                    let callback = self.link.callback(
                        |res: Response<Json<Result<common::LinkPreview, Error>>>| {
                            let Json(data) = res.into_body();

                            match data {
                                Ok(link_preview) => Msg::FetchLinkPreviewSuccess(link_preview),
                                Err(err) => Msg::FetchFailed(err.to_string()),
                            }
                        },
                    );

                    let mut options = FetchOptions::default();

                    options.mode = Some(RequestMode::Cors);

                    let task =
                        FetchService::fetch_with_options(request, options, callback).unwrap();

                    self.fetch_task = Some(task);
                }
            }
            Msg::FetchLinkPreviewSuccess(link_preview) => {
                self.link_preview = Some(link_preview);
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

        if let Some(story) = self.story.clone() {
            return html! {
                <section id="story-page">
                    <header>
                        {self.render_story_image()}
                        <h2 class="story-title">{story.title.clone()}</h2>
                    </header>
                </section>
            };
        }

        Html::default()
    }
}
