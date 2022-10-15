//! This is a module which allows you to resize images. 
//! With this module it is possible to resize images with different algorithms.
pub mod image_resizer {
    use std::path::Path;


    /// This function allows you to resize images in a folder.
    pub fn resize_batch_from_folder(
        path_input_folder: String,
        nwidth: u32,
        nheight: u32,
        filter: image::imageops::FilterType,
        path_output_folder: String,
        only_shrink:bool
    ) -> bool {
        let paths = std::fs::read_dir(path_input_folder);
        let filter = filter;
        let paths = match paths {
            Ok(res) => res,
            Err(_res) => return false,
        };
        for dir_entry in paths {
            let cur_dir_entry = match dir_entry {
                Ok(res) => res,
                Err(_res) => continue,
            };
            let path_str = cur_dir_entry.path();
            if path_str.is_dir() {
                continue;
            };
            let stem = path_str.file_stem().unwrap();
            let stem_str = stem.to_str().unwrap();
            let extension = path_str.extension().unwrap();
            let extension_str = extension.to_str().unwrap();
            let new_file_name = format!("{}{}{}", stem_str, "_resize.",extension_str);
            let output_path = std::path::Path::new(&path_output_folder).join(new_file_name);
            let output_path_str = output_path.to_str().unwrap();
            resize(path_str.to_str().unwrap().to_string(),nwidth,nheight,filter,output_path_str.to_string(),only_shrink);
            
        }
        return true;
    }
    
    /// This function resize a single image with different algorithms
    /// returns [bool]
    pub fn resize(
        path: impl AsRef<Path>,
        nwidth: u32,
        nheight: u32,
        filter: image::imageops::FilterType,
        path_new_image: impl AsRef<Path>,
        only_shrink:bool
    ) -> bool {
        let decoded_image = load_image(path);
        let decoded_image = match decoded_image {
            Ok(res) => res,
            Err(_err) => return false,
        };

        if only_shrink && ((nwidth > decoded_image.width()) ||  (nheight > decoded_image.height())){
            return false
        }

        let resized_image = decoded_image.resize(nwidth, nheight, filter);
        let res = resized_image.save(path_new_image);
        match res {
            Ok(_res) => return true,
            Err(_err) => return false,
        }
    }

    /// This function load a single image
    fn load_image(path: impl AsRef<Path>) -> std::result::Result<image::DynamicImage,Box<dyn std::error::Error>> {
        let res_image = image::io::Reader::open(path);
        let res_image = match res_image {
            Ok(res) => res,
            Err(err) => return Err(Box::new(err)),
        };
        let res_image = res_image.decode();
        match res_image {
            Ok(res) => return Ok(res),
            Err(err) => return Err(Box::new(err)),
        };
    }

    /// This function returns a filter type
    fn _get_filter_type(filter: String) -> image::imageops::FilterType {
        let filter_type = match filter.as_str() {
            "Nearest" => image::imageops::FilterType::Nearest,
            "Triangle" => image::imageops::FilterType::Triangle,
            "CatmullRom" => image::imageops::FilterType::CatmullRom,
            "Gaussian" => image::imageops::FilterType::Gaussian,
            "Lanczos3" => image::imageops::FilterType::Lanczos3,
            _ => image::imageops::FilterType::Lanczos3,
        };
        return filter_type;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_nearest_image() {
        assert_eq!(
            super::image_resizer::resize(
                "tests/test_nearest_image/house-7313645_1920.jpg".to_string(),
                400,
                400,
                image::imageops::FilterType::Nearest,
                "tests/test_nearest_image/nearest.jpg".to_string(),
                true
            ),
            true
        );
    }
    #[test]
    fn test_triangle_image() {
        assert_eq!(
            super::image_resizer::resize(
                "tests/test_triangle_image/dog-7330712_1920.jpg".to_string(),
                400,
                400,
                image::imageops::FilterType::Triangle,
                "tests/test_triangle_image/triangle.jpg".to_string(),
                true
            ),
            true
        );
    }

    #[test]
    fn test_catmull_rom() {
        assert_eq!(
            super::image_resizer::resize(
                "tests/test_catmull_rom/building-7014904_1920.jpg".to_string(),
                400,
                400,
                image::imageops::FilterType::CatmullRom,
                "tests/test_catmull_rom/catmull_rom.jpg".to_string(),
                true
            ),
            true
        );
    }
    #[test]
    fn test_gaussian() {
        assert_eq!(
            super::image_resizer::resize(
                "tests/test_gaussian/guggenheim-3654928_1920.jpg".to_string(),
                400,
                400,
                image::imageops::FilterType::Gaussian,
                "tests/test_gaussian/gaussian.jpg".to_string(),
                true
            ),
            true
        )
    }
    #[test]
    fn test_lanczos3() {
        assert_eq!(
            super::image_resizer::resize(
                "tests/test_lanczos3/bilbao-2111993_1920.jpg".to_string(),
                400,
                400,
                image::imageops::FilterType::Lanczos3,
                "tests/test_lanczos3/lanczos3.jpg".to_string(),
                true
            ),
            true
        );
    }
    #[test]
    fn test_batch_lanczos3() {
        assert_eq!(super::image_resizer::resize_batch_from_folder(
            "tests/test_batch_lanczos3/".to_string(),
            400,
            400,
            image::imageops::FilterType::Lanczos3,
            "tests/test_batch_lanczos3/resize".to_string(),
            true
        ),true);
    }
    #[test]
    fn test_batch_nearest() {
        assert_eq!(super::image_resizer::resize_batch_from_folder(
            "tests/test_batch_nearest/".to_string(),
            400,
            400,
            image::imageops::FilterType::Nearest,
            "tests/test_batch_nearest/resize".to_string(),
            true
        ),true);
    }
    #[test]
    fn test_batch_triangle_image() {
        assert_eq!(super::image_resizer::resize_batch_from_folder(
            "tests/test_batch_triangle_image/".to_string(),
            400,
            400,
            image::imageops::FilterType::Triangle,
            "tests/test_batch_triangle_image/resize".to_string(),
            true
        ),true);
    }
    #[test]
    fn test_small_image() {
        assert_eq!(super::image_resizer::resize_batch_from_folder(
            "tests/test_small_image/".to_string(),
            800,
            600,
            image::imageops::FilterType::Triangle,
            "tests/test_small_image/resize".to_string(),
            true
        ),true);
    }
}
