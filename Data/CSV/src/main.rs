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
}
