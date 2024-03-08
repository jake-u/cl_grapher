extern crate termsize;

use vec2d::Vec2D;
use vec2d::Size;
use vec2d::Coord;
use std::io;

/*
Vec2D uses numbers that correspond to the following for printing:

0 = ' '
1 = '+'
2 = '_'
3 = '|'
4 = '#'

*/

fn reset_graph(graph: &mut Vec2D<u8>)
{
    let half_x = graph.size().width / 2;
    let half_y = graph.size().height / 2;

    // blank graph & draw axis
    // increments x, for each y
    for it in graph.iter_mut()
    {
        if it.0.x == half_x && it.0.y == half_y
        {
            *it.1 = 1 as u8;
        } else if it.0.x == half_x
        {
            *it.1 = 2 as u8;
        } else if it.0.y == half_y
        {
            *it.1 = 3 as u8;
        } else {
            *it.1 = 0 as u8;
        }
    }
}

fn print_graph(graph: &mut Vec2D<u8>) 
{
    // clear screen
    print!("\x1B[2J\x1B[1;1H");

    // draw graph
    for it in graph.iter_mut()
    {
        match it.1 {
            0 => print!(" "),
            1 => print!("+"),
            2 => print!("|"),
            3 => print!("-"),
            4 => print!("#"),
            _ => print!("e"),
        }
    }
}

fn graph_linear_equation(graph: &mut Vec2D<u8>)
{
    let half_x = (graph.size().width / 2) as isize;
    let half_y = (graph.size().height / 2) as isize;

    // get variables from user
    let mut response = String::new();

    println!("y=mx+b");
    println!("Enter the slope (m): ");
    let _ = io::stdin().read_line(&mut response);
    let slope = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y={slope}x+b");
    println!("Enter the y-intercept (b): ");
    let _ = io::stdin().read_line(&mut response);
    let y_inter = response.trim().parse::<f32>().unwrap();


    // draw line
    for x in -half_x..half_x
    {
        let y = (((x as f32) * slope) + y_inter) as isize;
        
        if y >= -half_y && y < half_y
        {
            *graph.get_mut(Coord {x: (x + half_x) as usize, y: (y + half_y) as usize}).unwrap() = 4;
        }
    }

    print_graph(graph);
    println!("y={slope}x+{y_inter}");
}

fn graph_sine_wave(graph: &mut Vec2D<u8>)
{
    let half_x = (graph.size().width / 2) as isize;
    let half_y = (graph.size().height / 2) as isize;

    // get variables from user
    let mut response = String::new();

    println!("y=(a*sin(fx))+o");
    println!("Enter the amplitude (a): ");
    let _ = io::stdin().read_line(&mut response);
    let amp = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y=({amp}*sin(fx))+o");
    println!("Enter the frequency (f): ");
    let _ = io::stdin().read_line(&mut response);
    let freq = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y=({amp}*sin({freq}x))+o");
    println!("Enter the offset (o): ");
    let _ = io::stdin().read_line(&mut response);
    let offset = response.trim().parse::<f32>().unwrap();


    // draw wave
    for x in -half_x..half_x
    {
        let y = ((amp * ((x as f32) * freq).sin()) + offset) as isize;
       
        if y >= -half_y && y < half_y
        {
            *graph.get_mut(Coord {x: (x + half_x) as usize, y: (y + half_y) as usize}).unwrap() = 4;
        }
    }

    print_graph(graph);
    println!("y=({amp}*sin({freq}x))+{offset}");
}

fn graph_tangent_wave(graph: &mut Vec2D<u8>) 
{
    let half_x = (graph.size().width / 2) as isize;
    let half_y = (graph.size().height / 2) as isize;

    // get variables from user
    let mut response = String::new();

    println!("y=(a*tan(fx))+o");
    println!("Enter the amplitude (a): ");
    let _ = io::stdin().read_line(&mut response);
    let amp = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y=({amp}*tan(fx))+o");
    println!("Enter the frequency (f): ");
    let _ = io::stdin().read_line(&mut response);
    let freq = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y=({amp}*tan({freq}x))+o");
    println!("Enter the offset (o): ");
    let _ = io::stdin().read_line(&mut response);
    let offset = response.trim().parse::<f32>().unwrap();


    // draw wave
    for x in -half_x..half_x
    {
        let y = ((amp * ((x as f32) * freq).tan()) + offset) as isize;
        
        if y >= -half_y && y < half_y
        {
            *graph.get_mut(Coord {x: (x + half_x) as usize, y: (y + half_y) as usize}).unwrap() = 4;
        }
    }

    print_graph(graph);
    println!("y=({amp}*tan({freq}x))+{offset}");
}

fn graph_quadratic(graph: &mut Vec2D<u8>)
{
    let half_x = (graph.size().width / 2) as isize;
    let half_y = (graph.size().height / 2) as isize;

    // get variables from user
    let mut response = String::new();

    println!("y=ax^2 + bx + c");
    println!("Enter (a): ");
    let _ = io::stdin().read_line(&mut response);
    let a = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y={a}x^2 + bx + c");
    println!("Enter (b): ");
    let _ = io::stdin().read_line(&mut response);
    let b = response.trim().parse::<f32>().unwrap();

    response = String::new();

    println!("y={a}x^2 + {b}x + c");
    println!("Enter (c): ");
    let _ = io::stdin().read_line(&mut response);
    let c = response.trim().parse::<f32>().unwrap();


    // draw
    for x in -half_x..half_x
    {
        let y = ((a * (x * x) as f32) + (b * x as f32) + c) as isize;
        
        if y >= -half_y && y < half_y
        {
            *graph.get_mut(Coord {x: (x + half_x) as usize, y: (y + half_y) as usize}).unwrap() = 4;
        }
    }

    print_graph(graph);
    println!("y={a}x^2 + {b}x + {c}");
}

fn main() 
{
    // use termsize crate to get dimensions of command line, so the graph can adjust accordingly
    let rows: usize = (termsize::get().unwrap().rows).into();
    let cols: usize = termsize::get().unwrap().cols.into();
    let mut graph: Vec2D<u8> = Vec2D::from_example(Size::new(cols, rows), &0);
    reset_graph(&mut graph);
    
    // user input loop
    loop {
        
        println!("Enter an option:\n0: exit\n1: y=mx+b\n2: sin(x)\n3: tan(x)\n4: quadratic");
        let mut response = String::new();
        let _ = io::stdin().read_line(&mut response);
        let choice = response.trim().parse().unwrap();

        match choice {
            0 => break,
            1 => graph_linear_equation(&mut graph),
            2 => graph_sine_wave(&mut graph),
            3 => graph_tangent_wave(&mut graph),
            4 => graph_quadratic(&mut graph),
            _ => println!("Invalid option"),
        }

        reset_graph(&mut graph);
    }

    println!("Goodbye.");
}

