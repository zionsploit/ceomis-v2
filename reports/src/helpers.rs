use std::io::Cursor;

use genpdf::{elements::{Image, LinearLayout, PaddedElement, Paragraph}, fonts::{self, FontData, FontFamily}, style, Alignment, Element, Margins, Size};
use image::imageops::FilterType;

pub fn left_text_paragraph<T: Element>(element: T) -> PaddedElement<T> {
    PaddedElement::new(
        element, Margins::trbl(0, 10, 0, 10))
}

pub fn font_family () -> FontFamily<FontData> {
    fonts::from_files("reports/assets/fonts/Roboto/static/", "Roboto_Condensed", None).expect("Failed to load fonts")
}


pub fn city_images () -> Image {
    Image::from_path("reports/assets/img/seal.jpg").expect("Failed to load seal images")
        .with_scale((0.5, 0.5))
}

pub fn app_icon_images () -> Image {
    Image::from_path("reports/assets/img/appIcon.jpg").expect("Failed to load app icon images")
        .with_scale((0.06, 0.06))
}

pub fn doc_title () -> LinearLayout {
    let mut title_layout = LinearLayout::vertical();
    
    let city_title = PaddedElement::new(Paragraph::new("City Government of Pagadian")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold), Margins::trbl(10, 0, 0, 0));

    let oce_title = Paragraph::new("Office of the City Engineer").aligned(Alignment::Center);

    let street_titel = Paragraph::new("Benigno Aquino St., Brgy. San Jose").aligned(Alignment::Center);

    title_layout.push(city_title);
    title_layout.push(oce_title);
    title_layout.push(street_titel);

    title_layout
}

pub fn convert_img_into_jpeg<T: AsRef<[u8]>>(object_bytes: T) -> Vec<u8> {
    let dyn_img = image::load_from_memory(object_bytes.as_ref()).expect("Failed to load images");

    let rgb_img = dyn_img.resize_exact(555, 555, FilterType::Lanczos3).to_rgb8();
    let mut rgb_buf = Vec::new();
    rgb_img.write_to(&mut Cursor::new(&mut rgb_buf), image::ImageFormat::Jpeg).unwrap();
    rgb_buf
}

pub enum PaperSize {
    A4LandScape,
    A4Portrait,
}

impl PaperSize {
    pub fn size(&self) -> Size {
        match self {
            PaperSize::A4LandScape => Size::new(297.0, 210.0),
            PaperSize::A4Portrait => Size::new(210.0, 297.0),
        }
    }
}