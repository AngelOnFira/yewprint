use yew::prelude::*;
use yewprint::{Label, Switch};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
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
            <div>
                <Label>{"Privacy settings"}</Label>
                <Switch
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!("Enabled")}
                />
                <Switch
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!(<em>{"Public"}</em>)}
                />
                <Switch
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!(<strong>{"Cooperative"}</strong>)}
                />
                <Switch
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!(<u>{"Containing Text"}</u>)}
                />
            </div>
        }
    }
}
