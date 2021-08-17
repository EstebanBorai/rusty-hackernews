use js_sys::encode_uri_component;

use crate::constants::api;

pub fn make_link_preview_url(story_url: String) -> String {
    #[allow(unused_unsafe)]
    let url_query = unsafe { encode_uri_component(story_url.as_str()) };
    let url_query = String::from(url_query);
    let mut final_url = String::from(api::v1::PREVIEWS);

    final_url.push_str("?url=");
    final_url.push_str(url_query.as_str());

    final_url
}
