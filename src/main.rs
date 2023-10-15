const ROWS: usize = 100;
const COLS: usize = 100;

fn main() {
    let mut grid = vec![vec![' '; COLS]; ROWS];
    plot_line(&mut grid,10, 10, 50, 40);

    print_grid(&grid);
}

fn plot_line_high(grid: &mut Vec<Vec<char>>, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;

    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut d = (2 * dx) - dy;
    let mut x = x0;

    for y in y0..=y1 {
        plot(grid, x, y);
        if d > 0 {
            x = x + xi;
            d = d + (2 * (dx - dy));
        } else {
            d = d + 2 * dx;
        }
    }
}


fn plot_line_low(grid: &mut Vec<Vec<char>>, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;

    if dx < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut d = (2 * dy) - dx;
    let mut y = y0;

    for x in x0..=x1 {
        plot(grid, x, y);
        if d > 0 {
            y = y + yi;
            d = d + 2 * (dy - dx);
        } else {
            d = d + 2 * dy;
        }
    }
}


fn plot(grid: &mut Vec<Vec<char>>, x: i32, y: i32){
    grid[y as usize][x as usize] = '$'
}


fn plot_line(grid: &mut Vec<Vec<char>>, x0: i32, y0: i32, x1: i32, y1: i32) {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(grid,x1, y1, x0, y0);
        } else {
            plot_line_low(grid, x0, y0, x1, y1);
        }
    } else {
        if y0 > y1 {
            plot_line_high(grid, x1, y1, x0, y0);
        } else {
            plot_line_high(grid, x0, y0, x1, y1);
        }
    }
}


fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let mut substr = String::new();
        for &value in row {
            //println!("{}", value);
            substr.push_str(&value.to_string());
        }
        println!("{}", substr);
    }
}
