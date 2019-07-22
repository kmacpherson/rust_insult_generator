extern crate rand;

use rand::Rng;

fn main() {
    let insult_part_1 = ["Atomic", "Steamy", "Rusty", "Witless", "Lumpy", "Shitty", "Moist", "Chunky", "Lousy", "Bulbous", "Trashy", "Dumbass", "Nerdy", "Dotarded", "Crusty", "Brainless"];
    let insult_part_2 = ["Knob", "Bum", "Turd", "Prick", "Bulge", "Ass", "Chut", "Shit", "Rod", "Chode", "Fuck", "Wiener", "Jizz", "Panty", "Cock", "Dong"];
    let insult_part_3 = ["Vacuum", "General", "Pixie", "Spasm", "Fiend", "Fungus", "Tunnel", "Corporal", "Raider", "Demon", "Buccaneer", "Tyrant", "Juggler", "Magician", "Fiddle"];

    let part_1 = rand::thread_rng().gen_range(1, insult_part_1.len());
    let part_2 = rand::thread_rng().gen_range(1, insult_part_2.len());
    let part_3 = rand::thread_rng().gen_range(1, insult_part_3.len());

    println!("Insult is: {} {} {}", insult_part_1[part_1], insult_part_2[part_2], insult_part_3[part_3]);
}