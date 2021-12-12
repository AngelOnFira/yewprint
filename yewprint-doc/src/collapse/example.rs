use yew::prelude::*;
use yewprint::{Button, Collapse};

pub struct Example {
    link: &html::Scope<Self>,
    collapsed: bool,
}

pub enum Msg {
    ToggleCollapse,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            collapsed: true,
            link,
        }
    }

    fn update(&mut self, _ctx:  &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleCollapse => self.collapsed ^= true,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let logs = include_str!("example.log");

        html! {
            <div>
                <Button onclick={self.link.callback(|_| Msg::ToggleCollapse)}>
                    {"Toggle collapse"}
                </Button>
                <Collapse
                    is_open={!self.collapsed}
                    keep_children_mounted=true
                >
                    <pre class="bp3-code-block">
                        {logs}
                    </pre>
                </Collapse>
            </div>
        }
    }
}
