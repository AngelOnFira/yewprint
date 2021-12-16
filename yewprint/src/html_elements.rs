use yew::prelude::*;

macro_rules! build_component {
    ($name:ident, $props_name:ident, $tag:tt, $class:literal) => {
        pub type $name = $props_name;

        #[derive(Debug, Clone, PartialEq, Properties)]
        pub struct $props_name {
            #[prop_or_default]
            pub class: Classes,
            #[prop_or_default]
            pub children: html::Children,
        }

        impl Component for $props_name {
            type Message = ();
            type Properties = $props_name;

            fn create(ctx: &Context<Self>) -> Self {
                Self {
                    children: ctx.props().children.clone(),
                    class: ctx.props().class.clone(),
                }
            }

            fn view(&self, ctx: &yew::Context<Self>) -> Html {
                html! {
                    <$tag class={classes!($class, ctx.props().class.clone())}>
                        {self.children.clone()}
                    </$tag>
                }
            }
        }
    };
}

build_component!(H1, H1Props, h1, "bp3-heading");
build_component!(H2, H2Props, h2, "bp3-heading");
build_component!(H3, H3Props, h3, "bp3-heading");
build_component!(H4, H4Props, h4, "bp3-heading");
build_component!(H5, H5Props, h5, "bp3-heading");
build_component!(H6, H6Props, h6, "bp3-heading");

build_component!(Blockquote, BlockquoteProps, blockquote, "bp3-blockquote");
build_component!(Code, CodeProps, code, "bp3-code");
build_component!(Label, LabelProps, label, "bp3-label");
build_component!(Pre, PreProps, pre, "bp3-pre");
build_component!(Ol, OlProps, ol, "bp3-ol");
build_component!(Ul, UlProps, ul, "bp3-ul");
