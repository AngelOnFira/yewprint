use crate::{Icon, IconName};
use yew::prelude::*;
use web_sys::HtmlSelectElement;

pub struct HtmlSelect<T: Clone + PartialEq + 'static> {
    props: HtmlSelectProps<T>,
    link: html::Scope<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct HtmlSelectProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: Option<T>,
    pub options: Vec<(T, String)>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: Clone + PartialEq + 'static> Component for HtmlSelect<T> {
    type Message = Event;
    type Properties = HtmlSelectProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            link: ctx.link().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let input = msg.target_dyn_into::<HtmlSelectElement>();
        input.map(|input| {
            let i = input.selected_index() as usize;
            self.props.onchange.emit(self.props.options[i].0.clone());
        });
        false
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
        let option_children = self
            .props
            .options
            .iter()
            .map(|(value, label)| {
                let selected = self
                    .props
                    .value
                    .as_ref()
                    .map(|x| value == x)
                    .unwrap_or_default();

                html! {
                    <option selected={selected}>
                        {label}
                    </option>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class={classes!(
                    "bp3-html-select",
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.class.clone(),
                )}
            >
                <select
                    disabled={self.props.disabled}
                    onchange={self.link.callback(|x| x)}
                    title={self.props.title.clone()}
                >
                    {option_children}
                </select>
                <Icon icon={IconName::DoubleCaretVertical}/>
            </div>
        }
    }
}
