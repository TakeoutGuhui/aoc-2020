use crate::utils::read_lines_true;

pub fn part_1(){
    let lines = read_lines_true("input/day_4.txt").unwrap();
    //println!("{:?}", lines);
    let split = lines.split(|line| line == "");
    let mut pass_list: Vec<Vec<&str>> = Vec::new();

    for (id, pass_lines) in split.enumerate() {
        let mut fields = Vec::new();
        for line in pass_lines {
            let items: Vec<&str> = line.split(' ').collect();
            for item in items {
                fields.push(item);
            }
            
        }
        pass_list.push(fields);
        
    }

    pass_list.filter()
    for pass in pass_list.enumerate() {
        println!("{:?}", pass);
    }

    //let result = pass_list.iter().filter(|pass| pass.iter().count() == 8).count();
    //println!("{}", result);
}