use crate::day1::task2::get_elves;
use crate::utils::load_input;
use itertools::Itertools;
use nannou::prelude::*;
use std::time::Duration;

pub fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

trait Nannou {
    fn display(&self, draw: &Draw);
    fn update(&mut self);
}

struct Elf {
    snacks: Vec<usize>,
    position: Point2,
}

impl Nannou for Elf {
    fn display(&self, draw: &Draw) {
        let position = self.position;
        draw.ellipse()
            .x_y(position.x, position.y)
            .radius(
                self.snacks
                    .iter()
                    .map(|snack| *snack as f32 / 10000.0)
                    .sum::<f32>(),
            )
            .color(WHITE);
    }

    fn update(&mut self) {}
}

struct Model {
    elves: Vec<Elf>,
    time_since_last_selection: Duration,
    top_3_elf: usize,
    found_count: usize,
}

fn model(_app: &App) -> Model {
    let input = load_input(1);
    let elves = get_elves(&input);

    Model {
        time_since_last_selection: Duration::from_secs(0),
        found_count: 0,
        top_3_elf: elves
            .clone()
            .into_iter()
            .map(|elf| elf.iter().sum())
            .sorted()
            .rev()
            .take(3)
            .rev()
            .next()
            .unwrap(),
        elves: elves
            .into_iter()
            .enumerate()
            .map(|(index, snacks)| Elf {
                snacks,
                position: Point2::new(
                    (-300 + ((index % 30) as isize * 15)) as f32,
                    -300.0 + (index / 30) as f32 * 15.0,
                ),
            })
            .collect(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.time_since_last_selection < update.since_start + Duration::from_secs(1)
        && model.elves.len() > model.found_count
    {
        model.time_since_last_selection = update.since_start;
        if model.top_3_elf
            <= model
                .elves
                .get(model.found_count)
                .unwrap()
                .snacks
                .iter()
                .sum()
        {
            model.found_count += 1;
        } else {
            model.elves.remove(model.found_count);
        }
    }
    model.elves.iter_mut().for_each(|elf| elf.update());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    model.elves.iter().for_each(|elf| elf.display(&draw));
    draw.to_frame(app, &frame).unwrap();
}
