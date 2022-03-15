use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Button, ButtonGroup, IconName};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub large: bool,
    pub vertical: bool,
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
            <ButtonGroup
                minimal={ctx.props().minimal}
                fill={ctx.props().fill}
                large={ctx.props().large}
                vertical={ctx.props().vertical}
                style={Cow::Borrowed("margin:0;")}
            >
                <Button icon={IconName::Database>{"Queries"}</Button>}
                <Button icon={IconName::Function>{"Functions"}</Button>}
                <Button icon={IconName::Cog>{"Options"}</Button>}
            </ButtonGroup>
        }
    }
}
