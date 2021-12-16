use yew::prelude::*;

pub struct Text {
    props: TextProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextProps {
    #[prop_or_default]
    pub ellipsize: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    /// Wrap text in `span` instead of `div`.
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub style: String,
}

impl Component for Text {
    type Message = ();
    type Properties = TextProps;

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
            <@{if self.props.inline { "span" } else { "div"}}
                class={classes!(
                    self.props.class.clone(),
                    self.props.ellipsize.then (|| "bp3-text-overflow-ellipsis"),
                )}
                style={self.props.style.clone()}
                title={self.props.title.clone()}
            >
                {self.props.children.clone()}
            </@>
        }
    }
}
