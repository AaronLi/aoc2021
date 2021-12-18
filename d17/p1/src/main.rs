use std::fs;
// vertical(initial_y=y, step=n) = 0 + y + y-1 + y-2 + y-3 + y-4 + y-5 + ... + y-n
// vertical(initial_y=y, step=n) = n*y - ((n-1)*((n-1)+1))/2

// horizontal(step=n) = 0 + x + x-1 + x-2 + x-3 + ... + x-x + 0 + 0 + ... (if x > 0)
// horizontal(step=n) = 0 + x + x+1 + x+2 + x+3 + ... + x+x + 0 + 0 + ... (if x < 0)

// horizontal(initial_x=x, step=n) = min(x, n) * x - ((min(x, n)-1) * ((min(x, n)-1)+1))/2 (if x > 0)
// horizontal(initial_x=x, step=n) = min(-x, n) * x + ((min(-x, n)-1) * ((min(-x, n)-1)+1))/2 (if x <= 0)

fn calc_vertical(step: isize, initial_y: isize) -> isize {
    step * initial_y - ((step-1) * step)/2
}

fn calc_horizontal(step: isize, initial_x: isize) -> isize {
    match initial_x > 0 {
        true => {
            let min = step.min(initial_x);
            min * initial_x - ((min - 1) * min) / 2
        },
        false => {
            let min = step.min(-initial_x);
            min * initial_x + ((min - 1) * min) / 2
        }
    }
}



fn main() {
    let target = fs::read_to_string("input").and_then(|line| {
        let area_info = &line[13..line.len()];
        let mut range_components = area_info.split(", ");
        let x_range_info = range_components.next().expect("Invalid input format");
        let x_range = &x_range_info[2..x_range_info.len()].split("..").into_iter().map(|x| x.parse::<isize>().expect("Invalid input format")).collect::<Vec<isize>>();

        let y_range_info = range_components.next().expect("Invalid input format");
        let y_range = &y_range_info[2..y_range_info.len()].split("..").into_iter().map(|x| x.parse::<isize>().expect("Invalid input format")).collect::<Vec<isize>>();
        Ok((x_range[0]..x_range[1]+1, y_range[0]..y_range[1]+1))
    }).expect("File not found");

    let mut highest_y: Option<(isize, isize)> = None;

    let mut launch_y = 0;

    loop {
        let mut step = 0;
        let mut peak = (0, 0);
        let mut valid = false;
        loop {
            let proj_y = calc_vertical(step, launch_y);
            if proj_y < target.1.start {
                break
            }

            peak = (peak.0.max(proj_y), launch_y);
            if target.1.contains(&proj_y){
                valid = true;
            }
            step += 1;
        }
        if valid {
            if highest_y.is_none() || peak.0 > highest_y.unwrap().0 {
                highest_y = Some(peak);
            }
            println!("{:?}", highest_y.unwrap());
        }else{
            //break;
        }
        launch_y += 1;
    }

    println!("{:?}", highest_y.unwrap());
}