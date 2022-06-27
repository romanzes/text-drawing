use skia_safe::paint::Style;
use skia_safe::textlayout::{
    FontCollection, ParagraphBuilder, ParagraphStyle, RectHeightStyle, RectWidthStyle, TextStyle,
    TypefaceFontProvider,
};
use skia_safe::wrapper::ValueWrapper;
use skia_safe::{Color, Data, Font, FontMgr, FontStyle, ISize, Paint, Surface, TextBlob, Typeface};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    runic_text();
}

fn runic_text() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(32.0);
    text_style.set_font_families(&vec!["Open Sans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans = Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("Open Sans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    font_collection.set_default_font_manager(Some(FontMgr::default()), None);
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "ᚦᚨᛈᚨᚾᚨ";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/runic_text.png",
    );
    // panic!();
}

fn georgian_uppercase() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(26.6667);
    text_style.set_font_families(&vec!["Noto Sans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("NotoSansGeorgian-Bold.woff2")), None).unwrap();
    typeface_provider.register_typeface(font, Some("Noto Sans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "კვერცხუჯრედის\n".to_uppercase();
    println!("{}", text);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(320.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/georgian_uppercase.png",
    );
    panic!();
}

fn disappearing_letter() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(26.6667);
    text_style.set_font_families(&vec!["Open Sans Light"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Light.ttf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("Open Sans Light"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "1\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(15.218);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/disappearing_letter.png",
    );
    panic!();
}

fn totally_missing_glyphs() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(40.0);
    text_style.set_font_families(&vec!["Test", "Fallback"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("Adigiana_Ultra.ttf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("Test"));
    let font =
        Typeface::from_data(data_from_file_path(Path::new("NotoSans_CJK_SC.otf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("Fallback"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    font_collection.disable_font_fallback();
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    // let text = "訮鿪";
    // let text = "H鿪";
    let text = "一鿯";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(320.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/totally_missing_glyphs.png",
    );
}

fn text_without_layout() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(0, 136, 0));
    paint.set_style(Style::Stroke);
    paint.set_stroke_width(1.0);

    let adlery = Typeface::from_data(data_from_file_path(Path::new("Adlery.woff2")), None).unwrap();
    let blob = TextBlob::from_str("Skia!", &Font::new(adlery, 50.0)).unwrap();

    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    surface.canvas().draw_text_blob(blob, (0.0, 50.0), &paint);

    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_without_layout.png",
    );
}

fn chinese_shifting() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1000, 1000)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(40.0);
    text_style.set_font_families(&vec!["SourceHan-Sans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("SourceHan-Sans.ttf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("SourceHan-Sans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "         豎汽沟笨鹦险薨，莶迌砗枑翧隧庎覨：厉柯、吆妩縩、遡蹬劼、縘溙尟摱，咦掽苵瘔芣嬑鎊衢鯁憥訾馍敖貏膸韋蠏硠羵廢龐磹。\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1000.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/chinese_shifting.png",
    );
    panic!();
}

fn test_arimo_woff2() {
    let font =
        Typeface::from_data(data_from_file_path(Path::new("Arimo-Regular.woff2")), None).unwrap();
}

fn metrics_sigsegv() {
    let mut style = ParagraphStyle::new();
    style.set_text_style(&TextStyle::new());
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), None);
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text("Lorem ipsum dolor sit amet\n");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(100.0);

    let line_metrics = &paragraph.get_line_metrics()[0];
    line_metrics.get_style_metrics(line_metrics.start_index..line_metrics.end_index);
}

fn multi_line_end_spaces() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1000, 1000)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(18.666666666666668);
    text_style.set_letter_spacing(1.8666666666666668);
    text_style.set_font_families(&vec!["OpenSans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("OpenSans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Two lines with spaces  at the ends \n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(242.48);

    paragraph.get_line_metrics().iter().for_each(|metrics| {
        println!(
            "line start: {}, end: {}",
            metrics.start_index, metrics.end_index
        )
    });

    let line_metrics = &paragraph.get_line_metrics()[0];
    line_metrics.get_style_metrics(0..23)[0];

    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/multi_line_end_spaces.png",
    );
    panic!();
}

fn devanagari_test() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(65.0);
    text_style.set_font_families(&vec!["Adlery", "NotoSansDevanagari"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let adlery = Typeface::from_data(data_from_file_path(Path::new("Adlery.woff2")), None).unwrap();
    let noto_sans = Typeface::from_data(
        data_from_file_path(Path::new("NotoSansDevanagari-Regular.woff2")),
        None,
    )
    .unwrap();
    typeface_provider.register_typeface(adlery, Some("Adlery"));
    typeface_provider.register_typeface(noto_sans, Some("NotoSansDevanagari"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text("लि鿪ख\n");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/devanagari_test.png",
    );
}

fn get_ascent_from_font() {
    let typeface =
        Typeface::from_data(data_from_file_path(Path::new("LeagueSpartan.woff2")), None).unwrap();
    let font = Font::from_typeface(typeface, Some(1.0));
    let (_, metrics) = font.metrics();
    println!("ascent: {}", metrics.ascent);
}

fn spaces_with_different_style() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(18.6667);
    text_style.set_font_families(&vec!["OpenSans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(font, Some("OpenSans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text("Lorem ipsum ");

    let mut text_style_2 = TextStyle::new();
    text_style_2.set_color(Color::from_rgb(0, 0, 0));
    text_style_2.set_font_size(18.6667);
    text_style_2.set_font_families(&vec!["OpenSans"]);
    text_style_2.set_font_style(FontStyle::bold());
    paragraph_builder.push_style(&text_style_2);
    paragraph_builder.add_text("   \n");

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(320.0);

    let boxes = paragraph.get_rects_for_range(0..16, RectHeightStyle::Max, RectWidthStyle::Tight);
    let box_slice = boxes.as_slice();
    box_slice.iter().for_each(|bx| {
        println!("box left: {}, right: {}", bx.rect.left, bx.rect.right);
    });
    let left = box_slice.first().map(|t| t.rect.left as f64).unwrap_or(0.0);
    let right = box_slice.last().map(|t| t.rect.right as f64).unwrap_or(0.0);
    println!("left: {}, right: {}", left, right);

    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/spaces_with_different_style.png",
    );
    panic!();
}

fn locale_test() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(40.0);
    text_style.set_font_families(&vec!["NotoSansSC"]);
    text_style.set_locale("ja-JP");
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font = Typeface::from_data(
        data_from_file_path(Path::new("NotoSans_CJK_SC.woff2")),
        None,
    )
    .unwrap();
    typeface_provider.register_typeface(font, Some("NotoSansSC"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "餂侼\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(320.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/locale_test.png",
    );
    panic!();
}

fn letter_spacing() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1000, 1000)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(25.0099);
    // text_style.set_letter_spacing(-0.007);
    text_style.set_font_families(&vec!["Arimo"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let font = Typeface::from_data(data_from_file_path(Path::new("Arimo.woff2")), None).unwrap();
    typeface_provider.register_typeface(font, Some("Arimo"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1000.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/letter_spacing_2/test.png",
    );
    panic!();
}

fn twemoji() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1024, 1024)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    // text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(100.0);
    text_style.set_font_families(&vec!["Twemoji"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let typeface = Typeface::from_data(
        data_from_file_path(Path::new("TwitterColorEmoji.ttf")),
        None,
    )
    .unwrap();
    typeface_provider.register_typeface(typeface, Some("Twemoji"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "❤️‍🔥🧔🧔‍♀️🧔‍♂️\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1024.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(&mut surface, "/Users/romanpetrenko/Downloads/twemoji.png");
    panic!();
}

fn text_shifting_after_accent() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1024, 1024)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(60.0);
    text_style.set_font_families(&vec!["Adigiana"]);
    text_style.set_letter_spacing(20.0);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("Adigiana_Ultra.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("Adigiana"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "АБВГДЕЖЗИЍКЛ\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1024.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_shifting_after_accent.png",
    );
    panic!();
}

fn box_character_github_friendly() {
    let mut file = File::open(Path::new("NotoSansBold.otf")).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let data = Data::new_copy(&bytes.as_slice());
    let font = Typeface::from_data(data, None).unwrap();

    let mut typeface_provider = TypefaceFontProvider::new();
    typeface_provider.register_typeface(font, Some("NotoSansBold"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    font_collection.set_default_font_manager(Some(FontMgr::default()), None);

    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(60.0);
    text_style.set_font_families(&vec!["NotoSansBold"]);
    style.set_text_style(&text_style);

    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Test ￭月 ⚀ ⚁ ⚂ ⚃ ⚄ ⚅ 😁\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1024.0);

    let mut surface = Surface::new_raster_n32_premul(ISize::new(1024, 1024)).unwrap();
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/box_character_github_friendly.png",
    );
    panic!();
}

fn text_shifting_after_accent_github_friendly() {
    let mut file = File::open(Path::new("Adigiana_Ultra.ttf")).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let data = Data::new_copy(&bytes.as_slice());
    let font = Typeface::from_data(data, None).unwrap();

    let mut typeface_provider = TypefaceFontProvider::new();
    typeface_provider.register_typeface(font, Some("Adigiana"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(60.0);
    text_style.set_font_families(&vec!["Adigiana"]);
    text_style.set_letter_spacing(20.0);
    style.set_text_style(&text_style);

    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "АБВГДЕЖЗИЍКЛ\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1024.0);

    let mut surface = Surface::new_raster_n32_premul(ISize::new(1024, 1024)).unwrap();
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_shifting_after_accent.png",
    );
    panic!();
}

fn thai_text() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 320)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(29.3333);
    text_style.set_font_families(&vec!["OpenSans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("NotoSansThai.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("OpenSans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "อีกทั้งเป็นที่อยู่อาศัยของ\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(320.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/thai_text/thai_text.png",
    );
    panic!();
}

fn no_end_line_break_wrapping() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(32.0);
    text_style.set_font_families(&vec!["OpenSans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("OpenSans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Lorem ipsum";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(280.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/no_end_line_break_wrapping.png",
    );
    panic!();
}

fn text_wrapping() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(1000, 1000)).unwrap();
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(32.0);
    text_style.set_font_families(&vec!["OpenSans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("OpenSans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection.clone());
    let text = "Lorem ipsum/dolor\nLorem ipsum?dolor\nLorem ipsum,dolor\nLorem ipsum.dolor\nLorem ipsum<dolor\nLorem ipsum>dolor\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(230.0);
    paragraph.paint(surface.canvas(), skia_safe::Point::new(0.0, 0.0));

    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_wrapping.png",
    );
    panic!();
}

fn accented_text() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(61.3333);
    text_style.set_font_families(&vec!["Aileron"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("Aileron.woff2")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("Aileron"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Lorem Ipsum";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(309.22);
    let boxes =
        paragraph.get_rects_for_range(0..text.len(), RectHeightStyle::Max, RectWidthStyle::Tight);
    let box_slice = boxes.as_slice();
    box_slice.iter().for_each(|bx| {
        println!("box left: {}, right: {}", bx.rect.left, bx.rect.right);
    });
    let left = box_slice.first().map(|t| t.rect.left as f64).unwrap_or(0.0);
    let right = box_slice.last().map(|t| t.rect.right as f64).unwrap_or(0.0);
    println!("left: {}, right: {}", left, right);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/accented_text.png",
    );
    // panic!();
}

fn text_positioning() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(48.0);
    text_style.set_font_families(&vec!["Jua"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans = Typeface::from_data(data_from_file_path(Path::new("Jua.woff2")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("Jua"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Lorem ipsum\n";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/no_vertical_shift_2.png",
    );
    // panic!();
}

fn system_font_fallback() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(32.0);
    // text_style.set_font_families(&vec!["Open Sans"]);
    style.set_text_style(&text_style);
    // let mut typeface_provider = TypefaceFontProvider::new();
    // let open_sans = Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    // typeface_provider.register_typeface(open_sans, Some("Open Sans"));
    let mut font_collection = FontCollection::new();
    // font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    font_collection.set_default_font_manager(Some(FontMgr::default()), None);
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "⚀ ⚁ ⚂ ⚃ ⚄ ⚅ 😁";
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_drawing.png",
    );
    // panic!();
}

fn text_measuring() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(37.3333);
    text_style.set_letter_spacing(0.746665626667);
    text_style.set_font_families(&vec!["Open Sans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let open_sans =
        Typeface::from_data(data_from_file_path(Path::new("OpenSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(open_sans, Some("Open Sans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    let text = "Lorëm ipsum ";
    paragraph_builder.add_text(text);
    text_style.set_font_style(FontStyle::italic());
    paragraph_builder.push_style(&text_style);
    let text2 = "\n";
    paragraph_builder.add_text(text2);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let boxes = paragraph.get_rects_for_range(
        0..text.len() + text2.len(),
        RectHeightStyle::Max,
        RectWidthStyle::Tight,
    );
    let box_slice = boxes.as_slice();
    box_slice.iter().for_each(|bx| {
        println!("box left: {}, right: {}", bx.rect.left, bx.rect.right);
    });
    let left = box_slice.first().map(|t| t.rect.left as f64).unwrap_or(0.0);
    let right = box_slice.last().map(|t| t.rect.right as f64).unwrap_or(0.0);
    println!("left: {}, right: {}", left, right);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_drawing.png",
    );
    // panic!();
}

fn fallback_test() {
    let mut surface = Surface::new_raster_n32_premul(ISize::new(320, 240)).unwrap();
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_color(Color::from_rgb(0, 0, 0));
    text_style.set_font_size(58.666666666666664);
    text_style.set_font_families(&vec!["Montalaq", "Noto Sans"]);
    style.set_text_style(&text_style);
    let mut typeface_provider = TypefaceFontProvider::new();
    let montalaq =
        Typeface::from_data(data_from_file_path(Path::new("Montalaq.ttf")), None).unwrap();
    let noto_sans =
        Typeface::from_data(data_from_file_path(Path::new("NotoSans-Regular.ttf")), None).unwrap();
    typeface_provider.register_typeface(montalaq, Some("Montalaq"));
    typeface_provider.register_typeface(noto_sans, Some("Noto Sans"));
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text("AA AA");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(500.0);
    let point = skia_safe::Point::new(0.0, 0.0);
    surface.canvas().clear(Color::from_rgb(255, 255, 255));
    paragraph.paint(surface.canvas(), point);
    save_png(
        &mut surface,
        "/Users/romanpetrenko/Downloads/text_drawing.png",
    );
}

pub fn data_from_file_path(file_path: &Path) -> Data {
    let mut file = File::open(file_path).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    Data::new_copy(&bytes.as_slice())
}

pub fn save_png(surface: &mut Surface, path: &str) -> bool {
    let mut bytes: Vec<u8> = vec![];
    let image_info = surface.image_info();
    {
        let mut encoder = png::Encoder::new(
            &mut bytes,
            image_info.width() as u32,
            image_info.height() as u32,
        );
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().expect("failed to write file header");

        let mut dst_pixels = vec![0; image_info.height() as usize * image_info.min_row_bytes()];
        let pixels_read = surface.read_pixels(
            &image_info,
            &mut dst_pixels,
            image_info.min_row_bytes(),
            (0, 0),
        );
        if !pixels_read {
            println!("failed to read pixels");
        }
        let result = writer.write_image_data(dst_pixels.as_slice());
        if let Err(reason) = result {
            println!("failed to write image data: {}", reason);
        }
    }
    let data = skia_safe::Data::new_copy(&bytes);

    let mut file = std::fs::File::create(path).expect("failed to create the file");
    file.write_all(data.as_bytes())
        .expect("failed to write data to the file");

    return true;
}
