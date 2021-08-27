use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LinkPreview {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub surname: Option<String>,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub mod hacker_news {
    use serde::{Deserialize, Serialize};
    use std::convert::TryFrom;

    /// Item types
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Type {
        Job,
        Story,
        Comment,
        Poll,
        PollOpt,
    }

    /// Stories, comments, jobs, Ask HNs and even polls are just items.
    /// They're identified by their ids, which are unique integers, and
    /// live under `/v0/item/<id>`.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Item {
        /// The item's unique id.
        pub id: u64,
        /// `true` if the item is deleted.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or
        /// "pollopt".
        pub r#type: Type,
        /// The username of the item's author.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub by: Option<String>,
        /// Creation date of the item, in Unix Time.
        pub time: u64,
        /// The comment, story or poll text. HTML.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        /// true if the item is dead.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dead: Option<bool>,
        /// The comment's parent: either another comment or the relevant story.
        pub parent: Option<u64>,
        /// The pollopt's associated poll.
        pub poll: Option<u64>,
        /// The ids of the item's comments, in ranked display order.
        pub kids: Option<Vec<u64>>,
        /// The URL of the story.
        pub url: Option<String>,
        /// The story's score, or the votes for a pollopt.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub score: Option<u32>,
        /// The title of the story, poll or job. HTML.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        /// A list of related pollopts, in display order.
        pub parts: Option<Vec<u64>>,
        /// In the case of stories or polls, the total comment count.
        pub descendants: Option<u64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Story {
        /// The item's unique id.
        pub id: u64,
        /// The type of item. One of "job", "story", "comment", "poll", or
        /// "pollopt".
        pub r#type: Type,
        /// The username of the item's author.
        pub by: String,
        /// Creation date of the item, in Unix Time.
        pub time: u64,
        /// The ids of the item's comments, in ranked display order.
        pub kids: Option<Vec<u64>>,
        /// The URL of the story.
        pub url: Option<String>,
        /// The story's score, or the votes for a pollopt.
        pub score: u32,
        /// The title of the story, poll or job. HTML.
        pub title: String,
        /// In the case of stories or polls, the total comment count.
        pub descendants: Option<u64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Comment {
        /// The item's unique id.
        pub id: u64,
        /// The type of item. One of "job", "story", "comment", "poll", or
        /// "pollopt".
        pub r#type: Type,
        /// The username of the item's author.
        pub by: Option<String>,
        /// The ids of the item's comments, in ranked display order.
        pub kids: Option<Vec<u64>>,
        /// The comment's parent: either another comment or the relevant story.
        pub parent: Option<u64>,
        /// The comment, story or poll text. HTML.
        pub text: Option<String>,
        /// Creation date of the item, in Unix Time.
        pub time: u64,
    }

    impl TryFrom<Item> for Story {
        type Error = anyhow::Error;

        fn try_from(value: Item) -> Result<Self, Self::Error> {
            Ok(Story {
                id: value.id,
                r#type: value.r#type,
                by: value.by.unwrap(),
                time: value.time,
                kids: value.kids,
                url: value.url,
                score: value.score.unwrap(),
                title: value.title.unwrap(),
                descendants: value.descendants,
            })
        }
    }

    impl TryFrom<Item> for Comment {
        type Error = anyhow::Error;

        fn try_from(value: Item) -> Result<Self, Self::Error> {
            Ok(Comment {
                id: value.id,
                r#type: value.r#type,
                by: value.by,
                time: value.time,
                kids: value.kids,
                text: value.text,
                parent: value.parent,
            })
        }
    }
}
