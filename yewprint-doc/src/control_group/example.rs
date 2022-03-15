use yew::prelude::*;
use yewprint::{Button, ControlGroup, HtmlSelect, IconName, InputGroup};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub vertical: bool,
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
            <ControlGroup
                fill={ctx.props().fill}
                vertical={ctx.props().vertical}
            >
                <HtmlSelect<Option<Sorting>>
                    options={vec![
                        (None, "Filter".to_string()),
                        (Some(Sorting::NameAscending), "Name - ascending".to_string()),
                        (Some(Sorting::NameDescending), "Name - descending".to_string()),
                        (Some(Sorting::PriceAscending), "Price - ascending".to_string()),
                        (Some(Sorting::PriceDescending), "Price - descending".to_string()),
                    ]}
                />
                <InputGroup placeholder="Find filters..." />
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
