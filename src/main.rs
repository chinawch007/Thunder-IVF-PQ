use rand::Rng;
use Thunder_IVF_PQ::get_centroids;

fn main() {
    let mut r = rand::thread_rng();
    let n1: u8 = r.r#gen();
    
    println!("{}", n1);

    get_centroids(&vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]], 2);
}