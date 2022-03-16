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
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Callout
                title={ctx.props().show_title.then(|| "Visually important content")}
                without_icon={!ctx.props().show_icon}
                intent={ctx.props().intent}
            >
                <p>{"The Callout element's background reflects its intent, if any."}</p>
            </Callout>
        }
    }
}
