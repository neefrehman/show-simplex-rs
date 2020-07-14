use noise::{NoiseFn, OpenSimplex};
use schedule_recv::periodic_ms;
use std::io::{self, Write};
use structopt::StructOpt;

mod utils;

use utils::{get_terminal_size, map_to_range};

fn create_grid(columns: u16, rows: u16) -> Vec<(u16, u16)> {
    let mut point_positions: Vec<(u16, u16)> = Vec::new();

    for x in 0..rows {
        for y in 0..columns {
            point_positions.push((x, y))
        }
    }

    return point_positions;
}

fn map_noise_to_max(z: f64, max: i32) -> i32 {
    return map_to_range(z, 0.0, 1.0, 0.0, max as f64 + 1.0)
        .floor()
        .abs() as i32;
}

type DrawReturn = (i32, String);

fn draw_ascii(z: f64) -> DrawReturn {
    let mut z_mapped = map_noise_to_max(z, 9);
    z_mapped = if z_mapped > 9 { 9 } else { z_mapped };
    let char = match z_mapped as i32 {
        0 | 1 | 2 | 3 | 4 => " ",
        5 => ".",
        6 => "o",
        7 => "+",
        8 => "x",
        9 | _ => "X",
    };

    return (z_mapped, char.to_string());
}

fn draw_angle(z: f64) -> DrawReturn {
    let z_mapped = map_noise_to_max(z, 3);
    let char = match z_mapped as i32 {
        0 => "|",
        1 => "/",
        2 => "—",
        3 | _ => "\\",
    };

    return (z_mapped, char.to_string());
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    angle: bool,
    #[structopt(short, long)]
    number: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let draw_grid: fn(z: f64) -> DrawReturn;
    if args.angle {
        draw_grid = draw_angle;
    } else {
        draw_grid = draw_ascii;
    }

    let (columns, rows) = get_terminal_size();
    let point_positions = create_grid(columns, rows);
    let simplex = OpenSimplex::new();

    let mut noise_z = 0 as f64;
    const NOISE_Z_VEL: f64 = 0.000004;

    let tick = periodic_ms(21);

    loop {
        tick.recv().unwrap();
        print!("{}[2J", 27 as char);

        for i in 0..point_positions.len() {
            let position = point_positions[i];
            let (y, x) = position;

            let z = simplex.get([x as f64, y as f64, noise_z]) + 0.5;
            let (z_mapped, mut char) = draw_grid(z);

            if args.number {
                char = z_mapped.to_string();
            }

            io::stdout().flush().unwrap();
            print!("{}", char);

            noise_z += NOISE_Z_VEL;
        }
    }

    Ok(())
}
