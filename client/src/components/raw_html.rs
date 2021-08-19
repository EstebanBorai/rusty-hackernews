use yew::prelude::*;
use yew::web_sys::Element;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

pub struct RawHtml {
    props: RawHtmlProps,
    node_ref: NodeRef,
}

impl Component for RawHtml {
    type Message = ();
    type Properties = RawHtmlProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unreachable!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.props.inner_html);
    }

    fn view(&self) -> Html {
        // create the parent element and store a reference to it
        html! {
            <div ref=self.node_ref.clone()/>
        }
    }
}
