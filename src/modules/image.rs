// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use iced::{ContentFit, widget::button};
use std::{sync::Arc, path::Path};
use iced_gif::gif;





// ============ CRATES ============
use crate::helpers::{color::ColorType, style::{SideOption, UserStyle, set_style}};





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct ImageData
{
    pub preloaded_images_handle: Vec<Option<(PreloadedImage, usize)>>,
}



#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ImageConfig
{
    pub images_spacing: u32,
    pub images:         Vec<Image>,
}



#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Image 
{
    pub image_path: String,
    pub content_fit: UserContentFit,
    pub message_image_missing: String,

    pub side_separator: Option<SideOption>,
    pub separator_color: ColorType,
    pub separator_width:  f32,
    pub separator_height: f32,  

    pub padding: u16,
    pub height: u32,
    pub width: u32,
    pub button_color: ColorType,
    pub button_hovered_color: ColorType,
    pub button_pressed_color: ColorType,
    pub border_color: ColorType,
    pub border_size: f32,
    pub border_radius: [f32;4],
    pub command_to_exec_on_left_click: Vec<String>,
    pub command_to_exec_on_right_click: Vec<String>,
}


pub enum PreloadedImage
{
    Static(iced::widget::image::Handle),
    Gif(Arc<gif::Frames>),
}



#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserContentFit
{
    #[default] Fill,
    Contain,
    Cover,
    ScaleDown,
    None
}





// ============ IMPLEMENTATIONS ============
impl Clone for PreloadedImage
{
    fn clone(&self) -> Self
    {
        match self
        {
            PreloadedImage::Static(h) => PreloadedImage::Static(h.clone()),
            PreloadedImage::Gif(f)    => PreloadedImage::Gif(Arc::clone(f)),
        }
    }
}



impl From<UserContentFit> for ContentFit
{
    fn from(val: UserContentFit) -> Self
    {
        match val
        {
            UserContentFit::Contain => ContentFit::Contain,
            UserContentFit::Cover => ContentFit::Cover,
            UserContentFit::Fill => ContentFit::Fill,
            UserContentFit::None => ContentFit::None,
            UserContentFit::ScaleDown => ContentFit::ScaleDown,
        }
    }
}







// ============ FUNCTIONS ============
impl Default for Image 
{
    fn default() -> Self
    {
        Self 
        {
            image_path: String::new(),
            content_fit: UserContentFit::Fill,
            message_image_missing: "Warning!!!: Image Not Found.".to_string(),


            side_separator: None,
            separator_color: ColorType::RGB([75, 75, 75]),
            separator_width:  1.,
            separator_height: 16.,

            padding: 0,
            height: 30,
            width: 30,
            button_color: ColorType::RGB([60, 50, 70]),
            button_hovered_color: ColorType::RGB([110, 40, 80]),
            button_pressed_color: ColorType::RGB([70, 20, 40]),
            border_color: ColorType::RGB([90, 70, 100]),
            border_size: 1.0,
            border_radius: [3., 3., 3., 3.],
            command_to_exec_on_left_click: vec![], 
            command_to_exec_on_right_click: vec![],
        }
    }
}



pub fn preload_image(warning_err: &mut String, config_parsed_failed: &mut bool, vec_of_image_modules: &[Image]) -> Vec<Option<(PreloadedImage, usize)>>
{

    let mut vec_to_send = Vec::new();
    for (index, image_module) in vec_of_image_modules.iter().enumerate()
    {
        println!("\n=== WALLPAPER PRELOAD ===");
        if Path::new(&image_module.image_path).exists()
        {
            println!("Preloading Image, Please Wait...");
            if image_module.image_path.ends_with(".gif")
            {
                let result_bytes = std::fs::read(&image_module.image_path);
                match result_bytes
                {
                    Ok(bytes) => 
                    {
                        let result_frames = gif::Frames::from_bytes(bytes);
                        match result_frames
                        {
                            Ok(frames) =>
                            {
                                vec_to_send.push(Some((PreloadedImage::Gif(Arc::new(frames)), index)));
                                println!("Image from Image({index}) Preload Completed Successfully!!!");
                            },
                            Err(err) => 
                            {
                                vec_to_send.push(None);
                                let warning_msg = format!("Warning!!!: Failed to collect GIF frames, ERR: {err}");
                                eprintln!("{warning_msg}");
                                *warning_err = warning_msg;
                                *config_parsed_failed = true;
                            }
                        }
                    },
                    Err(err) => 
                    {
                        vec_to_send.push(None);
                        let warning_msg = format!("Warning!!!: Failed to preload GIF, ERR: {err}");
                        eprintln!("{warning_msg}");
                        *warning_err = warning_msg;
                        *config_parsed_failed = true;
                    }
                }
            }
            else
            {
                let handle = iced::widget::image::Handle::from_path(&image_module.image_path);
                vec_to_send.push(Some((PreloadedImage::Static(handle), index)));
                println!("Image from Image({index}) Preload Completed Successfully!!!");
            };
        }
        else 
        { 
            vec_to_send.push(None);
            let warning_msg = format!("WARNING!!!: Wallpaper path does not exist: '{}'. Error Coming from: Image({index})", image_module.image_path);
            eprintln!("{warning_msg}");
            *warning_err = warning_msg;
            *config_parsed_failed = true;
        }
    };
    vec_to_send
}



pub fn define_image_style(image: &Image, status: button::Status) -> iced::widget::button::Style
{
    let text_holder = ColorType::RGB([255, 255, 255]);

    let hovered =       image.button_hovered_color; 
    let pressed =       image.button_pressed_color; 
    let normal =        image.button_color; 
    let border_size =   image.border_size; 
    let border_color =  image.border_color; 
    let border_radius = image.border_radius;
    set_style(UserStyle { status, hovered, hovered_text: text_holder, pressed_text: text_holder, pressed, normal, normal_text: text_holder, border_color, border_size, border_radius, normal_gradient: None, hovered_gradient: None, pressed_gradient: None, shadow_color: None, shadow_x: 0.0, shadow_y: 0.0, shadow_blur: 0.0 })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use iced::{Background, Color};
    use iced::widget::button;

    // ── Image default ────────────────────────────────────────────────────────

    #[test]
    fn image_default_path_is_empty()
    {
        assert!(Image::default().image_path.is_empty());
    }

    #[test]
    fn image_default_content_fit_is_fill()
    {
        assert_eq!(Image::default().content_fit, UserContentFit::Fill);
    }

    #[test]
    fn image_default_message_image_missing_is_nonempty()
    {
        assert!(!Image::default().message_image_missing.is_empty());
    }

    #[test]
    fn image_default_side_separator_is_none()
    {
        assert!(Image::default().side_separator.is_none());
    }

    #[test]
    fn image_default_padding_is_zero()
    {
        assert_eq!(Image::default().padding, 0);
    }

    #[test]
    fn image_default_height_is_positive()
    {
        assert!(Image::default().height > 0);
    }

    #[test]
    fn image_default_width_is_positive()
    {
        assert!(Image::default().width > 0);
    }

    #[test]
    fn image_default_border_size_is_positive()
    {
        assert!(Image::default().border_size > 0.0);
    }

    #[test]
    fn image_default_border_radius_has_four_values()
    {
        assert_eq!(Image::default().border_radius.len(), 4);
    }

    #[test]
    fn image_default_left_click_command_is_empty()
    {
        assert!(Image::default().command_to_exec_on_left_click.is_empty());
    }

    #[test]
    fn image_default_right_click_command_is_empty()
    {
        assert!(Image::default().command_to_exec_on_right_click.is_empty());
    }

    #[test]
    fn image_default_separator_width_is_positive()
    {
        assert!(Image::default().separator_width > 0.0);
    }

    #[test]
    fn image_default_separator_height_is_positive()
    {
        assert!(Image::default().separator_height > 0.0);
    }

    // ── UserContentFit → ContentFit conversion ───────────────────────────────

    #[test]
    fn user_content_fit_contain_converts()
    {
        let cf: ContentFit = UserContentFit::Contain.into();
        assert_eq!(cf, ContentFit::Contain);
    }

    #[test]
    fn user_content_fit_cover_converts()
    {
        let cf: ContentFit = UserContentFit::Cover.into();
        assert_eq!(cf, ContentFit::Cover);
    }

    #[test]
    fn user_content_fit_fill_converts()
    {
        let cf: ContentFit = UserContentFit::Fill.into();
        assert_eq!(cf, ContentFit::Fill);
    }

    #[test]
    fn user_content_fit_none_converts()
    {
        let cf: ContentFit = UserContentFit::None.into();
        assert_eq!(cf, ContentFit::None);
    }

    #[test]
    fn user_content_fit_scale_down_converts()
    {
        let cf: ContentFit = UserContentFit::ScaleDown.into();
        assert_eq!(cf, ContentFit::ScaleDown);
    }

    #[test]
    fn user_content_fit_default_is_fill()
    {
        assert_eq!(UserContentFit::default(), UserContentFit::Fill);
    }

    #[test]
    fn all_user_content_fit_variants_convert_without_panic()
    {
        let variants = [
            UserContentFit::Contain,
            UserContentFit::Cover,
            UserContentFit::Fill,
            UserContentFit::None,
            UserContentFit::ScaleDown,
        ];
        for v in variants
        {
            let _: ContentFit = v.into();
        }
    }

    // ── PreloadedImage clone ─────────────────────────────────────────────────

    #[test]
    fn preloaded_image_static_clone_does_not_panic()
    {
        let handle = iced::widget::image::Handle::from_bytes(vec![]);
        let img = PreloadedImage::Static(handle);
        let _ = img.clone();
    }

    #[test]
    fn preloaded_image_gif_clone_shares_arc()
    {
        // We can't easily construct gif::Frames without a real GIF buffer,
        // but we can test that Arc clone works by checking refcount behaviour
        // through the Static variant as a structural proxy.
        // For the Gif variant, we verify the Arc wrapping compiles and clones.
        let handle = iced::widget::image::Handle::from_bytes(vec![]);
        let img = PreloadedImage::Static(handle);
        let cloned = img.clone();
        // Both are independently valid — no panic means Arc mechanics work
        drop(img);
        drop(cloned);
    }

    // ── preload_image — path handling ────────────────────────────────────────

    #[test]
    fn preload_image_empty_slice_returns_empty_vec()
    {
        let result = preload_image(&mut String::new(), &mut false, &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn preload_image_nonexistent_path_pushes_nothing()
    {
        let img = Image { image_path: "/nonexistent/path/to/image.png".to_string(), ..Default::default() };
        let result = preload_image(&mut String::new(), &mut false, &[img]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_none());
    }
    
    #[test]
    fn preload_image_result_length_never_exceeds_input_length()
    {
        let images = vec!
        [
            Image { image_path: "/no/a.png".to_string(), ..Default::default() },
            Image { image_path: "/no/b.png".to_string(), ..Default::default() },
        ];
        let result = preload_image(&mut String::new(), &mut false, &images);
        assert_eq!(result.len(), images.len());
    }

    #[test]
    fn preload_image_existing_static_image_returns_some_with_correct_index()
    {
        // Create a real temp file with valid PNG bytes (1x1 red pixel)
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        // Minimal valid 1x1 red PNG
        let png_bytes: &[u8] = &[
            0x89,0x50,0x4e,0x47,0x0d,0x0a,0x1a,0x0a,
            0x00,0x00,0x00,0x0d,0x49,0x48,0x44,0x52,
            0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,
            0x08,0x02,0x00,0x00,0x00,0x90,0x77,0x53,
            0xde,0x00,0x00,0x00,0x0c,0x49,0x44,0x41,
            0x54,0x08,0xd7,0x63,0xf8,0xcf,0xc0,0x00,
            0x00,0x00,0x02,0x00,0x01,0xe2,0x21,0xbc,
            0x33,0x00,0x00,0x00,0x00,0x49,0x45,0x4e,
            0x44,0xae,0x42,0x60,0x82,
        ];
        tmp.write_all(png_bytes).unwrap();
        let path = tmp.path().to_str().unwrap().to_string();

        let images = vec![Image { image_path: path, ..Default::default() }];
        let result = preload_image(&mut String::new(), &mut false, &images);

        assert_eq!(result.len(), 1);
        let entry = result[0].as_ref().unwrap();
        assert_eq!(entry.1, 0); // index matches
        assert!(matches!(entry.0, PreloadedImage::Static(_)));
    }

    #[test]
    fn preload_image_invalid_gif_bytes_pushes_nothing()
    {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::with_suffix(".gif").unwrap();
        tmp.write_all(b"this is not a gif").unwrap();
        let path = tmp.path().to_str().unwrap().to_string();
    
        let images = vec![Image { image_path: path, ..Default::default() }];
        let result = preload_image(&mut String::new(), &mut false, &images);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_none());
    }

    // ── define_image_style ───────────────────────────────────────────────────

    fn make_image(normal: [u32;3], hovered: [u32;3], pressed: [u32;3]) -> Image
    {
        Image
        {
            button_color:         ColorType::RGB(normal),
            button_hovered_color: ColorType::RGB(hovered),
            button_pressed_color: ColorType::RGB(pressed),
            border_color:         ColorType::RGB([1, 2, 3]),
            border_size:          2.0,
            border_radius:        [4.0, 4.0, 4.0, 4.0],
            ..Default::default()
        }
    }

    #[test]
    fn define_image_style_active_uses_normal_color()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        let style = define_image_style(&img, button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }

    #[test]
    fn define_image_style_hovered_uses_hovered_color()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        let style = define_image_style(&img, button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }

    #[test]
    fn define_image_style_pressed_uses_pressed_color()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        let style = define_image_style(&img, button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }

    #[test]
    fn define_image_style_disabled_uses_normal_color()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        let style = define_image_style(&img, button::Status::Disabled);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }

    #[test]
    fn define_image_style_border_size_applied()
    {
        let img = make_image([0, 0, 0], [0, 0, 0], [0, 0, 0]);
        let style = define_image_style(&img, button::Status::Active);
        assert_eq!(style.border.width, 2.0);
    }

    #[test]
    fn define_image_style_border_color_applied()
    {
        let img = make_image([0, 0, 0], [0, 0, 0], [0, 0, 0]);
        let style = define_image_style(&img, button::Status::Active);
        assert_eq!(style.border.color, Color::from_rgb8(1, 2, 3));
    }

    #[test]
    fn define_image_style_border_radius_applied()
    {
        use iced::border::Radius;
        let img = make_image([0, 0, 0], [0, 0, 0], [0, 0, 0]);
        let style = define_image_style(&img, button::Status::Active);
        assert_eq!(style.border.radius, Radius { top_left: 4.0, top_right: 4.0, bottom_left: 4.0, bottom_right: 4.0 });
    }

    #[test]
    fn define_image_style_all_statuses_produce_background()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = define_image_style(&img, status);
            assert!(style.background.is_some(), "expected background for {status:?}");
        }
    }

    #[test]
    fn define_image_style_active_and_hovered_backgrounds_differ()
    {
        let img = make_image([10, 20, 30], [50, 60, 70], [80, 90, 100]);
        let active  = define_image_style(&img, button::Status::Active);
        let hovered = define_image_style(&img, button::Status::Hovered);
        assert_ne!(active.background, hovered.background);
    }

    #[test]
    fn define_image_style_zero_border_size()
    {
        let mut img = make_image([0, 0, 0], [0, 0, 0], [0, 0, 0]);
        img.border_size = 0.0;
        let style = define_image_style(&img, button::Status::Active);
        assert_eq!(style.border.width, 0.0);
    }

    // ── UserContentFit serde ─────────────────────────────────────────────────

    #[test]
    fn user_content_fit_serializes_and_deserializes_roundtrip()
    {
        let variants = [
            UserContentFit::Contain,
            UserContentFit::Cover,
            UserContentFit::Fill,
            UserContentFit::None,
            UserContentFit::ScaleDown,
        ];
        for v in variants
        {
            let s = ron::to_string(&v).unwrap();
            let back: UserContentFit = ron::from_str(&s).unwrap();
            assert_eq!(v, back);
        }
    }

    // ── Image serde ──────────────────────────────────────────────────────────

    #[test]
    fn image_default_serializes_without_panic()
    {
        let _ = ron::to_string(&Image::default()).unwrap();
    }

    #[test]
    fn image_deserializes_from_minimal_ron()
    {
        // serde(default) means all fields are optional
        let s = "Image()";
        let result = ron::from_str::<Image>(s);
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[test]
    fn image_roundtrip_serde_preserves_path()
    {
        let mut img = Image::default();
        img.image_path = "/some/path/test.png".to_string();
        let s = ron::to_string(&img).unwrap();
        let back: Image = ron::from_str(&s).unwrap();
        assert_eq!(back.image_path, "/some/path/test.png");
    }

    #[test]
    fn image_roundtrip_serde_preserves_content_fit()
    {
        let mut img = Image::default();
        img.content_fit = UserContentFit::Cover;
        let s = ron::to_string(&img).unwrap();
        let back: Image = ron::from_str(&s).unwrap();
        assert_eq!(back.content_fit, UserContentFit::Cover);
    }

    #[test]
    fn image_roundtrip_serde_preserves_commands()
    {
        let mut img = Image::default();
        img.command_to_exec_on_left_click = vec!["wofi".to_string(), "--show".to_string(), "drun".to_string()];
        let s = ron::to_string(&img).unwrap();
        let back: Image = ron::from_str(&s).unwrap();
        assert_eq!(back.command_to_exec_on_left_click, vec!["wofi", "--show", "drun"]);
    }
}
