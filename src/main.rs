mod maze;

use crate::maze::maze::Maze;

fn main() {
    let maze = vec![
        vec!['S', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', 'E'],
        vec!['#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '.'],
        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.'],
        vec!['#', '#', '#', '.', '#', '.', '#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '#', '#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.'],
        vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#', '.', '#', '#'],
        vec!['.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.', '#', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '#', '#', '.'],
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '#', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '#', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
        vec!['#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '.', '#', '#'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '.'],
        vec!['#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.'],
        vec!['.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '.', '#', '.', '.'],
        vec!['.', '#', '.', '.', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.', '.'],
        vec!['.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '.', '#'],
        vec!['.', '#', '.', '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.', '.'],
        vec!['.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
        vec!['.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '#', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '.'],
        vec!['.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.', '#', '#', '.'],
        vec!['.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '#', '#', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '.', '.', '.', '.', '#', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '#', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.', '#', '.', '.', '#', '.', '#', '#', '#', '.', '#', '#', '.', '.', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.'],
        vec!['.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#', '.', '#', '#', '.', '#', '.'],
        vec!['.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.'],
        vec!['.', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '#', '.', '#', '#', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '#', '#', '.', '.'],
        vec!['.', '#', '#', '.', '#', '#', '.', '.', '.', '.', '#', '#', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '#', '.'],
        vec!['.', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '.', '#', '.', '.', '#', '#', '.'],
        vec!['.', '.', '.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        
    ];
    

    let mut maze_engine = Maze::new((0, 0), (0, 30), (true, 50));
    // maze_engine.show_maze(&maze);

    maze_engine.running_on_mize(maze.clone());
    maze_engine.show_route_in_terminal(maze, (true, 25));
}
