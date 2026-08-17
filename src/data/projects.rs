use dioxus::{prelude::*};

pub const IMG_PR1: Asset = asset!("/assets/img/project-img1.png");
pub const IMG_PR2: Asset = asset!("/assets/img/project-img2.png");
pub const IMG_PR3: Asset = asset!("/assets/img/project-img3.png");
pub const IMG_PR4: Asset = asset!("/assets/img/project-img4.jpg");
pub const IMG_PR5: Asset = asset!("/assets/img/project-img5.png");

// Project Box Props
#[derive(Props, Clone, PartialEq)]
pub struct ProjectBoxProps {
    pub slug: String,
    pub title: String,
    pub tech: Vec<String>,
    pub description: String,
    pub image: Asset,
    pub link: Option<String>,
}

pub fn projects() -> Vec<ProjectBoxProps> {
    vec![
        ProjectBoxProps {
            slug: "laser-turret-project".to_string(),
            title: "Laser Turret Project".to_string(),
            description: "An affordable, fully autonomous laser turret that detects, tracks, and fires at targets using AI-based image processing. Built as a Mechatronics Engineering graduation project at Istanbul Ticaret University.".to_string(),
            image: IMG_PR1,
            link: None,
            tech: vec!["Python".to_string(), "FastAPI".to_string(), "Ultralytics YOLO".to_string(), "Javascript".to_string(), "Arduino (C/C++)".to_string()],
        },
        ProjectBoxProps {
            slug: "interloper".to_string(),
            title: "Interloper - Language Learning Platform".to_string(),
            description: "A language learning platform which you can practice in with simulated environments.".to_string(),
            image: IMG_PR2,
            link: None,
            tech: vec!["Python".to_string(), "Flask".to_string(), "Langchain".to_string(), "Javascript".to_string()],
        },
        ProjectBoxProps {
            slug: "journal-lite".to_string(),
            title: "Journal Lite".to_string(),
            description: "A lightweight Python journaling application designed to be fast, simple, and focused on daily entries with useful productivity features.".to_string(),
            image: IMG_PR3,
            link: None,
            tech: vec!["Python".to_string(), "Qt".to_string(), "SQLite".to_string()],
        },
        ProjectBoxProps {
            slug: "btk-datathon-2025".to_string(),
            title: "E-Commerce Session Value Prediction Project".to_string(),
            description: "Built an e-commerce prediction pipeline to generate features and parse data using Pandas, and applies random forest classifier to predict session value in BTK Datathon 2025.".to_string(),
            image: IMG_PR4,
            link: None,
            tech: vec!["Python".to_string(), "Pandas".to_string(), "Matplotlib".to_string(), "XGBoost".to_string(), "LightGBM".to_string(), "Scikit-learn".to_string()],
        },
        ProjectBoxProps {
            slug: "teknofest-address-resolution-2025".to_string(),
            title: "Address Resolution Project".to_string(),
            description: "Built an address resolution pipeline that processes noisy and unstructured address texts, then uses hybrid NLP pipelines to find an address value in TEKNOFEST 2025 Address Resolution Hackathon.".to_string(),
            image: IMG_PR5,
            link: None,
            tech: vec!["Python".to_string(), "Regex".to_string(), "NLP".to_string(), "BERT".to_string(), "TF-IDF".to_string(), "FAISS".to_string()],
        },
    ]
}