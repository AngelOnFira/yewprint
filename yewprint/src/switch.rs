use yew::prelude::*;

pub struct Switch {
    props: SwitchProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for Switch {
    type Message = ();
    type Properties = SwitchProps;

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
            <label
                class={classes!(
                    "bp3-control",
                    "bp3-switch",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.class.clone(),
                )}
            >
                <input
                    type="checkbox"
                    checked={self.props.checked}
                    onclick={self.props.onclick.clone()}
                    disabled={self.props.disabled}
                />
                <span
                    class={classes!("bp3-control-indicator")}
                >
                </span>
                {self.props.label.clone()}
            </label>
        }
    }
}
