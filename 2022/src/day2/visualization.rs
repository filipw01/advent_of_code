// use crate::utils::load_input;
// use itertools::Itertools;
// use nannou::prelude::*;
// use nannou_egui::{self, egui, Egui};
// use std::time::Duration;
//
// pub fn main() {
//     nannou::app(model).update(update).simple_window(view).run();
// }
//
// struct Model {
//     elves: Vec<Elf>,
//     egui: Egui,
//     time_since_last_selection: Duration,
//     top_3_elf: usize,
//     found_count: usize,
// }
//
// trait Nannou {
//     fn display(&self, draw: &Draw);
//     fn update(&mut self);
// }
//
// struct Elf {
//     snacks: Vec<usize>,
//     position: Point2,
// }
//
// impl Nannou for Elf {
//     fn display(&self, draw: &Draw) {
//         let position = self.position;
//         draw.ellipse()
//             .x_y(position.x, position.y)
//             .radius(
//                 self.snacks
//                     .iter()
//                     .map(|snack| *snack as f32 / 10000.0)
//                     .sum::<f32>(),
//             )
//             .color(WHITE);
//         // self.snacks.iter().enumerate().for_each(|(index, snack)| {
//         //     draw.ellipse()
//         //         .x_y(self.position.x, self.position.y + (index * 20) as f32)
//         //         .radius(((*snack) / 1000) as f32)
//         //         .color(BLACK);
//         // });
//     }
//
//     fn update(&mut self) {}
// }
//
// fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
//     // Let egui handle things like keyboard and mouse input.
//     model.egui.handle_raw_event(event);
// }
//
// fn model(app: &App) -> Model {
//     let input = load_input(1);
//     let elves = get_elves(&input);
//
//     let window_id = app
//         .new_window()
//         .view(view)
//         .raw_event(raw_window_event)
//         .build()
//         .unwrap();
//     let window = app.window(window_id).unwrap();
//
//     let egui = Egui::from_window(&window);
//     Model {
//         time_since_last_selection: Duration::from_secs(0),
//         found_count: 0,
//         egui,
//         top_3_elf: elves
//             .clone()
//             .into_iter()
//             .map(|elf| elf.iter().sum())
//             .sorted()
//             .rev()
//             .take(3)
//             .rev()
//             .next()
//             .unwrap(),
//         elves: elves
//             .into_iter()
//             .enumerate()
//             .map(|(index, snacks)| Elf {
//                 snacks,
//                 position: Point2::new(
//                     (-300 + ((index % 30) as isize * 15)) as f32,
//                     -300.0 + (index / 30) as f32 * 15.0,
//                 ),
//             })
//             .collect(),
//     }
// }
//
// fn update(_app: &App, model: &mut Model, update: Update) {
//     let egui = &mut model.egui;
//
//     if model.time_since_last_selection < update.since_start + Duration::from_secs(1)
//         && model.elves.len() > model.found_count
//     {
//         model.time_since_last_selection = update.since_start;
//         if model.top_3_elf
//             <= model
//                 .elves
//                 .get(model.found_count)
//                 .unwrap()
//                 .snacks
//                 .iter()
//                 .sum()
//         {
//             model.found_count += 1;
//         } else {
//             model.elves.remove(model.found_count);
//         }
//     }
//     model.elves.iter_mut().for_each(|elf| elf.update());
//
//     egui.set_elapsed_time(update.since_start);
//     let ctx = egui.begin_frame();
//
//     egui::Window::new("Settings").show(&ctx, |ui| {
//         ui.label("X:");
//         ui.add(egui::Slider::new(
//             &mut model.elves[0].position.x,
//             1.0..=40.0,
//         ));
//
//         // Random color button
//         let clicked = ui.button("Random color").clicked();
//
//         if clicked {}
//     });
// }
//
// fn view(app: &App, model: &Model, frame: Frame) {
//     let draw = app.draw();
//     draw.background().color(PLUM);
//     model.elves.iter().for_each(|elf| elf.display(&draw));
//     draw.to_frame(app, &frame).unwrap();
//     // model.egui.draw_to_frame(&frame).unwrap();
// }
