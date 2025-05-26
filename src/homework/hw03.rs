const W: usize = 30;
const H: usize = 20;

fn main() {
    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            let c = if y == 0 || y == H - 1 || x == 0 || x == W - 1 {
                '*' // рамка
            } else if x == y * (W - 1) / (H - 1) {
                '*' // головна діагональ (зліва направо)
            } else if x == (H - 1 - y) * (W - 1) / (H - 1) {
                '*' // побічна діагональ (справа наліво)
            } else {
                ' '
            };
            output.push(c);
        }
        output.push('\n');
    }

    print!("{}", output);
}
