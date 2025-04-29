struct Matrix {
    mat: Vec<Vec<f64>>,
}


impl Matrix {
    fn new(height: usize, width: usize, content: f64) -> Matrix {
        let mat = vec![vec![content;width]; height];
        Matrix { mat }
    }

    fn zeros(height: usize, width: usize) -> Matrix {
        Self::new(height, width, 0.0)
    }

    fn identity(height: usize) -> Matrix {
        let mut mat = Self::zeros(height, height);
        for i in 0..mat.mat.len() {
            for j in 0..mat.mat[i].len() {
                mat.mat[i][j] = 1.0;
            }
        }
        return mat
    }

    fn get_element(&self, i: usize, j: usize) -> f64{
        
    }
}


fn main() {

}