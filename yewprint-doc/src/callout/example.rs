use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Callout, Intent};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub show_icon: bool,
    pub show_title: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.props != *ctx.props() {
            self.props = ctx.props().clone();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Callout
                title={self.props.show_title.then(|| Cow::Borrowed("Visually important content"))}
                without_icon={!self.props.show_icon}
                intent={self.props.intent}
            >
                <p>{"The Callout element's background reflects its intent, if any."}</p>
            </Callout>
        }
    }
}
