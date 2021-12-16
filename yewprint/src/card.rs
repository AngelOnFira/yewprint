use crate::Elevation;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub elevation: Elevation,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub interactive: bool,
    pub children: html::Children,
}

pub struct Card {
    props: CardProps,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

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
            <div class={classes!(
                "bp3-card",
                self.props.class.clone(),
                self.props.elevation,
                self.props.interactive.then(|| "bp3-interactive"),
            )}
            onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </div>
        }
    }
}
