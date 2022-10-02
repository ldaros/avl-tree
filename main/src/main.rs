use avl::*;
use nannou::prelude::*;

const RADIUS: f32 = 30.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    user_value: String,
    tree: Tree<i32>,
}

fn model(_app: &App) -> Model {
    Model {
        user_value: String::new(),
        tree: Tree::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    match _app.keys.down.iter().next() {
        Some(Key::Return) => {
            if let Ok(value) = _model.user_value.parse::<i32>() {
                _model.tree.insert(value);
            }
            _model.user_value.clear();
        }
        Some(Key::Back) => {
            _model.user_value.pop();
        }
        Some(Key::Key1) => {
            _model.user_value.push('1');
        }
        _ => {}
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(WHITE);

    // draw the current typed value on the top of the screen
    let display = _app.draw();

    display
        .text(&_model.user_value.to_string())
        .color(BLACK)
        .x_y(0.0, 300.0)
        .font_size(50);

    display.to_frame(_app, &frame).unwrap();

    let tree_draw = _app.draw();
    let spacing = calculate_spacing(&_model.tree.root);
    let spacing_to_scale = spacing / (spacing * 1.5);
    let tree_draw = tree_draw.scale(spacing_to_scale);
    recursively_draw_tree::<i32>(&_model.tree.root, &tree_draw, 0.0, 300.0, spacing);

    tree_draw.to_frame(_app, &frame).unwrap();
}

fn recursively_draw_tree<T: Ord + std::fmt::Display>(
    root: &Option<Box<Node<T>>>,
    draw: &Draw,
    x: f32,
    y: f32,
    mut spacing: f32,
) {
    match root {
        Some(node) => {
            draw.ellipse()
                .x_y(x, y)
                .w_h(RADIUS * 2.0, RADIUS * 2.0)
                .color(BLACK);

            draw.text(&node.key.to_string())
                .x_y(x, y + 2.0)
                .color(WHITE)
                .font_size(20)
                .z(2.0);

            let lx = x - spacing;
            let ly = y - spacing;
            let rx = x + spacing;
            let ry = y - spacing;

            if node.left.is_some() {
                draw_line(draw, x, y, lx, ly);
            }

            if node.right.is_some() {
                draw_line(draw, x, y, rx, ry);
            }

            spacing = spacing / 2.0;

            recursively_draw_tree::<T>(&node.left, draw, lx, ly, spacing);
            recursively_draw_tree::<T>(&node.right, draw, rx, ry, spacing);
        }

        None => {}
    }
}

fn draw_line(draw: &Draw, x: f32, y: f32, lx: f32, ly: f32) {
    draw.line()
        .start(pt2(x, y))
        .end(pt2(lx, ly))
        .color(BLACK)
        .stroke_weight(2.0)
        .z(1.0);
}

fn calculate_spacing<T: Ord + std::fmt::Display>(root: &Option<Box<Node<T>>>) -> f32 {
    match root {
        Some(node) => {
            let left = calculate_spacing::<T>(&node.left);
            let right = calculate_spacing::<T>(&node.right);

            if left > right {
                left + RADIUS * 2.0
            } else {
                right + RADIUS * 2.0
            }
        }

        None => 0.0,
    }
}
