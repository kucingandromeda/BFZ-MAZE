pub mod maze {
    pub struct Maze{
        queue: Vec<(i32, i32)>,
        processed: Vec<(i32, i32)>,
        processing: Option<(i32, i32)>,
        start_position: (i32, i32),
        end_position: (i32, i32),
        founded: bool,
    }

    impl Maze {
        pub fn new(start_position:(i32, i32), end_position:(i32, i32))-> Self{
            Maze{
                queue: Vec::new(),
                processed: Vec::new(),
                processing: None,
                start_position,
                end_position,
                founded: false,
            }
        }

        pub fn running_on_mize(&mut self, maze: Vec<Vec<char>>){
            // println!("{:?}", maze);
            
            self.queue.push(self.start_position);
            self.processing(maze);
            println!("{:?}", self.queue);

        }

        fn processing(&mut self, maze: Vec<Vec<char>>){

            loop {
                if self.queue.len() <= 0 || self.founded{
                    break;
                }

                println!("{:?} \n =======", self.queue);
                self.processing = Some(*self.queue.get(0).unwrap());
                self.processed.push(*self.queue.get(0).unwrap());
                self.queue.remove(0);

                self.check(&maze);
            }

        }

        fn check(&mut self, maze: &Vec<Vec<char>>){
            let colls_value_checks = [1, -1];
            for value_check in colls_value_checks{
                // colls
                let mut colls = self.processing.unwrap().0;
                colls += value_check;

                // println!("{} | {}", maze.len(), colls);
                if colls >= 0 && maze.len() > colls as usize{
                    let row = self.processing.unwrap().1;
                    let _value = maze.get(colls as usize)
                        .unwrap()
                        .get(row as usize)
                        .unwrap();
                    // println!("value in ({colls}, {row}) is {:?}", value);
                    if _value != &'#'{
                        if !self.processed.contains(&(colls, row)){

                            if (colls, row) == self.end_position{
                                println!("founded!");
                                self.founded = true;
                            }
                            self.queue.push((colls, row));
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
                    // println!("value in ({coll}, {rows}) is {:?}", value);
                    if _value != &'#'{
                        if !self.processed.contains(&(coll, rows)){

                            if (coll, rows) == self.end_position{
                                println!("founded!");
                                self.founded = true;
                            }
                            self.queue.push((coll, rows));
                        }
                    }
                }
            }

        }
    }
}