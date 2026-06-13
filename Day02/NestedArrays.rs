fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix_t: [[i32; 3]; 3] = [[0; 3];3];
    for i in  0..matrix.len() {
      for j in 0..matrix[i].len() {
          matrix_t[j][i] = matrix[i][j];      
      }
    }
    return matrix_t;
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
     ];

     println!("Original:");
     for row in matrix {
         println!("{row:?}");
     }

     let transposed = transpose(matrix);

     println!("\nTransposed:");
     for row in transposed {
         println!("{row:?}");
     }
}
