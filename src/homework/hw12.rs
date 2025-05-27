/*функція для обчислення мінімальної кількості переміщень
fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    
    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;

    for &weight in shipments.iter() {
        if weight > avg {
            moves += (weight - avg) as usize;
        }
    }

    moves as isize
}
*/

//функція генерації випадкових shipment з рівномірним розподілом
use rand::Rng; //rand = 0.8 в Cargo.toml

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(1..=100); 
    let mut shipments = vec![avg; n];
    
    
    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 0 {
            let delta = rng.gen_range(1..=shipments[i].min(5));
            shipments[i] -= delta;
            shipments[j] += delta;
        }
    }

    shipments
}
