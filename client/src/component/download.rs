use crate::Msg;
use seed::{prelude::*, *};
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Active,
    NotActive,
}
impl State {
    pub fn next(self) -> Self {
        match self {
            Self::Active => Self::NotActive,
            Self::NotActive => Self::Active,
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self::NotActive
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Active => "is-active",
            Self::NotActive => "",
        };
        write!(f, "{}", state)
    }
}
pub fn download(state: State) -> Node<Msg> {
    div![
        C![format![
            "dropdown is-right {}",
            match state {
                self::State::Active => {
                    "is-active"
                }
                self::State::NotActive => {
                    ""
                }
            }
        ]],
        div![
            C!["dropdown-trigger"],
            button![
                C!["button"],
                ev(Ev::Click, |_| Msg::DropdownNext),
                span!["Download"],
            ]
        ],
        div![
            C!["dropdown-menu"],
            attrs! {
            At::Id => "dropdown-menu",
            //Role => "menu"
            },
            div![
                C!["dropdown-content"],
                div![
                    C!["dropdown-item"],
                    button![
                        "Tar.gz",
                        ev(Ev::Click, move |_| Msg::Download("tar".to_string()))
                    ]
                ],
                div![
                    C!["dropdown-item"],
                    button![
                        "Zip",
                        ev(Ev::Click, move |_| Msg::Download("zip".to_string()))
                    ]
                ]
            ]
        ]
    ]
}
