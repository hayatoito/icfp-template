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

pub fn draw_svg_on_context(
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

pub fn draw_svg(
    problem: &Problem,
    solution: Option<&Solution>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let width = problem.room_width;
    let height = problem.room_height;

    std::fs::create_dir_all(out_path.as_ref().parent().unwrap())?;

    let surface = SvgSurface::new(width, height, Some(out_path.as_ref()))?;
    let cr = Context::new(&surface)?;
    draw_svg_on_context(&cr, problem, solution)?;
    Ok(())
}

pub fn draw_problem(id: ProblemId, out_path: impl AsRef<Path>) -> Result<()> {
    let problem = Problem::new(id)?;
    // render_svg(&problem, None, out_path)
    draw_svg(&problem, None, out_path)
}

pub fn draw_solution_file(
    id: ProblemId,
    solution_path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let solution = Solution::from(solution_path)?;
    draw_solution(id, &solution, out_path)
}

pub fn draw_solution(id: ProblemId, solution: &Solution, out_path: impl AsRef<Path>) -> Result<()> {
    let problem = Problem::new(id)?;
    draw_svg(&problem, Some(&solution), out_path)
}
