use yew::prelude::*;
use yewprint::{Button, ControlGroup, HtmlSelect, IconName, InputGroup};

pub struct Example {
    props: ExampleProps,
    link: html::Scope<Self>,
    selected: Option<Sorting>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub vertical: bool,
}

pub enum Msg {
    Select(Option<Sorting>),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            link: ctx.link().clone(),
            selected: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(v) => self.selected = v,
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
            <ControlGroup
                fill={self.props.fill}
                vertical={self.props.vertical}
            >
                <HtmlSelect<Option<Sorting>>
                    options={vec![
                        (None, "Filter".to_string()),
                        (Some(Sorting::NameAscending), "Name - ascending".to_string()),
                        (Some(Sorting::NameDescending), "Name - descending".to_string()),
                        (Some(Sorting::PriceAscending), "Price - ascending".to_string()),
                        (Some(Sorting::PriceDescending), "Price - descending".to_string()),
                    ]}
                    value={self.selected}
                    onchange={self.link.callback(|x| Msg::Select(x))}
                />
                // TODO: onchange
                <InputGroup
                    placeholder="Find filters..."
                />
                <Button icon={IconName::ArrowRight} />
            </ControlGroup>
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum Sorting {
    NameAscending,
    NameDescending,
    PriceAscending,
    PriceDescending,
}
