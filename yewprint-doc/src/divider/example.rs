use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Divider};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
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
            <ButtonGroup vertical={self.props.vertical}>
                <Button>{"File"}</Button>
                <Button>{"Edit"}</Button>
                <Divider vertical={self.props.vertical} />
                <Button>{"Create"}</Button>
                <Button>{"Delete"}</Button>
                <Divider vertical={self.props.vertical} />
                // <Button icon=IconName::Add />
                // <Button icon=IconName::Remove />
            </ButtonGroup>
        }
    }
}
