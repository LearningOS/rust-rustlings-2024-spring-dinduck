// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
<<<<<<< HEAD
    {
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
=======
    let y = &mut x;
    {
        *y += 100;
    }
    
        let z = &mut x;
>>>>>>> e7b90248970c29182cba86635a0ca686fadbd01e
    *z += 1000;
    assert_eq!(x, 1200);
}
