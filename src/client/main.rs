#![allow(non_snake_case)]
use dioxus::fermi::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use std::collections::HashMap;
use std::time::Duration;

pub mod server;
pub mod types;

use server::*;
use types::*;

static STATUSES: AtomRef<Vec<Status>> = |_| Vec::with_capacity(512);
static INSTANCES_ICONS: AtomRef<HashMap<String, String>> = |_| HashMap::new();

// #[prokio::main]
fn main() {
    // let json = include_str!("../../test_data/public.json");
    dioxus::web::launch(app);
    // dioxus::desktop::launch(app);
}

static SERVERS: [&str; 19] = [
    "https://genomic.social",
    "https://ecoevo.social",
    "https://sci.kiwi",
    // "https://mstdn.science",
    "https://fediscience.org",
    "https://scicomm.xyz",
    // "https://scholar.social",
    // "https://Tech.lgbt",
    "https://Mathstodon.xyz",
    "https://idf.social",
    // "https://types.pl",
    // "https://mastodon.elte.hu",
    "https://red.niboe.info",
    "https://sigmoid.social",
    "https://med-mastodon.com",
    "https://sciencemastodon.com",
    "https://drosophila.social",
    // "https://qoto.org", // CORS Error
    // "https://fosstodon.org",
    "https://astrodon.social",
    "https://deepspace.social",
    "https://spacey.space",
    "https://hci.social",
    "https://vis.social",
    "https://hcommons.social",
    "https://sciences.social",
];

fn update_statuses(cx: Scope) {
    let statuses = use_atom_ref(&cx, STATUSES);
    for i in SERVERS.iter() {
        let mut server = Server::new(i.to_string());
        use_future(&cx, (), |_| {
            let statuses = statuses.clone();
            async move {
                loop {
                    let new_status = server.fetch_newer().await;
                    if let Some(new_statuses) = new_status {
                        let mut writable = statuses.write();
                        writable.extend(new_statuses);
                        writable.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        if writable.len() > 256 {
                            writable.truncate(256);
                        }
                        drop(writable);
                    }

                    prokio::time::sleep(Duration::from_millis(31_000)).await;
                }
            }
        });
    }
}

fn get_server_icons(cx: Scope) {
    let instances_icons = use_atom_ref(&cx, INSTANCES_ICONS);
    for i in SERVERS.iter() {
        let server = Server::new(i.to_string());
        use_future(&cx, (), |_| {
            let instances_icons = instances_icons.clone();
            async move {
                let icon = server.get_icon().await;
                if let Some(icon) = icon {
                    let mut writable = instances_icons.write();
                    writable.insert(server.url_base.clone(), icon);
                    drop(writable);
                }
            }
        });
    }
}

fn app(cx: Scope) -> Element {
    get_server_icons(cx);
    update_statuses(cx);

    cx.render(rsx! (
        div { 
            class: "columns",
            div {
                class: "left_column"
            },
            div {
                class: "middle_column",
                Statuses {},
            },
            div {
                class: "right_column",
                About {},
            }
        }
    ))
}

fn About(cx: Scope) -> Element {
    cx.render(rsx! (
        img {
            src: "text_bubbles.png",
            style: "width: 100%;",
        }
        p {
            "This is an very early version of my aggregator. I've been learning the framework, and been doing genomics much longer than HTML/CSS lately! So give me some time. I can picture this becoming a full client as well."
        }
        p {
            "As I get the hang of this framework, expect fast iteration. Open to suggestions."
        }
        p {
            "Just tested positive for covid, so I'll be taking it easy this week. Sorry folks!"
        }
        a {
            href: "https://github.com/jguhlin/scikiwi",
            "GitHub Repo"
        }
        p {
            strong { "'Rules'" }
            ul {
                li {
                    "Don't leave any Mastodon client open when you aren't using it. It creates strain on the servers."
                }
            }
        }
        p {
            strong { "Built With" }
            ul {
                li {
                    a {
                        href: "https://www.rust-lang.org/",
                        "Rust Lang"
                    }
                }
                li {
                    a {
                        href: "https://dioxuslabs.com/",
                        "Dioxus",
                    }
                }
                li {
                    a {
                        href: "https://joinmastodon.org/",
                        "Mastodon"
                    }
                }
                li { 
                    a {
                        href: "https://docs.joinmastodon.org/client/intro/",
                        "Mastodon Client API"

                    }
                }
            }
        }
    ))
}

fn Statuses(cx: Scope) -> Element {
    let statuses = use_atom_ref(&cx, STATUSES);
    let statuses = statuses.read();
    let icons = use_atom_ref(&cx, INSTANCES_ICONS).read();
    cx.render(rsx! (
        div {
            class: "statuses",
            statuses.iter().map(|s| {
                let server_icon = icons.get(s.server_base.as_ref().unwrap()).unwrap_or(&String::new()).clone();
                rsx! (
                        StatusItem {
                            server_icon: server_icon,
                            avatar: s.account.avatar.clone(),
                            display_name: s.account.display_name.clone(),
                            content: s.content.clone(),
                            created_at: s.created_at.clone(),
                            acct: s.account.acct.clone(),
                            account_url: s.account.url.clone(),
                            url: s.interact_url.as_ref().unwrap().clone(),
                        }
                )
            })
         }
    ))
}

fn ServerIcon(cx: Scope<ServerIconProps>) -> Element {
    cx.render(rsx!(img {
        src: "{cx.props.icon}",
        width: "64",
        height: "64",
    }))
}

#[derive(PartialEq, Props)]
struct ServerIconProps {
    icon: String,
}

fn StatusItem(cx: Scope<StatusProps>) -> Element {
    cx.render(rsx! (
        div {
            class: "status",
            if cx.props.server_icon.is_some() && !cx.props.server_icon.as_ref().unwrap().is_empty() {
                rsx!(cx, ServerIcon {
                    icon: cx.props.server_icon.as_ref().unwrap().clone(),
                })
            } else {
                rsx!(cx, p {
                    style: "width: 64px; height: 64px; background-color: #000;"
                })
            }
            a { href: "{cx.props.account_url}",
                img { class: "status_avatar",
                    src: "{cx.props.avatar}" }
                p { class: "status_author", "{cx.props.display_name} (Click to View Profile)" }
                p { class: "status_acct", "{cx.props.acct}" }
            }
            p {
                class: "status_content",
                dangerous_inner_html: "{cx.props.content}"
            }
            p {
                a {
                    href: "{cx.props.url}",
                    class: "status_date",
                    style: "color: #DCDCDC; text-decoration: none; font-size: 0.9em; font-weight: 400; margin-left: 0.5em; margin-right: 0.5em;",
                    "{cx.props.created_at}"
                }
                a {
                    href: "{cx.props.url}?type=reply",
                    class: "status_link",
                    style: "color: #DCDCDC; text-decoration: none; font-size: 0.9em; font-weight: 400; margin-left: 0.5em; margin-right: 0.5em;",
                    "Reply"
                }
                a {
                    href: "{cx.props.url}?type=reblog",
                    class: "status_link",
                    style: "color: #DCDCDC; text-decoration: none; font-size: 0.9em; font-weight: 400; margin-left: 0.5em; margin-right: 0.5em;",
                    "Share"
                }
                a {
                    href: "{cx.props.url}?type=favourite",
                    class: "status_link",
                    style: "color: #DCDCDC; text-decoration: none; font-size: 0.9em; font-weight: 400; margin-left: 0.5em; margin-right: 0.5em;",
                    "Like"
                }
            }
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct StatusProps {
    pub server_icon: Option<String>,
    pub avatar: String,
    pub display_name: String,
    pub acct: String,
    pub content: String,
    pub created_at: String,
    pub url: String,
    pub account_url: String,
}
