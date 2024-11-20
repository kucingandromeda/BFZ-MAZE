mod maze;

use crate::maze::maze::Maze;

fn main() {
    let maze = vec![
        vec!['S', '.', '.', '#', '.' ,'.' ,'.'],
        vec!['.', '#', '.', '.', '.' ,'#' ,'.'],
        vec!['.', '#', '.', '.', '.' ,'.' ,'.'],
        vec!['.', '.', '#', '#', '.' ,'.' ,'.'],
        vec!['#', '.', '#', 'E', '.' ,'#' ,'.'],
    ];

    let mut maze_engine = Maze::new((0, 0), (4, 3));
    maze_engine.running_on_mize(maze);
}
