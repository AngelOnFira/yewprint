use crate::{Button, ButtonGroup, ControlGroup, IconName, InputGroup, Intent};
use std::fmt::Display;
use std::ops::{Add, Bound, RangeBounds, Sub};
use std::str::FromStr;
use yew::html::IntoPropValue;
use yew::prelude::*;
use web_sys::HtmlInputElement;

pub struct NumericInput<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Default
        + 'static,
{
    props: NumericInputProps<T>,
    link: html::Scope<Self>,
    input: String,
}


#[derive(Clone, PartialEq, Properties, Default)]
pub struct NumericInputProps<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Default
        + 'static,
{
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub left_element: Option<Html>,
    #[prop_or_default]
    pub right_element: Option<Html>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: T,
    #[prop_or_default]
    pub bounds: NumericInputRangeBounds<T>,
    #[prop_or_default]
    pub increment: T,
    #[prop_or_default]
    pub disable_buttons: bool,
    #[prop_or_default]
    pub buttons_on_the_left: bool,
}

pub enum Msg {
    InputUpdate(InputEvent),
    Up,
    Down,
    Noop,
}

impl<T> Component for NumericInput<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Default
        + 'static,
{
    type Message = Msg;
    type Properties = NumericInputProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            link: ctx.link().clone(),
            input: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUpdate(event) => {
                let input = event.target_dyn_into::<HtmlInputElement>();
                let mut new_value = "".to_owned();

                input.map(|input| {
                    new_value = input.value();
                });
                // - is here to be able to delete and write negative numbers
                if new_value.trim() == "-" {
                    false
                } else if new_value.trim() == "" {
                    self.update_value(Default::default())
                } else if let Ok(new_value) = new_value.trim().parse::<T>() {
                    self.update_value(new_value)
                } else {
                    self.update_value(self.props.value)
                }
            }
            Msg::Up => self.update_value(self.props.value + self.props.increment),
            Msg::Down => self.update_value(self.props.value - self.props.increment),
            Msg::Noop => false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.props != *ctx.props() {
            if self.props.value != ctx.props().value {
                if ctx.props().value == Default::default() {
                    self.input = "".to_owned();
                } else {
                    self.input = ctx.props().value.to_string();
                }
            }
            self.props = ctx.props().clone();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let NumericInputProps {
            value,
            increment,
            disabled,
            disable_buttons,
            buttons_on_the_left,
            ..
        } = self.props;
        let bounds = &self.props.bounds;
        let button_up_disabled = disabled || bounds.clamp(value + increment, increment) == value;
        let button_down_disabled = disabled || bounds.clamp(value - increment, increment) == value;

        let buttons = if disable_buttons {
            html!()
        } else {
            html! {
                <ButtonGroup vertical=true class={classes!("bp3-fixed")}>
                    <Button
                        icon={IconName::ChevronUp}
                        disabled={button_up_disabled}
                        onclick={self.link.callback(|_| Msg::Up)}
                    />
                    <Button
                        icon={IconName::ChevronDown}
                        disabled={button_down_disabled}
                        onclick={self.link.callback(|_| Msg::Down)}
                    />
                </ButtonGroup>
            }
        };

        let input_group = html! {
            <InputGroup
                placeholder={self.props.placeholder.clone()}
                large={self.props.large}
                disabled={self.props.disabled}
                left_icon={self.props.left_icon}
                left_element={self.props.left_element.clone()}
                right_element={self.props.right_element.clone()}
                value={self.input.clone()}
                // TODO: normalize if we are to use oninput directly on inputs or process onchange and send string
                oninput={self.link.callback(|e: InputEvent| Msg::InputUpdate(e))}
                onkeydown={self.link.callback(|e: KeyboardEvent| {
                    if e.key() == "ArrowUp" {
                        Msg::Up
                    } else if e.key() == "ArrowDown" {
                        Msg::Down
                    } else {
                        Msg::Noop
                    }
                })}
            />
        };

        if buttons_on_the_left {
            html! {
                <ControlGroup
                    class={classes!("bp3-numeric-input")}
                    fill={self.props.fill}
                    large={self.props.large}
                >
                    {buttons}
                    {input_group}
                </ControlGroup>
            }
        } else {
            html! {
                <ControlGroup
                    class={classes!("bp3-numeric-input")}
                    fill={self.props.fill}
                    large={self.props.large}
                >
                    {input_group}
                    {buttons}
                </ControlGroup>
            }
        }
    }
}

impl<T> NumericInput<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Default
        + 'static,
{
    fn update_value(&mut self, mut new_value: T) -> bool {
        if new_value != Default::default() {
            new_value = self.props.bounds.clamp(new_value, self.props.increment);
            self.input = new_value.to_string();
        } else {
            self.input = "".to_owned();
        }

        if new_value != self.props.value {
            self.props.onchange.emit(new_value);
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericInputRangeBounds<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}

impl<T> Default for NumericInputRangeBounds<T> {
    fn default() -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl<T> NumericInputRangeBounds<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn clamp(&self, value: T, increment: T) -> T {
        match (self.start, self.end) {
            (Bound::Included(min), _) if value < min => min,
            (Bound::Excluded(min), _) if value <= min => min + increment,
            (_, Bound::Included(max)) if value > max => max,
            (_, Bound::Excluded(max)) if value >= max => max - increment,
            _ => value,
        }
    }
}

macro_rules! impl_into_prop_value {
    ($path:path) => {
        impl<T: Copy> IntoPropValue<NumericInputRangeBounds<T>> for $path {
            fn into_prop_value(self) -> NumericInputRangeBounds<T> {
                NumericInputRangeBounds {
                    start: self.start_bound().cloned(),
                    end: self.end_bound().cloned(),
                }
            }
        }
    };
}

impl_into_prop_value!(std::ops::Range<T>);
impl_into_prop_value!(std::ops::RangeFrom<T>);
impl_into_prop_value!(std::ops::RangeFull);
impl_into_prop_value!(std::ops::RangeInclusive<T>);
impl_into_prop_value!(std::ops::RangeTo<T>);
impl_into_prop_value!(std::ops::RangeToInclusive<T>);
