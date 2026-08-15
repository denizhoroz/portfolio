use dioxus::{html::{a::href, img}, prelude::*};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},

    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

// Import assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const TEXT_WHOAMI: &str = include_str!("../assets/texts/whoami.md");
const IMG_PR1: Asset = asset!("/assets/img/project-img1.png");
const IMG_PR2: Asset = asset!("/assets/img/project-img2.png");
const IMG_PR3: Asset = asset!("/assets/img/project-img3.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 300px;",
            Hero {}
            AboutMe {}
            MyWorks {}
            FollowMe {}
        }
    }
}

// Title section
#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "page-block",  
            div {
                h1 { 
                    class: "block-title",
                    "denizhoroz" 
                }

                h1 { 
                    class: "block-subtitle",
                    "Software Engineer" 
                }
            }
        }
    }
}

// AboutMe section
#[component]
pub fn AboutMe() -> Element {
    rsx! {
        div {
            id: "aboutme",
            class: "page-block",

            div {
                class: "zen-content",
                h1 {
                    class: "block-title",
                    "who am i?"
                }
                h1 {
                    class: "block-desc",
                    {TEXT_WHOAMI}
                }
            }
        }
    }
}

// MyWorks section
#[component]
pub fn MyWorks() -> Element {
    rsx! {
        div {
            id: "myworks",
            class: "page-block",
            div {
                style: "display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 20px",
                h1 {
                    class: "block-title",
                    "my works"
                }
                div {
                    class: "project-container",
 
                    {ProjectBox(ProjectBoxProps { 
                        title: ("Laser Turret Project".to_string()), 
                        description: ("An affordable, fully autonomous laser turret that detects, tracks, and fires at targets using AI-based image processing. Built as a Mechatronics Engineering graduation project at Istanbul Ticaret University.".to_string()), 
                        image: (IMG_PR1), 
                        link: (None) 
                    })}

                }

                a {
                    class: "block-desc",
                    "see all projects"
                }
            }
        }
    }
}


// FollowMe section
#[component]
pub fn FollowMe() -> Element {
    rsx! {
        div {
            id: "followme",
            class: "page-block",
            h1 {
                class: "block-title",
                "reach me"
            }
        }
    }
} 



// Project Box Props
#[derive(Props, Clone, PartialEq)]
pub struct ProjectBoxProps {
    title: String,
    description: String,
    image: Asset,
    link: Option<String>,
}


// Small Project Box
#[component]
pub fn ProjectBox(props: ProjectBoxProps) -> Element {
    rsx! {
        div {
            class: "projectbox",

            img { src: props.image }
            h3 { "{props.title}" }
            p { "{props.description}" }
        }
    }
}


// /// Blog page
// #[component]
// pub fn Blog(id: i32) -> Element {
//     rsx! {
//         div {
//             id: "blog",

//             // Content
//             h1 { "This is blog #{id}!" }
//             p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

//             // Navigation links
//             Link {
//                 to: Route::Blog { id: id - 1 },
//                 "Previous"
//             }
//             span { " <---> " }
//             Link {
//                 to: Route::Blog { id: id + 1 },
//                 "Next"
//             }
//         }
//     }
// }

// Shared navbar component.
// #[component]
// fn Navbar() -> Element {
//     rsx! {
//         div {
//             id: "navbar",
//             Link {
//                 to: Route::Home {},
//                 "Home"
//             }
//             Link {
//                 to: Route::Blog { id: 1 },
//                 "Blog"
//             }
//         }

//         Outlet::<Route> {}
//     }
// }

// /// Echo component that demonstrates fullstack server functions.
// #[component]
// fn Echo() -> Element {
//     let mut response = use_signal(|| String::new());

//     rsx! {
//         div {
//             id: "echo",
//             h4 { "ServerFn Echo" }
//             input {
//                 placeholder: "Type here to echo...",
//                 oninput:  move |event| async move {
//                     let data = echo_server(event.value()).await.unwrap();
//                     response.set(data);
//                 },
//             }

//             if !response().is_empty() {
//                 p {
//                     "Server echoed: "
//                     i { "{response}" }
//                 }
//             }
//         }
//     }
// }

// Echo the user input on the server.
// #[post("/api/echo")]
// async fn echo_server(input: String) -> Result<String, ServerFnError> {
//     Ok(input)
// }
