pub mod maze {
    use std::{os::unix::thread, process::Command, thread::sleep, time::Duration};

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum PositionPoint {
        Parent((i32, i32)),
        Children((i32, i32)),
    }

    impl PositionPoint {
        // fn get_children_tuples(&self, children:PositionPoint)-> (i32, i32){
        //     if let PositionPoint::Children(value) = children{
        //         return value;
        //     } else {
        //         (-1, -1)
        //     }
        // }

        fn parent_to_children(&self, parent:PositionPoint)-> Option<PositionPoint>{

            if let PositionPoint::Parent(x) = parent{
                Some( PositionPoint::Children(x))
            } else {
                None
            }

        }
    }

    pub struct Maze{
        queue: Vec<(i32, i32)>,
        processed: Vec<(i32, i32)>,
        processing: Option<(i32, i32)>,
        start_position: (i32, i32),
        end_position: (i32, i32),
        children_end_position_enum: PositionPoint,
        rutes: Vec<(PositionPoint, PositionPoint)>,
        // opt
        live_streaming: (bool, i32),
    }

    impl Maze {
        pub fn new(start_position:(i32, i32), end_position:(i32, i32), live_streaming: (bool, i32))-> Self{
            // let end_position_enum = (
            //     PositionPoint::Children(end_position)
            // );

            Maze{
                queue: Vec::new(),
                processed: Vec::new(),
                processing: None,
                start_position,
                end_position,
                children_end_position_enum: PositionPoint::Children(end_position),
                rutes: Vec::new(),
                // opt
                live_streaming
            }
        }

        pub fn show_maze(&self, maze: &Vec<Vec<char>>){
            for colls in maze{
                println!("{:?}", colls)
            }
        }



        pub fn running_on_mize(&mut self, maze: Vec<Vec<char>>){
            self.queue.push(self.start_position);
            self.processing(maze);

        }

        fn processing(&mut self, maze: Vec<Vec<char>>){

            loop {
                if self.queue.len() <= 0{
                    break;
                }

                self.processing = Some(*self.queue.get(0).unwrap());
                self.processed.push(*self.queue.get(0).unwrap());
                self.queue.remove(0);
                self.check(&maze);

                if self.live_streaming.0{
                    self.live_streaming_fn(maze.clone());
                    sleep(Duration::from_millis(self.live_streaming.1 as u64));
                }
                
                if self.processing.unwrap() == self.end_position{
                    break;
                }
  

            }

        }

        pub fn live_streaming_fn(&self, mut maze: Vec<Vec<char>>){
            let rutes_raw:Vec<(i32, i32)> = self.rutes.clone()
                .into_iter()
                .map(|(cordinate, _)|{
                    if let PositionPoint::Parent(location) = cordinate {
                        return location
                    } else {
                        (-1, -1)
                    }
                })
                .collect();

            for (colls, rows) in rutes_raw{
                maze[colls as usize][rows as usize] = '@';
            }

            Command::new("clear")
                .status()
                .unwrap();
            self.show_maze(&maze);
        }

        pub fn show_route_in_terminal(&self, mut maze: Vec<Vec<char>>, live_streaming: (bool, i32)){
            let route = self.searching_chain();
            println!("route is \n {:?} \n ====", route);

            if !live_streaming.0{

                for (coll, row) in route{
                    maze[coll as usize][row as usize] = '@';
                    
                }
                println!("fast route is");
                self.show_maze(&maze);
            } else {

                
                
                println!("fast route is");
                for (coll, row) in route{
                    maze[coll as usize][row as usize] = '@';
                    Command::new("clear")
                    .status()
                    .unwrap();
                    self.show_maze(&maze);
                    sleep(Duration::from_millis(live_streaming.1 as u64));
                }
            }
            
        }

        fn searching_chain(&self)-> Vec<(i32, i32)>{
            let mut saving_value = self.children_end_position_enum;
            let mut routes = vec![
                saving_value,
            ];

            loop {
                if let PositionPoint::Children(x) = saving_value{
                    if x == self.start_position{
                        break;
                    }
                }

                let (mut _index_end, value_end) = self.rutes.clone().into_iter()
                .enumerate()
                .find(|(_, value)| {
                    let children_end = value.1;
                    if children_end == saving_value{
                        true
                    } else {
                        false
                    }
                })
                .unwrap();

                let parent = value_end.0;
                let parent = value_end.0.parent_to_children(parent)
                    .unwrap();
                routes.push(parent);
                saving_value = parent;
            }

            let mut routes_raw:Vec<(i32, i32)> = routes.clone()
            .into_iter()
            .map(|cor|{
              if let PositionPoint::Children(value) = cor {
                  return value
              } else {
                  return (-1, -1)
              }
            })
            .collect();
            routes_raw.reverse();

            return routes_raw;
        }

        fn check(&mut self, maze: &Vec<Vec<char>>){
            let colls_value_checks = [1, -1];
            for value_check in colls_value_checks{
                // colls
                let mut colls = self.processing.unwrap().0;
                colls += value_check;

                if colls >= 0 && maze.len() > colls as usize{
                    let row = self.processing.unwrap().1;
                    let _value = maze.get(colls as usize)
                        .unwrap()
                        .get(row as usize)
                        .unwrap();
                    
                    if _value != &'#'{
                        if !self.processed.contains(&(colls, row)) && !self.queue.contains(&(colls, row)){
                            self.queue.push((colls, row));

                            // rutes
                            let routes = (
                                PositionPoint::Parent(self.processing.unwrap()),
                                PositionPoint::Children((colls, row))
                            );

                            self.rutes.push(routes);
                        }
                    }
                }

                // rows
                let mut rows = self.processing.unwrap().1;
                rows += value_check;

                if rows >= 0 && maze.get(self.processing.unwrap().0 as usize).unwrap().len() > rows as usize {
                    let coll = self.processing.unwrap().0;
                    let _value = maze.get(coll as usize)
                        .unwrap()
                        .get(rows as usize)
                        .unwrap();

                    if _value != &'#'{
                        if !self.processed.contains(&(coll, rows)) && !self.queue.contains(&(coll, rows)){
                            self.queue.push((coll, rows));

                            // rutes
                            let routes = (
                                PositionPoint::Parent(self.processing.unwrap()),
                                PositionPoint::Children((coll, rows))
                            );

                            self.rutes.push(routes);
                        }
                    }
                }
            }

        }
    }
}