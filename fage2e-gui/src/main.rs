use strum::IntoEnumIterator;

use fage2e;

fn main() {
    for ability in fage2e::Ability::iter() {
        println!("{}:", ability);
        for focus in ability.focuses() {
            println!("\t{}", focus.base_name());
        }
    }
}
