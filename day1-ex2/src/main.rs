#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    print!(
        r"|{} {} {}|
|{} {} {}|
|{} {} {}|
",
        matrix[0][0], //
        matrix[0][1],
        matrix[0][2],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
    );
}

fn pretty_print_slice(matrix: &[&[i32]]) {
    print!(
        r"|{} {} {}|
|{} {} {}|
|{} {} {}|
",
        matrix[0][0], //
        matrix[0][1],
        matrix[0][2],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    let x = &&matrix[0..1][0..1];
    println!("x = {:?}", x);
    pretty_print(&matrix);
    // pretty_print_slice(&matrix[0..]);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
