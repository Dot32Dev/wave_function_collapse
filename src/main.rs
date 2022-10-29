use rand::Rng;

const WIDTH: usize = 50;
const HEIGHT: usize = 10;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Connection {
    Double,
    Empty,
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Cell {
    character: char,
    connections: [Connection; 4],
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Collapsed(Cell),
    Uncollapsed(u8),
}

fn main() {
    // let Cells = ["═", "║", "╔", "╗", "╚", "╝╠", "╣", "╦", "╩", "╬"];
    let cells = [
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
    ];

    //1 dimensional array for a 2 dimensional map
    let mut map = [Tile::Uncollapsed(cells.len() as u8); WIDTH * HEIGHT];
    let pos: usize = rand::thread_rng().gen_range(0..WIDTH * HEIGHT);
    println!("pos: {}", pos);
    let chosen_cell = cells[rand::thread_rng().gen_range(0..cells.len())];
    
    // check top
    if pos > WIDTH {
        match map[pos-WIDTH] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy = 0;
                for cell in cells.iter() {
                    if cell.connections[2] == chosen_cell.connections[0] {
                        new_entropy += 1;
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                *entropy = new_entropy;
            },
        }
    }
    // check right
    if pos % WIDTH != WIDTH - 1 {
        match map[pos+1] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy = 0;
                for cell in cells.iter() {
                    if cell.connections[3] == chosen_cell.connections[1] {
                        new_entropy += 1;
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                *entropy = new_entropy;
            },
        }
    }
    // check bottom
    if pos < WIDTH*(HEIGHT-1) {
        match map[pos+WIDTH] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy = 0;
                for cell in cells.iter() {
                    if cell.connections[0] == chosen_cell.connections[2] {
                        new_entropy += 1;
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                *entropy = new_entropy;
            },
        }
    }
    // check left
    if pos % WIDTH > 0 {
        match map[pos-1] {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(ref mut entropy) => {
                let mut new_entropy = 0;
                for cell in cells.iter() {
                    if cell.connections[1] == chosen_cell.connections[3] {
                        new_entropy += 1;
                    }
                }
                // entropy -= cells.len() as u8 - new_entropy;
                *entropy = new_entropy;
            },
        }
    }

    map[pos] = Tile::Collapsed(chosen_cell);

    let mut possible_tiles = Vec::new();
    for (i, tile) in map.iter().enumerate() {
        match tile {
            Tile::Collapsed(_) => (),
            Tile::Uncollapsed(entropy) => {
                'scan: for other in possible_tiles.iter() {
                    match map[*other] {
                        Tile::Collapsed(_) => unreachable!("possible_tiles contains a collapsed tile"),
                        Tile::Uncollapsed(other_entropy) => {
                            if entropy < &other_entropy {
                                possible_tiles.clear();
                                possible_tiles.push(i);
                                break 'scan;
                            } else if entropy == &other_entropy {
                                possible_tiles.push(i);
                                break 'scan;
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

    println!("possible_tiles: {:?}", possible_tiles);
    let chosen_tile = possible_tiles[rand::thread_rng().gen_range(0..possible_tiles.len())];
    println!("chosen_tile: {}", chosen_tile);

    draw_map(&map);
}

fn draw_map(map: &[Tile; WIDTH*HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match map[y*WIDTH+x] {
                Tile::Collapsed(cell) => print!("{}", cell.character),
                Tile::Uncollapsed(entropy) => {
                    if entropy < 10 {
                        print!("{}", entropy)
                    } else {
                        print!("█")
                    }
                },
            }
        }
        println!("");
    }
}
