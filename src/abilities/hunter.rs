// use std::fmt;
//
// enum HunterAbilities {
//     Split_Shot,
//     Auto_Shot,
//     Trap,
//     Snipe,
// }
//
//
// enum WarriorAbilities {
//     Block,
//     Bash,
//     Taunt,
//     Barrier,
// }
//
// impl fmt::Display for HunterAbilities {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // First, get the debug string (e.g., "Split_Shot")
//         let debug_str = format!("{:?}", self);
//         // Replace underscores with spaces (e.g., "Split Shot")
//         let display_str = debug_str.replace('_', " ");
//         // Finally, write it out
//         write!(f, "{}", display_str)
//     }
// }