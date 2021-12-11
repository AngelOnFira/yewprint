use yew::prelude::*;
use yewprint::{Checkbox, Label};

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
        Self { props: ctx.props() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Label>{"Assign responsability"}</Label>
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!("Gilad Gray")}
                />
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!("Jason Killian")}
                />
                <Checkbox
                    disabled={self.props.disabled}
                    inline={self.props.inline}
                    large={self.props.large}
                    label={html!("Antoine Llorca")}
                />
            </div>
        }
    }
}
