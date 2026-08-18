use dioxus::{prelude::*};

pub const IMG_PR1: Asset = asset!("/assets/img/project-img1.png");
pub const IMG_PR2: Asset = asset!("/assets/img/project-img2.png");
pub const IMG_PR3: Asset = asset!("/assets/img/project-img3.png");
pub const IMG_PR4: Asset = asset!("/assets/img/project-img4.jpg");
pub const IMG_PR5: Asset = asset!("/assets/img/project-img5.png");
pub const IMG_PR6: Asset = asset!("/assets/img/project-img6.png");

// Project Box Props
#[derive(Props, Clone, PartialEq)]
pub struct ProjectBoxProps {
    pub slug: String,
    pub title: String,
    pub tech: Vec<String>,
    pub description: String,
    pub image: Asset,
    pub gitlink: String,
}

pub fn projects() -> Vec<ProjectBoxProps> {
    vec![
        ProjectBoxProps {
            slug: "laser-turret-project".to_string(),
            title: "Laser Turret Project".to_string(),
            description: "An affordable, fully autonomous laser turret that detects, tracks, and fires at targets using AI-based image processing. Built as a Mechatronics Engineering graduation project at Istanbul Ticaret University.".to_string(),
            image: IMG_PR1,
            gitlink: "https://github.com/denizhoroz/laser-turret-project".to_string(),
            tech: vec!["Python".to_string(), "FastAPI".to_string(), "Ultralytics YOLO".to_string(), "Javascript".to_string(), "Arduino (C/C++)".to_string()],
        },
        ProjectBoxProps {
            slug: "interloper".to_string(),
            title: "Interloper - Language Learning Platform".to_string(),
            description: "A language learning platform which you can practice in with simulated environments.".to_string(),
            image: IMG_PR2,
            gitlink: "https://github.com/denizhoroz/interloper".to_string(),
            tech: vec!["Python".to_string(), "Flask".to_string(), "Langchain".to_string(), "Javascript".to_string()],
        },
        ProjectBoxProps {
            slug: "journal-lite".to_string(),
            title: "Journal Lite".to_string(),
            description: "A lightweight Python journaling application designed to be fast, simple, and focused on daily entries with useful productivity features.".to_string(),
            image: IMG_PR3,
            gitlink: "https://github.com/denizhoroz/journal-lite".to_string(),
            tech: vec!["Python".to_string(), "Qt".to_string(), "SQLite".to_string()],
        },
        ProjectBoxProps {
            slug: "btk-datathon-2025".to_string(),
            title: "E-Commerce Session Value Prediction Project".to_string(),
            description: "Built an e-commerce prediction pipeline to generate features and parse data using Pandas, and applies random forest classifier to predict session value in BTK Datathon 2025.".to_string(),
            image: IMG_PR4,
            gitlink: "https://github.com/denizhoroz/btk-datathon-2025".to_string(),
            tech: vec!["Python".to_string(), "Pandas".to_string(), "Matplotlib".to_string(), "XGBoost".to_string(), "LightGBM".to_string(), "Scikit-learn".to_string()],
        },
        ProjectBoxProps {
            slug: "teknofest-address-resolution-2025".to_string(),
            title: "Address Resolution Project".to_string(),
            description: "Built an address resolution pipeline that processes noisy and unstructured address texts, then uses hybrid NLP pipelines to find an address value in TEKNOFEST 2025 Address Resolution Hackathon.".to_string(),
            image: IMG_PR5,
            gitlink: "https://github.com/denizhoroz/address-resolution".to_string(),
            tech: vec!["Python".to_string(), "Regex".to_string(), "NLP".to_string(), "BERT".to_string(), "TF-IDF".to_string(), "FAISS".to_string()],
        },
        ProjectBoxProps {
            slug: "fusionfm".to_string(),
            title: "FusionFM".to_string(),
            description: "A Windows 98 themed website which includes a radio for listening to music and a messaging app to chat with other people on the website.".to_string(),
            image: IMG_PR6,
            gitlink: "https://github.com/denizhoroz/fusionfm".to_string(),
            tech: vec!["Javascript".to_string(), "Node.js".to_string()],
        },
    ]
}