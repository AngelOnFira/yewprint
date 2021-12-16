use yew::prelude::*;

pub struct ControlGroup {
    props: ControlGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ControlGroupProps {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for ControlGroup {
    type Message = ();
    type Properties = ControlGroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
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
            <div
                class={classes!(
                    "bp3-control-group",
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.class.clone(),
                )}
            >
                {self.props.children.clone()}
            </div>
        }
    }
}
