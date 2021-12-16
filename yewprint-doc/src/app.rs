use crate::button_group::*;
use crate::buttons::*;
use crate::callout::*;
use crate::card::*;
use crate::checkbox::*;
use crate::collapse::*;
use crate::control_group::*;
use crate::divider::*;
use crate::html_select::*;
use crate::icon::*;
use crate::input_group::*;
use crate::menu::*;
use crate::numeric_input::*;
use crate::panel_stack::*;
use crate::progress_bar::*;
use crate::radio::*;
use crate::slider::*;
use crate::spinner::*;
use crate::switch::*;
use crate::tabs::*;
use crate::tag::*;
use crate::text::*;
use crate::text_area::*;
use crate::tree::*;
use yew_router::prelude::*;
use yew::prelude::*;
use yewprint::{IconName, Menu, MenuItem};

pub struct App {
    link: html::Scope<Self>,
    dark_theme: bool,
    active: Option<DocMenu>,
}

pub enum Msg {
    ToggleLight,
    GoToMenu(DocMenu),
}

#[derive(Clone, PartialEq, Routable)]
pub enum DocMenu {
    #[at("/button-group")]
    ButtonGroup,
    #[at("/button")]
    Button,
    #[at("/callout")]
    Callout,
    #[at("/card")]
    Card,
    #[at("/checkbox")]
    Checkbox,
    #[at("/collapse")]
    Collapse,
    #[at("/control-group")]
    ControlGroup,
    #[at("/html-select")]
    HtmlSelect,
    #[at("/divider")]
    Divider,
    #[at("/icon")]
    Icon,
    #[at("/input-group")]
    InputGroup,
    #[at("/menu")]
    Menu,
    #[at("/numeric-input")]
    NumericInput,
    #[at("/panel-stack")]
    PanelStack,
    #[at("/progress-bar")]
    ProgressBar,
    #[at("/radio")]
    Radio,
    #[at("/slider")]
    Slider,
    #[at("/spinner")]
    Spinner,
    #[at("/switch")]
    Switch,
    #[at("/tabs")]
    Tabs,
    #[at("/tag")]
    Tag,
    #[at("/textarea")]
    TextArea,
    #[at("/text")]
    Text,
    #[at("/tree")]
    Tree,
    #[at("/")]
    Home,
}

fn switch (route: &DocMenu) -> Html {
    match route {
        // TODO: redirect home instead of rendering button
        DocMenu::Button | DocMenu::Home => html!{<ButtonDoc />},
        DocMenu::ButtonGroup => html!{<ButtonGroupDoc />},
        DocMenu::Callout => html!{<CalloutDoc />},
        DocMenu::Card => html!{<CardDoc />},
        DocMenu::Checkbox => html!{<CheckboxDoc />},
        DocMenu::Collapse => html!{<CollapseDoc />},
        DocMenu::ControlGroup => html!{<ControlGroupDoc />},
        DocMenu::Divider => html!{<DividerDoc />},
        DocMenu::HtmlSelect => html!{<HtmlSelectDoc />},
        DocMenu::Icon => html!{<IconDoc />},
        DocMenu::InputGroup => html!{<InputGroupDoc />},
        DocMenu::Menu => html!{<MenuDoc />},
        DocMenu::NumericInput => html!{<NumericInputDoc />},
        DocMenu::PanelStack => html!{<PanelStackDoc />},
        DocMenu::ProgressBar => html!{<ProgressBarDoc />},
        DocMenu::Radio => html!{<RadioDoc />},
        DocMenu::Slider => html!{<SliderDoc />},
        DocMenu::Spinner => html!{<SpinnerDoc />},
        DocMenu::Switch => html!{<SwitchDoc />},
        DocMenu::Tabs => html!{<TabsDoc />},
        DocMenu::Tag => html!{<TagDoc />},
        DocMenu::Text => html!{<TextDoc />},
        DocMenu::TextArea => html!{<TextAreaDoc />},
        DocMenu::Tree => html!{<TreeDoc />},
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true),
            link: ctx.link().clone(),
            // TODO: don't use global default
            active: BrowserHistory::default().location().route(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleLight => self.dark_theme ^= true,
            Msg::GoToMenu(doc_menu) => {
                if self.active != Some(doc_menu.clone()) {
                    self.active = Some(doc_menu.clone());
                    BrowserHistory::default().push(doc_menu);
                }
            },
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let netlify_badge = if self.dark_theme {
            "https://www.netlify.com/img/global/badges/netlify-color-accent.svg"
        } else {
            "https://www.netlify.com/img/global/badges/netlify-color-bg.svg"
        };
        let go_to_theme_label = if self.dark_theme {
            "Light theme"
        } else {
            "Dark theme"
        };
        let go_to_theme_icon = if self.dark_theme {
            IconName::Flash
        } else {
            IconName::Moon
        };

        html! {
            <div class={classes!("docs-root", self.dark_theme.then(|| "bp3-dark"))}>
                <div class={classes!("docs-app")}>
                    <div class={classes!("docs-nav-wrapper")}>
                        <div class={classes!("docs-nav")}>
                            <div class={classes!("docs-nav-title")}>
                                <a class={classes!("docs-logo")} href="/">
                                    {crate::include_raw_html!("logo.svg")}
                                </a>
                                <div>
                                    <div class={classes!("bp3-navbar-heading", "docs-heading")}>
                                        {"Yewprint"}
                                    </div>
                                    <a
                                        class={classes!("bp3-text-muted")}
                                        href="https://github.com/yewprint/yewprint"
                                        target="_blank"
                                    >
                                        <small>{"View on GitHub"}</small>
                                    </a>
                                </div>
                            </div>
                            <Menu>
                                <MenuItem
                                    text={html!(go_to_theme_label)}
                                    onclick={self.link
                                        .callback(|e: MouseEvent| {e.prevent_default(); Msg::ToggleLight})}
                                    icon={go_to_theme_icon}
                                />
                                <MenuItem
                                    text={html!("Button")}
                                    href="/button"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Button)})
                                    }
                                    active={
                                        self.active == Some(DocMenu::Button) ||
                                        self.active == Some(DocMenu::Home)
                                    }
                                />
                                <MenuItem
                                    text={html!("ButtonGroup")}
                                    href="/button-group"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::ButtonGroup)})
                                    }
                                    active={self.active == Some(DocMenu::ButtonGroup)}
                                />
                                <MenuItem
                                    text={html!("Callout")}
                                    href="/callout"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Callout)})
                                    }
                                    active={self.active == Some(DocMenu::Callout)}
                                />
                                <MenuItem
                                    text={html!("Card")}
                                    href="/card"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Card)})
                                    }
                                    active={self.active == Some(DocMenu::Card)}
                                />
                                <MenuItem
                                    text={html!("Checkbox")}
                                    href="/checkbox"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Checkbox)})
                                    }
                                    active={self.active == Some(DocMenu::Checkbox)}
                                />
                                <MenuItem
                                    text={html!("Collapse")}
                                    href="/collapse"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Collapse)})
                                    }
                                    active={self.active == Some(DocMenu::Collapse)}
                                />
                                <MenuItem
                                    text={html!("ControlGroup")}
                                    href="/control-group"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::ControlGroup)})
                                    }
                                    active={self.active == Some(DocMenu::ControlGroup)}
                                    />
                                <MenuItem
                                    text={html!("Divider")}
                                    href="/divider"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Divider)})
                                    }
                                    active={self.active == Some(DocMenu::Divider)}
                                />
                                <MenuItem
                                    text={html!("HtmlSelect")}
                                    href="/html-select"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::HtmlSelect)})
                                    }
                                    active={self.active == Some(DocMenu::HtmlSelect)}
                                />
                                <MenuItem
                                    text={html!("Icon")}
                                    href="/icon"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Icon)})
                                    }
                                    active={self.active == Some(DocMenu::Icon)}
                                />
                                <MenuItem
                                    text={html!("InputGroup")}
                                    href="/input-group"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::InputGroup)})
                                    }
                                    active={self.active == Some(DocMenu::InputGroup)}
                                />
                                <MenuItem
                                    text={html!("Menu")}
                                    href="/menu"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Menu)})
                                    }
                                    active={self.active == Some(DocMenu::Menu)}
                                />
                                <MenuItem
                                    text={html!("NumericInput")}
                                    href="/numeric-input"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::NumericInput)})
                                    }
                                    active={self.active == Some(DocMenu::NumericInput)}
                                />
                                <MenuItem
                                    text={html!("PanelStack")}
                                    href="/panel-stack"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::PanelStack)})
                                    }
                                    active={self.active == Some(DocMenu::PanelStack)}
                                />
                                <MenuItem
                                    text={html!("ProgressBar")}
                                    href="/progress-bar"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::ProgressBar)})
                                    }
                                    active={self.active == Some(DocMenu::ProgressBar)}
                                />
                                <MenuItem
                                    text={html!("Radio")}
                                    href="/radio"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Radio)})
                                    }
                                    active={self.active == Some(DocMenu::Radio)}
                                />
                                <MenuItem
                                    text={html!("Slider")}
                                    href="/slider"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Slider)})
                                    }
                                    active={self.active == Some(DocMenu::Slider)}
                                />
                                <MenuItem
                                    text={html!("Spinner")}
                                    href="/spinner"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Spinner)})
                                    }
                                    active={self.active == Some(DocMenu::Spinner)}
                                />
                                <MenuItem
                                    text={html!("Switch")}
                                    href="/switch"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Switch)})
                                    }
                                    active={self.active == Some(DocMenu::Switch)}
                                />
                                <MenuItem
                                    text={html!("Tabs")}
                                    href="/tabs"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Tabs)})
                                    }
                                    active={self.active == Some(DocMenu::Tabs)}
                                />
                                <MenuItem
                                    text={html!("Tag")}
                                    href="/tag"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Tag)})
                                    }
                                    active={self.active == Some(DocMenu::Tag)}
                                />
                                <MenuItem
                                    text={html!("Text")}
                                    href="/text"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Text)})
                                    }
                                    active={self.active == Some(DocMenu::Text)}
                                />
                                <MenuItem
                                    text={html!("TextArea")}
                                    href="/textarea"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::TextArea)})
                                    }
                                    active={self.active == Some(DocMenu::TextArea)}
                                />
                                <MenuItem
                                    text={html!("Tree")}
                                    href="/tree"
                                    onclick={self.link
                                        .callback(|e: MouseEvent| { e.prevent_default(); Msg::GoToMenu(DocMenu::Tree)})
                                    }
                                    active={self.active == Some(DocMenu::Tree)}
                                />
                                // NOTE: thanks to keep this list of <MenuItem> sorted
                                //       alphabetically (except for the light switch)
                            </Menu>
                            <div class={classes!("docs-nav-sponsors")}>
                                <a href="https://www.netlify.com">
                                    <img
                                        src={netlify_badge}
                                        alt="Deploys by Netlify"
                                    />
                                </a>
                            </div>
                        </div>
                    </div>
                    <main class={classes!("docs-content-wrapper")} role="main">
                        <div class={classes!("docs-page")}>
                            <BrowserRouter>
                                <Switch<DocMenu> render={Switch::render(switch)} />
                            </BrowserRouter>
                        </div>
                    </main>
                </div>
            </div>
        }
    }
}
