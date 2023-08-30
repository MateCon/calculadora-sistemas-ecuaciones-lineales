use core::fmt;
use std::io::{self, Write};

/* Ej
 * Input:
 * 2
 * 3
 * 1 2 3
 * 2 3 1
 * Output:
 * 1   2|  3 
 * 0 0.5|2.5
 */

fn main() {
    let mut matrix = Matrix {
        i: usize_input("Ingrese la cantidad de filas de la matriz (i): "), 
        j: usize_input("Ingrese la cantidad de columnas de la matriz (j): "),
        content: vec![]
    };

    matrix.load_content();

    matrix.gauss_elimination();

    println!("La matriz escalonada es:");
    println!("{matrix}");
}

struct Matrix {
    i: usize,
    j: usize,
    content: Vec<Vec<f32>>,
}

impl Matrix {
    fn load_content(&mut self) {
        println!("Ingrese su matriz: ");

        let mut input: String;

        for _ in 0..self.i {
            input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let row: Vec<f32> = input
                .trim()
                .split(" ")
                .map(|s| { s.parse().expect("All inputs must be numbers") })
                .collect();

            if row.len() < self.j {
                panic!("Demasiados pocos numeros");
            } else if row.len() > self.j {
                panic!("Demasiados numeros");
            }

            self.content.push(row);
        }
    }

    fn gauss_elimination(&mut self) {
        let width = self.get_row(0).len();
        let height = self.content.len();

        if height > width {
            panic!("No manejo cuando hay mas filas que columnas");
        }

        let mut shift_i = 0;

        while shift_i < height {
            /* Consigo un vector que me dice cuantos ceros hay al principio de cada fila
            * Ej: Para la matriz
            *   1 2 3
            *   0 0 5
            * El vector sería <0, 2>
            */
            let mut zero_count: Vec<i16> = vec![];
            let mut curr_zero_count;

            for row in self.content.iter() {
                curr_zero_count = 0;
                for value in row.iter() {
                    if *value == 0.0 {
                        curr_zero_count += 1;
                    } else {
                        break;
                    }
                }
                zero_count.push(curr_zero_count);
            }

            /* Busco el menor valor de zero_count, si hay varios no importa cual es, 
            * así que tomo el primero y hago swap_rows entre la primera fila y el indice que consiga
            * Ej:
            *   <0, 1> -> self.swap_rows(0, 1)
            *   <4, 2, 1, 3> -> self.swap_rows(0, 2)
            */
            let mut min_zero_count = i16::MAX;
            let mut min_zero_count_index: usize = 0;

            for (i, value) in zero_count.iter().enumerate() {
                if i < shift_i {
                    continue;
                }

                if min_zero_count > *value {
                    min_zero_count = *value;
                    min_zero_count_index = i;
                }
            }

            self.swap_rows(shift_i, min_zero_count_index);

            /* Ahora que se que el primer valor de la primer fila no es cero
            * para cada otra fila que no tenga 0 en el indice 1 multiplico
            * la fila por el opuesto del cociente entre el primer valor de la 
            * primera fila y el valor de la fila n
            * Ej:
            *   3 1               3  1
            *   2 2 F2*(-3/2) -> -3 -3
            * 
            * Después le sumo la fila 0 a la fila n
            *  3  1       3  1
            * -3 -3 F2+F1 0 -2
            */
            for i in shift_i+1..self.i {
                let value1 = self.get_value(shift_i, min_zero_count_index);
                let value2 = self.get_value(i, min_zero_count_index);

                if value2 != 0.0 { 
                    let k = value1 / value2;
                    self.multiply_row(i, -k);
                    self.sum_row(i, shift_i);
                }
            }

            shift_i += 1;
        }
    }

    fn swap_rows(&mut self, i1: usize, i2: usize) {
        let mut temp: f32;

        for j in 0..self.j {
            temp = self.get_value(i1, j);
            self.set_value(i1, j, self.get_value(i2, j));
            self.set_value(i2, j, temp);
        }
    }

    fn multiply_row(&mut self, i: usize, k: f32) {
        let row = self.get_mut_row(i);

        for value in row.iter_mut() {
            *value *= k;
        }
    }

    fn sum_row(&mut self, i1: usize, i2: usize) {
        for j in 0..self.j {
            *self.get_mut_value(i1, j) += self.get_value(i2, j);
        }
    }

    fn get_mut_row(&mut self, i: usize) -> &mut Vec<f32> {
        match self.content.get_mut(i) {
            Some(val) => val,
            None => panic!("i is out of bounds"),
        }
    }

    fn get_row(&self, i: usize) -> &Vec<f32> {
        match self.content.get(i) {
            Some(val) => val,
            None => panic!("i is out of bounds"),
        }
    }

    fn get_mut_value(&mut self, i: usize, j: usize) -> &mut f32 {
        match self.get_mut_row(i).get_mut(j) {
            Some(val) => return val,
            None => panic!("j is out of bounds"),
        };
    }

    fn get_value(&self, i: usize, j: usize) -> f32 {
        match self.get_row(i).get(j) {
            Some(val) => return *val,
            None => panic!("j is out of bounds"),
        };
    }

    fn set_value(&mut self, i: usize, j: usize, value: f32) {
        match self.content.get_mut(i) {
            Some(row) => match row.get_mut(j) {
                Some(val) => *val = value,
                None => panic!("j is out of bounds"),
            },
            None => panic!("i is out of bounds")
        };
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut column_lengths: Vec<usize> = vec![];

        for j in 0..self.j {
            let mut column_length = 0;
            for i in 0..self.i {
                let curr_length = self.get_value(i, j).to_string().len();
                if curr_length > column_length {
                    column_length = curr_length;
                }
            }
            column_lengths.push(column_length);
        }

        for row in self.content.iter() {
            for (j, value) in row.iter().enumerate() {
                let mut extra_spaces = String::new();
                let space_count = column_lengths[j] - value.to_string().len();
                for _ in 0..space_count {
                    extra_spaces.push(' ');
                }
                let after_char = if j == self.j - 2 {
                    '|'
                } else {
                    ' '
                };
                write!(f, "{}{}{}", extra_spaces, value, after_char)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn usize_input(message: &str) -> usize
{
    print!("{message}");
    io::stdout().flush().expect("Flush failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Expected a number")
}
