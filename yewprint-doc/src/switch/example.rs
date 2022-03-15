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
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Label>{"Privacy settings"}</Label>
                <Switch
                    disabled={ctx.props().disabled}
                    inline={ctx.props().inline}
                    large={ctx.props().large}
                    label={html!("Enabled")}
                />
                <Switch
                    disabled={ctx.props().disabled}
                    inline={ctx.props().inline}
                    large={ctx.props().large}
                    label={html!(<em>{"Public"}</em>)}
                />
                <Switch
                    disabled={ctx.props().disabled}
                    inline={ctx.props().inline}
                    large={ctx.props().large}
                    label={html!(<strong>{"Cooperative"}</strong>)}
                />
                <Switch
                    disabled={ctx.props().disabled}
                    inline={ctx.props().inline}
                    large={ctx.props().large}
                    label={html!(<u>{"Containing Text"}</u>)}
                />
            </div>
        }
    }
}
