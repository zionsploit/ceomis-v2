use printpdf::{ImageCompression, ImageOptimizationOptions, PdfWarnMsg, RawImage};

pub struct Image {
    pub image_raw: RawImage
}

impl Image {
    pub fn new(image: &Vec<u8>) -> Self {

        let image_warning = PdfWarnMsg {
            msg: "Failed to Initialize Image".to_string(),
            op_id: 0,
            page: 1,
            severity: printpdf::PdfParseErrorSeverity::Error
        };

        let image = RawImage::decode_from_bytes(&image, &mut vec![image_warning]).unwrap();

        Self { image_raw: image }
    }

    pub fn into_jpeg(&self) -> Self {

        let mut image = self.image_raw.clone();

        image.optimize(&ImageOptimizationOptions {
            format: Some(ImageCompression::Jpeg),
            auto_optimize: Some(true),
            ..Default::default()
        }).unwrap();

        Self { image_raw: image }
        
    }

    pub fn set_width (&self, w: usize) -> Self {
        let mut images = self.image_raw.clone();
        images.width = w;

        Self { image_raw: images }
    }

    pub fn set_height(&self, h: usize) -> Self {
        let mut images = self.image_raw.clone();
        images.height = h;

        Self { image_raw: images }
    }
}

