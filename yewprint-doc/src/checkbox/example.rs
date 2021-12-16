use yew::prelude::*;
use yewprint::{Checkbox, Label};

#[derive(Clone, PartialEq)]
pub struct CheckedState {
    pub gilard: bool,
    pub jason: bool,
    pub antoine: bool,
}

pub struct Example {
    props: ExampleProps,
    state: CheckedState,
    link: html::Scope<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
}

pub enum Msg {
    ToggleGilard,
    ToggleJason,
    ToggleAntoine,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            link: ctx.link().clone(),
            state: CheckedState {
                gilard: false,
                jason: false,
                antoine: false
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleGilard => self.state.gilard = !self.state.gilard,
            Msg::ToggleJason => self.state.jason = !self.state.jason,
            Msg::ToggleAntoine => self.state.antoine = !self.state.antoine,
        }
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

                <Label>{"Assign responsability"}</Label>
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    checked={self.state.gilard}
                    onchange={self.link.callback(|_| Msg::ToggleGilard)}
                    label={html!("Gilad Gray")}
                />
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    checked={self.state.jason}
                    onchange={self.link.callback(|_| Msg::ToggleJason)}
                    label={html!("Jason Killian")}
                />
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    checked={self.state.antoine}
                    onchange={self.link.callback(|_| Msg::ToggleAntoine)}
                    label={html!("Antoine Llorca")}
                />
            </div>
        }
    }
}
