use dioxus::{html::{a::href, h1, img}, prelude::*};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[route("/")]
    Home {},

    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

// Import assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const IMG_PR1: Asset = asset!("/assets/img/project-img1.png");
const IMG_PR2: Asset = asset!("/assets/img/project-img2.png");
const IMG_PR3: Asset = asset!("/assets/img/project-img3.png");
const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");
const LINKEDIN_ICON: Asset = asset!("/assets/icons/linkedin.svg");
const X_ICON: Asset = asset!("/assets/icons/x.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Google+Sans:ital,opsz,wght@0,17..18,400..700;1,17..18,400..700&display=swap"
        }

        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column;",
            Hero {}
            AboutMe {}
            MyWorks {}
            ReachMe {}
            Footer {}
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
                h3 {
                    class: "block-desc",
                    "Hello, I'm Deniz. I develop helpful apps to solve people's problems."
                }
                h3 {
                    class: "block-desc",
                    "I've always wanted to learn more about technology and since I've started my engineering journey in 2022, I am constantly trying to learn new things and apply the things I've learned by making new projects."
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
                    
                    a {
                        href: "_blank",
                        {ProjectBox(ProjectBoxProps { 
                            title: ("Laser Turret Project".to_string()), 
                            description: ("An affordable, fully autonomous laser turret that detects, tracks, and fires at targets using AI-based image processing. Built as a Mechatronics Engineering graduation project at Istanbul Ticaret University.".to_string()), 
                            image: (IMG_PR1), 
                            link: (None) 
                        })}
                    }
                    a {
                        href: "_blank",
                        {ProjectBox(ProjectBoxProps { 
                            title: ("Interloper - Language Learning Platform".to_string()), 
                            description: ("A language learning platform which you can practice in with simulated environments.".to_string()), 
                            image: (IMG_PR2), 
                            link: (None) 
                        })}
                    }
                    a {
                        href: "_blank",
                        {ProjectBox(ProjectBoxProps { 
                            title: ("Journal Lite".to_string()), 
                            description: ("A lightweight Python journaling application designed to be fast, simple, and focused on daily entries with useful productivity features.".to_string()), 
                            image: (IMG_PR3), 
                            link: (None) 
                        })}
                    }
                }

                a {
                    class: "block-desc button",
                    href: "_blank",
                    "see all projects"
                }
            }
        }
    }
}


// ReachMe section
#[component]
pub fn ReachMe() -> Element {
    rsx! {
        div {
            id: "reachme",
            class: "page-block",
            div {
                style: "display: flex; flex-direction: column; gap: 50px",
                
                h1 {
                    class: "block-title",
                    "reach me"
                }
                div {
                    style: "display: flex; flex-direction: row; gap: 200px",
                    div {
                        style: "display: flex; flex-direction: column; gap: 20px;",
                        h1 { 
                            class: "block-subtitle",
                            "my email"
                        }
                        a {
                            class: "social-anchor",
                            "denizhoroz.ofcl@gmail.com"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 20px;",
                        h1 {
                            class: "block-subtitle",
                            "my socials"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://github.com/denizhoroz",
                            target: "_blank",
                            img { style: "width: 40px", src: GITHUB_ICON, alt: "GitHub", class: "social-icon"} 
                            "github"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://www.linkedin.com/in/denizhoroz",
                            target: "_blank",
                            img { style: "width: 40px", src: LINKEDIN_ICON, alt: "LinkedIn", class: "social-icon"} 
                            "linkedin"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://x.com/denizerenhoroz",
                            target: "_blank",
                            img { style: "width: 40px", src: X_ICON, alt: "X", class: "social-icon"} 
                            "X"
                        }
                    }
                }
            }
        }
    }
} 

// Footer section
#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            style: "margin: auto; justify-content: center; align-items: center; text-align: center;",
            "denizhoroz - 2026"
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

            img { 
                class: "image-portrait",
                style: "height: 200px; object-fit: cover;",
                src: props.image 
            }
            
            h3 { 
                style: "font-size: 24px; font-weight: bold",
                "{props.title}" 
            }
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
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "navbar-container",
        
            div {
                class: "navbar-title",
                h1 { "denizhoroz" }
            }

            div {
                id: "navbar",
                a { href: "#hero", "home" }
                a { href: "#aboutme", "who am i?" }
                a { href: "#myworks", "my works" }
                a { href: "#reachme", "reach me" }
            }
        }
        Outlet::<Route> {}
    }
}

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
