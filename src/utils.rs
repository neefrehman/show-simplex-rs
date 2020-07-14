use terminal_size::{terminal_size, Height, Width};

pub fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    let mut columns = 0;
    let mut rows = 0;

    if let Some((Width(w), Height(h))) = size {
        columns = w;
        rows = h;
    } else {
        println!("Unable to get terminal size");
    }

    return (columns, rows);
}

pub fn map_to_range(
    value: f64,
    input_min: f64,
    input_max: f64,
    output_min: f64,
    output_max: f64,
) -> f64 {
    if (input_min - input_max).abs() < f64::EPSILON {
        return output_min;
    }

    let mut out_val =
        ((value - input_min) / (input_max - input_min)) * (output_max - output_min) + output_min;

    if output_max < output_min {
        if out_val < output_max {
            out_val = output_max;
        } else if out_val > output_min {
            out_val = output_min;
        }
    } else if out_val > output_max {
        out_val = output_max;
    } else if out_val < output_min {
        out_val = output_min;
    }

    return out_val;
}
