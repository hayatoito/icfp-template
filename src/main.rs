use clap::Parser;

use icfp2024::prelude::*;
// use icfp2024::problem::*;
// use icfp2024::solution;
// use icfp2024::solver;
// use icfp2024::solver_sa;
// use icfp2024::render;

#[derive(Parser, Debug)]
#[clap(name = "icfp2024")]
enum Cli {
    // DrawProblem {
    //     id: ProblemId,
    //     out_path: PathBuf,
    // },
    // DrawSolution {
    //     id: ProblemId,
    //     solution_path: PathBuf,
    //     out_path: PathBuf,
    // },
    // Solve {
    //     id: ProblemId,
    //     #[arg(long)]
    //     initial_solution_path: Option<PathBuf>,
    // },
    // Bench {
    //     id: ProblemId,
    // },
    // Score {
    //     id: ProblemId,
    //     solution_path: PathBuf,
    // },
    BestScoreRefresh,
}

fn main() -> Result<()> {
    env_logger::init();
    match Cli::parse() {
        // Cli::DrawProblem { id, out_path } => {
        //     render::render_problem(id, out_path)?;
        // }
        // Cli::DrawSolution {
        //     id,
        //     solution_path,
        //     out_path,
        // } => {
        //     render::render_solution_file(id, solution_path, out_path)?;
        // }
        // Cli::Solve {
        //     id,
        //     initial_solution_path,
        // } => {
        //     solver::solve(solver_sa::SolverSa::new(
        //         id,
        //         // 5_000_000.0,
        //         Some(100.0),
        //         // 3 hours.
        //         // solver_sa::End::MaxDuration(std::time::Duration::from_secs(12 * 3_600)),
        //         solver_sa::End::MaxDuration(std::time::Duration::from_secs(60)),
        //         // solver_sa::End::MaxDuration(std::time::Duration::from_secs(3600)),
        //         initial_solution_path.and_then(|path| solution::Solution::from(path).ok()),
        //     )?)?;
        // }
        // Cli::Bench { id } => {
        //     solver::solve(solver_sa::SolverSa::new(
        //         id,
        //         Some(100.0),
        //         solver_sa::End::MaxIteration(50_000),
        //         None,
        //     )?)?;
        // }
        // Cli::Score { id, solution_path } => {
        //     let problem = Problem::new(id)?;
        //     let solution = solution::Solution::from(solution_path)?;
        //     let score = solver_sa::score(&problem, id, id.into(), &solution);
        //     println!("{score}");
        // }
        Cli::BestScoreRefresh => {
            // solution::BestScore::refresh()?;
        }
    }
    Ok(())
}
