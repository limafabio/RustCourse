fn magnitude(v:[f64;3]) -> f64 {
    let sum = v[0]*v[0] + v[1]*v[1] + v[2]*v[2];
    sum.sqrt()
}

fn normalize(v:[f64;3]) -> [f64;3] {

      let m = magnitude(v);
      assert_ne!(0.0,m);
      [v[0]/m,v[1]/m,v[2]/m]   
}

fn main() {
    println!("Magnitude of a unit vector: {}",magnitude([0.0, 1.0, 0.0]));
    let z = [1.0, 2.0, 9.0];
    println!("Magnitude of a unit vector: {}", magnitude(z));
    println!("Magnitude of {z:?}: {}", magnitude(z));
    let x = normalize(z);
    println!("Magnitude of {x:?} after normalization: {}", magnitude(x));
}
