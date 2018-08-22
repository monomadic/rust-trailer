


// use plotlib;
// use plotlib::style::Point;

// pub fn draw() {
//     let data = [
//         (-3.0, 2.3),
//         (-1.6, 5.3),
//         (0.3, 0.7),
//         (4.3, -1.4),
//         (6.4, 4.3),
//         (8.5, 3.7),
//     ];
//     let s1 = plotlib::scatter::Scatter::from_slice(&data);
//     let s2 = plotlib::scatter::Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
//         .style(plotlib::scatter::Style::new().marker(plotlib::style::Marker::Square));
//     let v = plotlib::view::ContinuousView::new()
//         .add(&s1)
//         .add(&s2)
//         .x_range(-5., 10.)
//         .y_range(-2., 6.)
//         .x_label("Some varying variable")
//         .y_label("The response of something");
//     println!("{}", plotlib::page::Page::single(&v).to_text());
// }






use plotlib;
use plotlib::style::Line;

pub fn draw(values: Vec<f64>) {
  let idx_values:Vec<(f64, f64)> = values.into_iter().enumerate().map(|(i, e)| (i as f64, e)).collect();
  println!("{:?}", idx_values);
  let l1 = plotlib::line::Line::new(&idx_values)
      .style(plotlib::line::Style::new().colour("burlywood"));
  let v = plotlib::view::ContinuousView::new().add(&l1);
  // println!("{}", plotlib::page::Page::single(&v).to_text());
  plotlib::page::Page::single(&v).save("line.svg");
}
