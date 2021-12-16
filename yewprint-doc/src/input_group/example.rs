use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};
use web_sys::HtmlInputElement;

pub struct Example {
    link: html::Scope<Self>,
    props: ExampleProps,
    histogram_value: String,
    password_value: String,
    password_strength: Html,
    tags_value: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub fill: bool,
    pub large: bool,
    pub small: bool,
    pub round: bool,
}

pub enum Msg {
    AddHistogramEntry,
    UpdateHistogram(InputEvent),
    AddPasswordEntry,
    UpdatePassword(InputEvent),
    AddTagsEntry,
    UpdateTags(InputEvent),
    Noop,
}

macro_rules! alert {
    ($($arg:tt)*) => {
        gloo_dialogs::alert(&format!(
            $($arg)*
        ))
    };
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            props: ctx.props().clone(),
            link: ctx.link().clone(),
            histogram_value: Default::default(),
            password_value: Default::default(),
            password_strength: Default::default(),
            tags_value: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddHistogramEntry => {
                alert!("You sent: {}", self.histogram_value);
                self.histogram_value = Default::default();
                true
            }
            Msg::UpdateHistogram(event) => {
                let input = event.target_dyn_into::<HtmlInputElement>();
                input.map(|input| {
                    self.histogram_value = input.value();
                });
                true
            }
            Msg::AddPasswordEntry => {
                alert!("You sent: {}", self.password_value);
                self.password_value = Default::default();
                self.password_strength = html!();
                true
            }
            Msg::UpdatePassword(event) => {
                let input = event.target_dyn_into::<HtmlInputElement>();
                input.map(|input| {
                    let value = input.value();
                    self.password_strength = match value.len() {
                        n if n == 0 => html!(),
                        n if n < 4 => html!(<Tag>{"weak"}</Tag>),
                        n if n < 8 => html!(<Tag>{"medium"}</Tag>),
                        _ => html!(<Tag>{"strong"}</Tag>),
                    };
                    self.password_value = value;
                });
                true
            }
            Msg::AddTagsEntry => {
                alert!("You sent: {}", self.tags_value);
                self.tags_value = Default::default();
                true
            }
            Msg::UpdateTags(event) => {
                let input = event.target_dyn_into::<HtmlInputElement>();
                input.map(|input| {
                    self.tags_value = input.value();
                });
                true
            }
            Msg::Noop => false,
        }
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
            <>
                <InputGroup
                    fill={self.props.fill}
                    large={self.props.large}
                    small={self.props.small}
                    round={self.props.round}
                    disabled={self.props.disabled}
                    left_icon={IconName::Filter}
                    placeholder="Filter histogram..."
                    value={self.histogram_value.clone()}
                    oninput={self.link.callback(|e: InputEvent| Msg::UpdateHistogram(e))}
                    onkeydown={self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddHistogramEntry } else { Msg::Noop }
                    })}
                />
                <InputGroup
                    fill={self.props.fill}
                    large={self.props.large}
                    small={self.props.small}
                    round={self.props.round}
                    disabled={self.props.disabled}
                    left_element={self.password_strength.clone()}
                    placeholder="Enter your password..."
                    value={self.password_value.clone()}
                    oninput={self.link.callback(|e: InputEvent| Msg::UpdatePassword(e))}
                    onkeydown={self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddPasswordEntry } else { Msg::Noop }
                    })}
                    right_element={{html! {
                        <Button
                            icon={IconName::Lock}
                            minimal=true
                            disabled={self.props.disabled}
                        />
                    }}}
                />
                <InputGroup
                    fill={self.props.fill}
                    large={self.props.large}
                    small={self.props.small}
                    round={self.props.round}
                    disabled={self.props.disabled}
                    left_icon={IconName::Tag}
                    placeholder="Find tags"
                    value={self.tags_value.clone()}
                    oninput={self.link.callback(|e: InputEvent| Msg::UpdateTags(e))}
                    onkeydown={self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddTagsEntry } else { Msg::Noop }
                    })}
                    right_element={{html! {
                        <Tag
                            minimal={true}
                            round={self.props.round}
                        >
                            {{10000 / 1.max(self.tags_value.len().pow(2))}}
                        </Tag>
                    }}}
                />
            </>
        }
    }
}
