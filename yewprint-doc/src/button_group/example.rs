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
            <ButtonGroup
                minimal={self.props.minimal}
                fill={self.props.fill}
                large={self.props.large}
                vertical={self.props.vertical}
                style="margin:0;"
            >
                <Button icon={IconName::Database}>{"Queries"}</Button>
                <Button icon={IconName::Function}>{"Functions"}</Button>
                <Button icon={IconName::Cog}>{"Options"}</Button>
            </ButtonGroup>
        }
    }
}
