use std::time::{Instant};
use rand::Rng;
use colored::*;

const WIDTH: usize = 220;
const HEIGHT: usize = 50;

#[derive(Clone, Copy, PartialEq)]
enum Connection {
    Double,
    Single,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    character: char,
    connections: [Connection; 4],
}

#[derive(Clone)]
enum Tile {
    Collapsed(Cell),
    Uncollapsed(Vec<Cell>),
}

fn main() {
    // let Cells = ["═", "║", "╔", "╗", "╚", "╝╠", "╣", "╦", "╩", "╬"];
    let cells = [
        // Double lines

        Cell {
            character: ' ',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
        Cell {
            character: '═',
            connections: [
                Connection::Empty, 
                Connection::Double, 
                Connection::Empty, 
                Connection::Double
            ],
        },
        Cell {
            character: '║',
            connections: [
                Connection::Double, 
                Connection::Empty, 
                Connection::Double, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╔',
            connections: [
                Connection::Empty, 
                Connection::Double, 
                Connection::Double, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╗',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Double, 
                Connection::Double
            ],
        },
        Cell {
            character: '╚',
            connections: [
                Connection::Double, 
                Connection::Double, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╝',
            connections: [
                Connection::Double, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Double
            ],
        },
        Cell {
            character: '╠',
            connections: [
                Connection::Double, 
                Connection::Double, 
                Connection::Double, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╣',
            connections: [
                Connection::Double, 
                Connection::Empty, 
                Connection::Double, 
                Connection::Double
            ],
        },
        Cell {
            character: '╦',
            connections: [
                Connection::Empty, 
                Connection::Double, 
                Connection::Double, 
                Connection::Double
                ],
        },
        Cell {
            character: '╩',
            connections: [
                Connection::Double, 
                Connection::Double, 
                Connection::Empty, 
                Connection::Double
            ],
        },
        Cell {
            character: '╬',
            connections: [
                Connection::Double, 
                Connection::Double, 
                Connection::Double, 
                Connection::Double
            ],
        },

        // Joiners

        Cell {
            character: '╒',
            connections: [
                Connection::Empty, 
                Connection::Double, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╓',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Double, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╕',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Double
            ],
        },
        Cell {
            character: '╖',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Double, 
                Connection::Single
            ],
        },
        Cell {
            character: '╘',
            connections: [
                Connection::Single, 
                Connection::Double, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╙',
            connections: [
                Connection::Double, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╛',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Double
            ],
        },
        Cell {
            character: '╜',
            connections: [
                Connection::Double, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single
            ],
        },
        Cell {
            character: '╞',
            connections: [
                Connection::Single, 
                Connection::Double, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╟',
            connections: [
                Connection::Double, 
                Connection::Single, 
                Connection::Double, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╡',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Double
            ],
        },
        Cell {
            character: '╢',
            connections: [
                Connection::Double, 
                Connection::Empty, 
                Connection::Double, 
                Connection::Single
            ],
        },
        Cell {
            character: '╤',
            connections: [
                Connection::Empty, 
                Connection::Double, 
                Connection::Single, 
                Connection::Double
            ],
        },
        Cell {
            character: '╥',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Double, 
                Connection::Single
            ],
        },
        Cell {
            character: '╧',
            connections: [
                Connection::Single, 
                Connection::Double, 
                Connection::Empty, 
                Connection::Double
            ],
        },
        Cell {
            character: '╨',
            connections: [
                Connection::Double, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Single
            ],
        },
        Cell {
            character: '╪',
            connections: [
                Connection::Single, 
                Connection::Double, 
                Connection::Single, 
                Connection::Double
            ],
        },
        Cell {
            character: '╫',
            connections: [
                Connection::Double, 
                Connection::Single, 
                Connection::Double, 
                Connection::Single
            ],
        },

        // Single Lines

        Cell {
            character: '┌',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '┐',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Single
            ],
        },
        Cell {
            character: '└',
            connections: [
                Connection::Single, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
        Cell {
            character: '┘',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single
            ],
        },
        Cell {
            character: '├',
            connections: [
                Connection::Single, 
                Connection::Single, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '┤',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Single
            ],
        },
        Cell {
            character: '┬',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Single, 
                Connection::Single
            ],
        },
        Cell {
            character: '┴',
            connections: [
                Connection::Single, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Single
            ],
        },
        Cell {
            character: '┼',
            connections: [
                Connection::Single, 
                Connection::Single, 
                Connection::Single, 
                Connection::Single
            ],
        },
        Cell {
            character: '│',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '─',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Single
            ],
        },

        // Curves

        Cell {
            character: '╭',
            connections: [
                Connection::Empty, 
                Connection::Single, 
                Connection::Single, 
                Connection::Empty
            ],
        },
        Cell {
            character: '╮',
            connections: [
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single, 
                Connection::Single
            ],
        },
        Cell {
            character: '╯',
            connections: [
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty, 
                Connection::Single
            ],
        },
        Cell {
            character: '╰',
            connections: [
                Connection::Single, 
                Connection::Single, 
                Connection::Empty, 
                Connection::Empty
            ],
        },
    ];
    
    //1 dimensional array for a 2 dimensional map
    let mut map = vec![Tile::Uncollapsed(cells.to_vec()); WIDTH * HEIGHT];
    
    let begining = Instant::now();
    'outer: loop {
        for _i in 0..20 {
            let mut possible_tiles = Vec::new();
            for (i, tile) in map.iter().enumerate() {
                match tile {
                    Tile::Collapsed(_) => (),
                    Tile::Uncollapsed(entropy) => {
                        for other in possible_tiles.iter() {
                            match &map[*other] {
                                Tile::Collapsed(_) => unreachable!("possible_tiles contains a collapsed tile"),
                                Tile::Uncollapsed(other_entropy) => {
                                    if entropy.len() < other_entropy.len() {
                                        possible_tiles.clear();
                                        possible_tiles.push(i);
                                        break;
                                    } else if entropy.len() == other_entropy.len() {
                                        possible_tiles.push(i);
                                        break;
                                    }
                                },
                            }
                        }
                        if possible_tiles.is_empty() {
                            possible_tiles.push(i);
                        }
                    },
                }
            }

            // println!("possible_tiles: {:?}", possible_tiles);

            if possible_tiles.len() == 0 {
                break 'outer;
            }

            let chosen_tile = possible_tiles[rand::thread_rng().gen_range(0..possible_tiles.len())];
            // println!("chosen_tile: {}", chosen_tile);
            
            match &map[chosen_tile] {
                Tile::Collapsed(_) => unreachable!("chosen_tile is a collapsed tile"),
                Tile::Uncollapsed(entropy) => {
                    let chosen_cell = entropy[rand::thread_rng().gen_range(0..entropy.len())].clone();
                    map[chosen_tile] = Tile::Collapsed(chosen_cell);
                    propogate_entropy(&mut map, &chosen_tile, &cells, &chosen_cell);
                },
            }
        }
        print!("{esc}c", esc = 27 as char);
        draw_map(&map);
    }

    println!("Generated {} tiles in {} seconds", WIDTH*HEIGHT, begining.elapsed().as_secs());
}

fn draw_map(map: &Vec<Tile>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match &map[y*WIDTH+x] {
                Tile::Collapsed(cell) => {
                    // print!("{}", cell.character)
                    if x < 3 || y == 0 || x > WIDTH-3 || y == HEIGHT-1 {
                        print!("{}", cell.character.to_string().on_red())
                    } else {
                        print!("{}", cell.character)
                    }
                },
                Tile::Uncollapsed(entropy) => {
                    if entropy.len() < 10 {
                        print!("{}", entropy.len())
                    } else {
                        // print!("{}", "█".truecolor(255/30*entropy.len() as u8, 255/30*entropy.len() as u8, 255/30*entropy.len() as u8))
                        // print!(" ")
                        if x < 3 || y == 0 || x > WIDTH-3 || y == HEIGHT-1 {
                            print!("{}", "░".on_red())
                        } else {
                            print!("░")
                        }
                    }
                },
            }
        }
        println!("");
    }
}

fn propogate_entropy(map: &mut Vec<Tile>, pos: &usize, cells: &[Cell], chosen_cell: &Cell) {
    // check top
    if pos > &WIDTH {
        match map[pos-WIDTH] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy: Vec<Cell> = Vec::new();
                for cell in cells.iter() {
                    if cell.connections[2] != chosen_cell.connections[0] {
                        new_entropy.push(cell.clone());
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                // *entropy = new_entropy;
                subtract_vector(entropy, &new_entropy);
            },
        }
    }
    // check right
    if pos % WIDTH != WIDTH - 1 {
        match map[pos+1] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy: Vec<Cell> = Vec::new();
                for cell in cells.iter() {
                    if cell.connections[3] != chosen_cell.connections[1] {
                        new_entropy.push(cell.clone());
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                // *entropy = new_entropy;
                subtract_vector(entropy, &new_entropy);
            },
        }
    }
    // check bottom
    if pos < &(WIDTH*(HEIGHT-1)) {
        match map[pos+WIDTH] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy: Vec<Cell> = Vec::new();
                for cell in cells.iter() {
                    if cell.connections[0] != chosen_cell.connections[2] {
                        new_entropy.push(cell.clone());
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                subtract_vector(entropy, &new_entropy);
            },
        }
    }
    // check left
    if pos % WIDTH > 0 {
        match map[pos-1] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy: Vec<Cell> = Vec::new();
                for cell in cells.iter() {
                    if cell.connections[1] != chosen_cell.connections[3] {
                        new_entropy.push(cell.clone());
                    }
                }

                subtract_vector(entropy, &new_entropy);
            },
        }
    }
}

fn subtract_vector(a: &mut Vec<Cell>, b: &Vec<Cell>) {
    a.retain(|x| !b.contains(x));
}