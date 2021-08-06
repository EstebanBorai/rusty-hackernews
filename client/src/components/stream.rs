use anyhow::Error;
use common::Story;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::RequestMode;

use crate::components::story::Story as StoryComponent;

#[cfg(not(debug_assertions))]
const STORIES_V1_ENDPOINT: &str = "https://fluxcap.herokuapp.com/api/v1/stories";

#[cfg(debug_assertions)]
const STORIES_V1_ENDPOINT: &str = "http://0.0.0.0:3000/api/v1/stories";

pub struct Stream {
    error_message: Option<String>,
    fetch_task: Option<FetchTask>,
    is_loading: bool,
    link: ComponentLink<Self>,
    stories: Option<Vec<Story>>,
}

pub enum Msg {
    FetchStories,
    FetchSucced(Vec<Story>),
    FetchFailed(String),
}

impl Stream {
    fn render_story(story: Story) -> Html {
        let Story {
            title,
            by,
            descendants: _,
            id: _,
            kids: _,
            score,
            time: _,
            r#type: _,
            url,
        } = story;
        let image_url: Option<String> = None;

        html! {
            <StoryComponent
                by=by
                title=title
                image_url=image_url
                score=score
                url=url
            />
        }
    }

    fn render_stories(&self) -> Html {
        if let Some(stories) = &self.stories {
            return html! {
                <ul id="stream">
                    {
                        for stories.into_iter().map(|story| {
                            Stream::render_story(story.clone())
                        })
                    }
                </ul>
            };
        }

        html! {
            <h2>{"No stories found at this time. Retry later today!"}</h2>
        }
    }
}

impl Component for Stream {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error_message: None,
            fetch_task: None,
            is_loading: true,
            link,
            stories: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchStories => {
                self.is_loading = true;
                self.error_message = None;

                let request = Request::get(STORIES_V1_ENDPOINT).body(Nothing).unwrap();
                let callback =
                    self.link
                        .callback(|res: Response<Json<Result<Vec<Story>, Error>>>| {
                            let Json(data) = res.into_body();

                            match data {
                                Ok(stories) => Msg::FetchSucced(stories),
                                Err(err) => Msg::FetchFailed(err.to_string()),
                            }
                        });

                let mut options = FetchOptions::default();

                options.mode = Some(RequestMode::Cors);

                let task = FetchService::fetch_with_options(request, options, callback).unwrap();

                self.fetch_task = Some(task);
            }
            Msg::FetchFailed(error_message) => {
                self.is_loading = false;
                self.error_message = Some(error_message);
            }
            Msg::FetchSucced(stories) => {
                self.is_loading = false;
                self.error_message = None;
                self.stories = Some(stories);
            }
        };

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(Msg::FetchStories);
        }
    }

    fn view(&self) -> Html {
        if self.is_loading {
            return html! {
                <h1>{"Fetching the latest stories"}</h1>
            };
        }

        if self.error_message.is_some() {
            let error_message = self.error_message.clone();
            let error_message = error_message.unwrap();

            return html! {
                <h1>{"An error ocurred fetching stories!"}<br />{error_message}</h1>
            };
        }

        self.render_stories()
    }
}
