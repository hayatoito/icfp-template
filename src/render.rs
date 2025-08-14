use crate::prelude::*;
use std::f64::consts::PI;

use crate::problem::*;
use crate::solution::*;
use cairo::SvgSurface;

use cairo::Context;

fn convert_to_rgb(n: u8) -> (u8, u8, u8) {
    let red = (n % 8) * 32;
    let green = ((n / 8) % 8) * 32;
    let blue = ((n / 64) % 4) * 64;
    (red, green, blue)
}

pub fn render_svg_on_context(
    cr: &Context,
    problem: &Problem,
    solution: Option<&Solution>,
) -> Result<()> {
    // White background
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint()?;

    // stage
    cr.set_source_rgba(0.8, 0.8, 0.8, 0.5); // gray
    cr.rectangle(
        problem.stage_bottom_left[0],
        problem.stage_bottom_left[1],
        problem.stage_width,
        problem.stage_height,
    );
    cr.fill_preserve()?;
    // #ffc0cb
    cr.set_source_rgba(1.0, 0.8, 0.8, 0.5); // pink
    cr.set_line_width(10.0);
    cr.stroke()?;

    // Attendee
    const ATTENDEE_RADIUS: f64 = 5.0;
    cr.set_source_rgba(0.0, 0.0, 1.0, 0.5); // blue
    for a in &problem.attendees {
        cr.arc(a.x, a.y, ATTENDEE_RADIUS, 0.0, 2.0 * PI);
        cr.fill()?;
    }

    // Pillars
    cr.set_source_rgba(0.5, 0.5, 0.5, 0.5); // gray
    cr.set_line_width(5.0);
    for p in &problem.pillars {
        cr.arc(p.center[0], p.center[1], p.radius, 0.0, 2.0 * PI);
        cr.fill()?;
    }

    // Musician
    const MUSICIAN_RADIUS: f64 = 5.0;

    if let Some(solution) = solution {
        for ((p, volume), inst) in solution
            .placements
            .iter()
            .zip(solution.volumes.iter())
            .zip(problem.musicians.iter())
        {
            let (r, g, b) = convert_to_rgb(((*inst * 19) % 256) as u8);
            let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);

            //
            if *volume == 0.0 {
                cr.set_source_rgba(r, g, b, 0.3);
                cr.arc(p.x, p.y, MUSICIAN_RADIUS, 0.0, 2.0 * PI);
                cr.fill()?;
            } else {
                cr.set_source_rgb(r, g, b);
                cr.arc(p.x, p.y, MUSICIAN_RADIUS, 0.0, 2.0 * PI);
                cr.fill()?;
            }
        }
    }

    Ok(())
}

pub fn render_svg_cairo(
    problem: &Problem,
    solution: Option<&Solution>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let width = problem.room_width;
    let height = problem.room_height;

    std::fs::create_dir_all(out_path.as_ref().parent().unwrap())?;

    let surface = SvgSurface::new(width, height, Some(out_path.as_ref()))?;
    let cr = Context::new(&surface)?;
    render_svg_on_context(&cr, problem, solution)?;
    Ok(())
}

pub fn render_svg(
    problem: &Problem,
    solution: Option<&Solution>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    use svg::node::element::*;
    use svg::*;

    std::fs::create_dir_all(out_path.as_ref().parent().unwrap())?;

    let mut document =
        Document::new().set("renderBox", (0, 0, problem.room_width, problem.room_height));

    // Fill canvas
    document = document.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white"),
    );

    let stage = Rectangle::new()
        .set("x", problem.stage_bottom_left[0])
        .set("y", problem.stage_bottom_left[1])
        .set("width", problem.stage_width)
        .set("height", problem.stage_height)
        .set(
            "style",
            "stroke:pink;stroke-width:10;fill:gray;fill-opacity:0.1;stroke-opacity:0.9",
        );

    document = document.add(stage);

    const ATTENDEE_RADIUS: f64 = 5.0;

    for a in &problem.attendees {
        let circle = Circle::new()
            .set("cx", a.x)
            .set("cy", a.y)
            .set("r", ATTENDEE_RADIUS)
            .set("style", "fill:blue");
        document = document.add(circle);
    }

    for p in &problem.pillars {
        let circle = Circle::new()
            .set("cx", p.center[0])
            .set("cy", p.center[1])
            .set("r", p.radius)
            .set("style", "stroke-width:5;fill:gray;fill-opacity:0.3");
        document = document.add(circle);
    }

    const MUSICIAN_RADIUS: f64 = 5.0;

    if let Some(solution) = solution {
        for ((p, volume), inst) in solution
            .placements
            .iter()
            .zip(solution.volumes.iter())
            .zip(problem.musicians.iter())
        {
            let (r, g, b) = convert_to_rgb(((*inst * 19) % 256) as u8);
            let color = format!("fill:rgb({},{},{})", r, g, b);
            if *volume == 0.0 {
                let color = format!("{};fill-opacity:0.3", color);
                let circle = Circle::new()
                    .set("cx", p.x)
                    .set("cy", p.y)
                    .set("r", MUSICIAN_RADIUS)
                    .set("style", color);
                document = document.add(circle);
            } else {
                let circle = Circle::new()
                    .set("cx", p.x)
                    .set("cy", p.y)
                    .set("r", MUSICIAN_RADIUS)
                    .set("style", color);
                document = document.add(circle);
            }
        }
    }

    svg::save(out_path, &document)?;

    Ok(())
}

pub fn render_problem(id: ProblemId, out_path: impl AsRef<Path>) -> Result<()> {
    let problem = Problem::new(id)?;
    // render_svg(&problem, None, out_path)
    render_svg_cairo(&problem, None, out_path)
}

pub fn render_solution_file(
    id: ProblemId,
    solution_path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let solution = Solution::from(solution_path)?;
    render_solution(id, &solution, out_path)
}

pub fn render_solution(
    id: ProblemId,
    solution: &Solution,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let problem = Problem::new(id)?;
    // render_svg(&problem, Some(&solution), out_path)
    render_svg_cairo(&problem, Some(&solution), out_path)
}
