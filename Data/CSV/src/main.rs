//type Value = String;
//type Row = Vec<Value>;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please supply a command and a filename.");
        return
    }
    let command = args.get(1).unwrap();
    let filename = args.get(2).unwrap();

    let _csv_string = std::fs::read_to_string(filename);

    match command.as_str() {
        "decode" => {
            todo!();
        }
        "encode" => {
            todo!();
        }
        _ => {
            println!("Unknown command: {:?}", command);
            return
        }
    }
}
