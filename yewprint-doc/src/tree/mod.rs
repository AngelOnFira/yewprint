mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct TreeDoc;

impl Component for TreeDoc {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        TreeDoc
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Tree"}</H1>
                <SourceCodeUrl />
                <ExampleContainer source={source}>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_source_code_component!();
