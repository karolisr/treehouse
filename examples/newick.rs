use treehouse::parse_newick;
fn main() {
    let data = "(((×Five:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7)×Eight×:0.8,×Nine×:0.9)Ten×:1.0;";
    let data = String::from(data);
    println!("\n{data}\n");
    let tree = parse_newick(data);
    println!("{:?}", tree);
}
