#![allow(dead_code, unused_variables)]
use dragon_curve::Sym::*;
use glam::Vec2;
use std::collections::HashMap;
use tiny_skia::{LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};
use dragon_curve::*;
use dragon_curve::DrawingCommand::{LineTo, MoveTo};
// dragon
// angle = 90
// START = FX+FX+
// X -> X+YF
// Y -> FX-Y

// Hilbert
// START = A
// A -> +BF-AFA-FB+
// B -> -AF+BFB+FA-

// where + == left turn angle
// F == move forward
// non-terminals are ignored during drawing

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start: Vec<Sym> = vec![F, X, Plus, F, X, Plus];
    let dragon_rules: HashMap<Sym, Vec<Sym>> =
        [
            (X, vec![X, Plus, Y, F]),
            (Y, vec![F, X, Minus, Y]),
        ].iter().cloned().collect();

    let mut l_string = start;
    for i in 0..12 {
        l_string = iterate(l_string, &dragon_rules);
    }
    let turtle_commands = to_turtle_commands(l_string);
    let drawing_commands = to_drawing_commands(
        Vec2::new(650.0, 480.0),
        Vec2::new(-1.0, 0.0),
        10.0,
        turtle_commands
    );
    let mut paint = Paint::default();
    paint.set_color_rgba8(100, 50, 127, 255);
    paint.anti_alias = true;

    let path = PixmapRenderer{}.render(drawing_commands).ok_or("RRender failure")?;
    let stroke = Stroke {
        width: 2.5,
        line_join: LineJoin::Miter,
        line_cap: tiny_skia::LineCap::Butt,
        ..Stroke::default()
    };
    let mut pixmap = Pixmap::new(960*2, 960).ok_or("Can't init pixmap")?;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.save_png("image.png")?;
    Ok(())
}

// fn render_to_pixbuffer(mut pb: PathBuilder, commands: Vec<DrawingCommand>) -> Path {
//     let pb = PathBuilder::new();
//     for command in commands {
//         match command {
//             LineTo(x, y) => pb.line_to(x, y),
//             MoveTo(x, y) => pb.move_to(x, y),
//         }
//     }
//     pb.finish().unwrap()
// }

trait Renderer<I,O> {
    fn render(&mut self, commands: Vec<I>) -> O;
} 

struct PixmapRenderer;
impl Renderer<DrawingCommand, Option<Path>> for PixmapRenderer {
    fn render(&mut self, commands: Vec<DrawingCommand>) -> Option<Path> {
        let mut pb = PathBuilder::new();
        for command in commands {
            match command {
                LineTo(x, y) => pb.line_to(x, y),
                MoveTo(x, y) => pb.move_to(x, y),
            }
        }
        pb.finish()
    }
}