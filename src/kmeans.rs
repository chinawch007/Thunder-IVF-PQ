use rand::seq::SliceRandom;

type Point = Vec<f64>;
type Cluster = Vec<Point>; // some points
type AllVectors = Vec<Cluster>; // all points but grouped by clusters

//random choose for now
pub fn get_centroids(vectors: &Cluster, k: usize) -> Cluster {
    //let mut centroids = Cluster::new();
    let mut centroids = vec![];
    let mut indices = (0..vectors.len()).collect::<Vec<usize>>();

    indices.shuffle(&mut rand::thread_rng());
        
    for i in 0..k {
        centroids.push(vectors[indices[i]].clone());
    }
    println!("Centroids: {:?}", centroids);
    
    centroids
}

// ivf_pq use the squared euclidean distance, so we don't sqrt
fn squared_euclidean_distance(v1 : &Point, v2 : &Point) -> f64 {
    v1.iter().zip(v2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        //.sqrt()
}

fn calculate_one_centroid(cluster: &Cluster) -> Point {
    let mut centroid = vec![0.0; cluster[0].len()];
    println!("Calculating centroid for cluster: {:?}", cluster);

    for point in cluster {
        for (i, value) in point.iter().enumerate() {
            centroid[i] += value;
        }
    }

    for i in 0..centroid.len() {
        centroid[i] /= cluster.len() as f64;
    }

    centroid
}

fn calculate_new_centroids(all_vectors: &AllVectors) -> Cluster {
    println!("Calculating new centroids for all clusters: {:?}", all_vectors);

    let mut new_centroid = vec![vec![0.0]; all_vectors.len()];

    for (i, cluster) in all_vectors.iter().enumerate() {
        new_centroid[i] = calculate_one_centroid(cluster);
    }

    new_centroid
}

// get all points and the number of clusters, return the clusters of points
pub fn k_means(vectors: &Cluster, k: usize) -> AllVectors {
    let mut centroids = get_centroids(&vectors, k);
    println!("Initial Centroids: {:?}", centroids);
    let mut clusters = vec![vec![]; k];

    // The rest of the k-means algorithm would go here, including:
    // 1. Assigning each vector to the nearest centroid to form clusters.
    // 2. Updating the centroids by calculating the mean of the vectors in each cluster.
    // 3. Repeating the process until convergence or a maximum number of iterations is reached.
 
    // for now we use the max rounds to limit the times of iterations,
    // but we can also use the convergence condition to stop the loop
    let mut i = 0;
    let max_rounds = 10;
    loop { //until convergence
        for v in vectors {
            let mut min_distance = f64::MAX;
            let mut closest_centroid = 0;
            
            for (i, centroid) in centroids.iter().enumerate() {
                let distance = squared_euclidean_distance(&v, &centroid);
                if distance < min_distance {
                    min_distance = distance;
                    closest_centroid = i;
                }
            }
            clusters[closest_centroid].push(v.clone());
            println!("Vector: {:?} is closest to Centroid: {:?}", v, centroids[closest_centroid]);
        }

        centroids = calculate_new_centroids(&clusters);
        
        i += 1;
        if i >= max_rounds {
            break;
        }
    }

    clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_centroids() {
        let vectors = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ];
        let k = 2;
        let centroids = get_centroids(&vectors, k);
        assert_eq!(centroids.len(), k);
        println!("Debug print vector{:?}", centroids);
    }

    #[test]
    fn test_squared_euclidean_distance() {
        let v1 = vec![1.0, 2.0];
        let v2 = vec![4.0, 6.0];
        let distance = squared_euclidean_distance(&v1, &v2);
        assert_eq!(distance, 25.0);
    }

    #[test]
    fn test_calculate_one_centroid() {
        let cluster = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
        ];
        let centroid = calculate_one_centroid(&cluster);
        assert_eq!(centroid, vec![3.0, 4.0]);
    }

    #[test]
    fn test_calculate_new_centroids() {
        let all_vectors = vec![
            vec![
                vec![1.0, 2.0],
                vec![3.0, 4.0],
            ],
            vec![
                vec![5.0, 6.0],
                vec![7.0, 8.0],
            ],
        ];
        let new_centroids = calculate_new_centroids(&all_vectors);
        assert_eq!(new_centroids, vec![vec![2.0, 3.0], vec![6.0, 7.0]]);
    }

    #[test]
    fn test_k_means() {
        let vectors = vec![
            vec![1.0, 2.0],
            vec![1.5, 1.8],
            vec![5.0, 8.0],
            vec![8.0, 8.0],
            vec![1.0, 0.6],
            vec![9.0, 11.0],
        ];
        let k = 2;
        let clusters = k_means(&vectors, k);
        assert_eq!(clusters.len(), k);

        for cluster in clusters {
            println!("Cluster: {:?}", cluster);
        }
    }
}
