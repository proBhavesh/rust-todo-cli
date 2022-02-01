use std::collections::HashMap;

// #[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo{
    fn insert(&mut self, key: String){
        self.map.insert(key, true);
    }
    //saves the file
    fn save(self)->Result<(), std::io::Error>{
        let mut content = String::new();
        for (k,v) in self.map{
            let record=format!("{}\t{}\n",k,v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    // adding todos

    fn new()-> Result<Todo, std::io::Error>{
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content=String::new();
        f.read_to_string(&mut content)?;
    }
}
fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");


    let mut todo=Todo{
        map: HashMap::new(),
    };

    if action=="add"{
        todo.insert(item);
        match todo.save(){
            Ok(_)=>println!("todo saved"),
            Err(why)=>println!("An error occured: {}", why),
            }
    }
    // println!("{:?}, {:?}", action, item);
}
