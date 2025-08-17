use gtk::glib::clone;
use gtk::{prelude::*, ScrolledWindow};
use gtk::{glib, Align, Application, ApplicationWindow, Box, DrawingArea, Orientation, Scale};
use std::cell::{RefCell};
use std::rc::Rc;
use crate::problem::*;
use crate::solution::*;
use crate::draw::*;

const APP_ID: &str = "hayato.icfp";

pub fn run(id: ProblemId, receiver: async_channel::Receiver<Solution>) -> glib::ExitCode {
    println!("gui run");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        build_ui(app, id, receiver.clone());
    });
    // app.run()
    app.run_with_args::<&str>(&[])
}

fn build_ui(app: &Application, id: ProblemId, receiver: async_channel::Receiver<Solution>) {
    let problem = Problem::new(id).expect("problem::new?");
    let solution: Rc<RefCell<Option<Solution>>> = Rc::new(RefCell::new(None));

    let scale = Scale::builder()
        .width_request(200)
        .draw_value(true)
        .adjustment(
            &gtk::Adjustment::builder()
                .lower(0.1)
                .upper(2.0)
                .step_increment(0.1)
                .value(1.0)
                .build(),
        )
        .build();

    let drawing_area = DrawingArea::builder()
        .content_width(problem.room_width as i32)
        .content_height(problem.room_height as i32)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    scale.connect_value_changed(clone!(
        #[weak]
        drawing_area,
        move |_| {
            drawing_area.queue_draw();
        }
    ));

    drawing_area.set_draw_func(clone!(
        #[weak]
        solution,
        #[weak]
        scale,
        move |_, cr, _width, _height| {
            cr.scale(scale.value(), scale.value());
            draw_svg_on_context(&cr, &problem, solution.borrow().as_ref()).expect("draw?");
        }
    ));

    let buttons = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Start)
        .spacing(8)
        .orientation(Orientation::Horizontal)
        .build();
    buttons.append(&scale);

    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(8)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&buttons);

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(1800)
        .min_content_height(1800)
        .child(&drawing_area)
        .build();
    gtk_box.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();

    glib::spawn_future_local(async move {
        while let Ok(a) = receiver.recv().await {
            *solution.borrow_mut() = Some(a);
            drawing_area.queue_draw();
        }
    });

}
