use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Draw vertical main road with continuous lines
    canvas.draw_line((400, 0), (400, 800)).unwrap();

    // Draw horizontal main road with continuous lines
    canvas.draw_line((0, 400), (800, 400)).unwrap();

    // Draw dashed vertical lanes
    for i in (250..600).step_by(50) {
        draw_dashed_line(canvas, i, 0, i, 275);
    }
    for i in (250..600).step_by(50) {
        draw_dashed_line(canvas, i, 550, i, 850);
    }

    // // Draw dashed horizontal lanes
    for i in (250..600).step_by(50) {
        draw_dashed_line(canvas, 0, i, 275, i);
    }
    for i in (250..600).step_by(50) {
        draw_dashed_line(canvas, 550, i, 850, i);
    }
}

fn draw_dashed_line(canvas: &mut Canvas<Window>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dash_length = 10;
    let gap_length = 10;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = ((dx * dx + dy * dy) as f64).sqrt() as i32;
    let dashes = (length / (dash_length + gap_length)) as i32;

    for i in 0..dashes {
        let start_x = x1 + i * (dash_length + gap_length) * dx / length;
        let start_y = y1 + i * (dash_length + gap_length) * dy / length;
        let end_x = start_x + dash_length * dx / length;
        let end_y = start_y + dash_length * dy / length;
        canvas.draw_line((start_x, start_y), (end_x, end_y)).unwrap();
    }
}
