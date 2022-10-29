use rand::Rng;

const WIDTH: usize = 50;
const HEIGHT: usize = 10;

#[derive(Clone, Copy, PartialEq)]
enum Connection {
    Double,
    Single,
    Empty,
}

#[derive(Clone, Copy)]
struct Cell {
    character: char,
    connections: [Connection; 4],
}

#[derive(Clone, Copy)]
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

    let mut map = [[Tile::Uncollapsed(cells.len() as u8); WIDTH]; HEIGHT];
    let pos = (rand::thread_rng().gen_range(0..HEIGHT), rand::thread_rng().gen_range(0..WIDTH));
    let chosen_cell = cells[rand::thread_rng().gen_range(0..cells.len())];
    
    // check top
    if pos.0 > 0 {
        match map[pos.0-1][pos.1] {
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
    if pos.1 < WIDTH-1 {
        match map[pos.0][pos.1+1] {
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
    if pos.0 < HEIGHT-1 {
        match map[pos.0+1][pos.1] {
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
    if pos.1 > 0 {
        match map[pos.0][pos.1-1] {
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

    map[pos.0][pos.1] = Tile::Collapsed(chosen_cell);

    draw_map(&map);
}

fn draw_map(map: &[[Tile; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match map[y][x] {
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
