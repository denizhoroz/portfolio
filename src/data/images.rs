use dioxus::prelude::*;

const IMG_PR1: Asset = asset!("/assets/img/project-img1.png");
const IMG_PR2: Asset = asset!("/assets/img/project-img2.png");
const IMG_PR3: Asset = asset!("/assets/img/project-img3.png");
const IMG_PR4: Asset = asset!("/assets/img/project-img4.jpg");
const IMG_PR5: Asset = asset!("/assets/img/project-img5.png");
const IMG_PR6: Asset = asset!("/assets/img/project-img6.png");

/// Resolves a `Project`'s `image_key` to a bundled asset.
///
/// `None` rather than a panic: once rows come from a database, an unknown key is
/// a data problem and must not take the page down.
pub fn asset_for(key: &str) -> Option<Asset> {
    match key {
        "project-img1" => Some(IMG_PR1),
        "project-img2" => Some(IMG_PR2),
        "project-img3" => Some(IMG_PR3),
        "project-img4" => Some(IMG_PR4),
        "project-img5" => Some(IMG_PR5),
        "project-img6" => Some(IMG_PR6),
        _ => None,
    }
}
