use pdf_writer::{types::TextRenderingMode, writers::ExtGraphicsState};
use printpdf::{graphics::{Line, Point}, units::Mm, PdfDocument};

static ROBOTO_TTF: &[u8] = include_bytes!("../assets/fonts/RobotoMedium.ttf");
static SVG: &str = include_str!("../assets/svg/tiger.svg");

fn main() {

    let mut doc = PdfDocument::new("My first document");

    // shape 1 (line)
    let line1 = Line {
        points: vec![
            (Point::new(Mm(100.0), Mm(100.0)), false),
            (Point::new(Mm(100.0), Mm(200.0)), false),
            (Point::new(Mm(300.0), Mm(200.0)), false),
            (Point::new(Mm(300.0), Mm(100.0)), false),
        ],
        is_closed: true,
    };
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));

    let mut ops = vec![
        Op::SetOutlineColor { color: outline_color },
        Op::SetOutlineThickness { pt: Pt(10.0) },
        Op::DrawLine { line: line1 },
    ];

    // shape 2 (polygon)
    let line2 = Polygon {
        rings: vec![vec![
            (Point::new(Mm(150.0), Mm(150.0)), false),
            (Point::new(Mm(150.0), Mm(250.0)), false),
            (Point::new(Mm(350.0), Mm(250.0)), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));
    let dash_pattern = LineDashPattern {
        dash_1: Some(20),
        ..Default::default()
    };

    ops.extend_from_slice(&[
        Op::SaveGraphicsState,
        Op::LoadGraphicsState { gs: doc.add_graphics_state(
            ExtGraphicsState::new_overprint_stroke(true) + 
            ExtGraphicsState::new_blend_mode(BlendMode::Multiply)
        ) },
        Op::SetLineDashPattern { dash: dash_pattern },
        Op::SetLineJoinStyle { join: LineJoinStyle::Round },
        Op::SetLineCapStyle { join: LineCapStyle::Round },
        Op::SetFillColor { color: fill_color_2 },
        Op::SetOutlineThickness { pt: Pt(15.0) },
        Op::RestoreGraphicsState,
    ]);

    // font loading
    let font_id = doc.add_external_font(ROBOTO_TTF).unwrap();
    
    current_layer.begin_text_section();

    ops.extend_from_slice(&[
        Op::StartTextSection,

        Op::SetFont { font: font_id.clone(), size: Pt(33.0) },
        Op::SetTextCursor { point: Point { x: Mm(10.0).into(), y: Mm(100.0).into() } }, // from bottom left
        Op::SetLineHeight { lh: Pt(33.0) },
        Op::SetWordSpacing { percent: 3000.0 },
        Op::SetCharacterSpacing { multiplier: 10.0 },
        
        Op::WriteText { text: "Lorem ipsum".to_string(), font: font_id.clone() },
        Op::AddLineBreak,
        Op::WriteText { text: "dolor sit amet".to_string(), font: font_id.clone() },
        Op::AddLineBreak,

        Op::SetTextRenderingMode { mode: TextRenderingMode::FillStroke },
        Op::SetCharacterSpacing { multiplier: 0.0 },
        Op::SetTextMatrix { matrix: TextMatrix::Rotate(10.0 /* degrees ccw */) },

        Op::WriteText { text: "Lorem ipsum".to_string(), font: font_id.clone() },
        Op::SetLineOffset { multiplier: 10.0 },
        Op::SetTextRenderingMode { mode: TextRenderingMode::Stroke },
        Op::SetFont { font: font_id.clone(), size: Pt(18.0) },
        Op::WriteText { text: "dolor sit amet".to_string(), font: font_id.clone() },

        Op::EndTextSection,
    ]);

    let svg = Svg::parse(SVG).unwrap();
    let xobject_id = doc.add_xobject(&svg);
    let transform = SvgTransform {
        rotate: Some(SvgRotation {
            angle_ccw_degrees: i as f32 * 36.0,
            rotation_center_x: rotation_center_x.into_pt(300.0),
            rotation_center_y: rotation_center_y.into_pt(300.0),
        }),
        translate_x: Some(Mm(i as f32 * 20.0 % 50.0).into()),
        translate_y: Some(Mm(i as f32 * 30.0).into()),
        ..Default::default()
    };

    ops.extend_from_slice(&[
        Op::InstantiateXObject { xobject_id: xobject_id, transform: transform }
    ]);

    let _bookmark_id = doc.add_bookmark("Chapter 1", /* page */ 0);

    // collect pages
    let pages = vec![
        PdfPage::new(Mm(210.0), Mm(297.0), ops)
    ];
    
    let bytes = doc.save_to_bytes().unwrap();
    std::fs::write("./simple.pdf", bytes);
}