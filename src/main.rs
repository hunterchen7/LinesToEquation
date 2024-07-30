mod edge_detection;
mod fourier;
mod img_to_line;
use std::thread;
use std::time::Instant;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let builder = thread::Builder::new().stack_size(32 * 1024 * 1024);

    let handler = builder
        .spawn(|| {
            let img = img_to_line::get_image("images/shapes1.png");

            let now = Instant::now();
            let mut edges = edge_detection::canny(&img, 50.0, 100.0);
            println!("Sobel: {:?}", now.elapsed());
            edges.save("generated/edges.png").unwrap();

            let now = Instant::now();
            let mut lines = img_to_line::edges_to_lines_b(&mut edges);
            lines.sort_by_key(|b| std::cmp::Reverse(b.len())); // sort by length
            lines.truncate(32); // only take n longest lines
            img_to_line::lines_to_img(&lines);
            println!("Edges to lines: {:?}", now.elapsed());

            let mut file = File::create("generated/equations.txt").unwrap();

            let now = Instant::now();
            let mut equations = Vec::new();
            for line in lines.iter() {
                let equation = fourier::construct_equation(line);
                // construct equation but it only uses every nth point
                // let equation = fourier::construct_equation(&line.iter().step_by(10).map(|p| *p).collect::<Vec<_>>());
                let _ = file.write(equation.as_bytes());
                let _ = file.write(b"\n");
                equations.push(equation);
                // println!("Equation: {}", equation);
            }
            println!("Construct equations: {:?}", now.elapsed());
        })
        .unwrap();

    handler.join().unwrap();
    Ok(())
}
