use svg::{
    node::element::{
        path::Data, ClipPath, Definitions, LinearGradient, Path, Polyline, Rectangle, Stop,
    },
    Document,
};

const FILL: &str = "url(#bg)";

#[must_use]
pub fn generate_logo(width: f32) -> String {
    Document::new()
        .set("viewBox", "0 0 20 20")
        .add(create_defs())
        .add(create_bar(width, BarPosition::Left))
        .add(create_bar(width, BarPosition::Right))
        .add(create_v(width))
        .add(create_t(width))
        .to_string()
}

fn create_defs() -> Definitions {
    let gradient = LinearGradient::new()
        .set("id", "bg")
        .set("x1", 0.5)
        .set("y1", 0.0)
        .set("x2", 0.5)
        .set("y2", 1.0)
        .add(Stop::new().set("offset", "100%").set("stop-color", "white"));

    let clip_path = ClipPath::new().set("id", "cut-off-top").add(
        Rectangle::new()
            .set("x", "0")
            .set("y", "0")
            .set("width", "20")
            .set("height", "20"),
    );

    Definitions::new().add(clip_path).add(gradient)
}

fn create_bar(width: f32, postion: BarPosition) -> Path {
    let starting_x = match postion {
        BarPosition::Left => 0.0,
        BarPosition::Right => 20.0 - width,
    };

    let data = Data::new()
        .move_to((starting_x, 0))
        .horizontal_line_to(starting_x + width)
        .vertical_line_to(20)
        .horizontal_line_to(starting_x)
        .close();

    Path::new().set("d", data).set("fill", FILL)
}

fn create_v(width: f32) -> Polyline {
    let half_width = width / 2.0;
    let right_point = 20.0 - half_width;
    let points = format!("{half_width} 0 10 20 {right_point} 0");

    Polyline::new()
        .set("points", points)
        .set("fill", "none")
        .set("stroke", FILL)
        .set("stroke-width", width)
        .set("stroke-linecap", "square")
        .set("stoke-linejoin", "bevel")
        .set("clip-path", "url(#cut-off-top)")
}

fn create_t(width: f32) -> Path {
    let half_width = width / 2.0;
    let right_point = 20.0 - half_width;

    let mut left_line = Line::from_coordinates(half_width, 0.0, 10.0, 20.0);
    let mut right_line = Line::from_coordinates(right_point, 0.0, 10.0, 20.0);
    let offset = 2.5;
    left_line.intercept -= width * offset;
    right_line.intercept -= width * offset;

    let top_line = Line::from_coordinates(0.0, 0.0, 20.0, 0.0);

    let top_left_upper = find_intersection(&left_line, &top_line);
    let top_right_upper = find_intersection(&right_line, &top_line);
    let spike_bottom = find_intersection(&left_line, &right_line);

    let top_line = Line::from_coordinates(0.0, width, 20.0, width);
    let bottom_line =
        Line::from_coordinates(0.0, spike_bottom.1 - width, 20.0, spike_bottom.1 - width);

    let top_left_lower = find_intersection(&left_line, &top_line);
    let top_right_lower = find_intersection(&right_line, &top_line);
    let spike_left = find_intersection(&left_line, &bottom_line);
    let spike_right = find_intersection(&right_line, &bottom_line);

    let data = Data::new()
        .move_to(top_left_upper)
        .line_to(top_right_upper)
        .line_to(top_right_lower)
        .line_to((spike_right.0, width))
        .line_to(spike_right)
        .line_to(spike_bottom)
        .line_to(spike_left)
        .line_to((spike_left.0, width))
        .line_to(top_left_lower)
        .close();

    Path::new().set("d", data).set("fill", FILL)
}

struct Line {
    slope: f32,
    intercept: f32,
}

impl Line {
    fn from_coordinates(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let slope = (y2 - y1) / (x2 - x1);
        let intercept = y1 - slope * x1;

        Self { slope, intercept }
    }
}

fn find_intersection(line1: &Line, line2: &Line) -> (f32, f32) {
    let x = (line2.intercept - line1.intercept) / (line1.slope - line2.slope);
    let y = line1.slope * x + line1.intercept;

    (x, y)
}

#[derive(Clone, Copy)]
enum BarPosition {
    Left,
    Right,
}
