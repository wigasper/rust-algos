#[derive(Default)]

pub struct LinearModel {
    coeffs: Vec<f64>,
    intercept: f64,
}

impl LinearModel {
    fn fit(&mut self, x: Vec<Vec<f64>>, y: Vec<f64>) {
        let mut coeffs = Vec::new();
        let mut x_means = Vec::new();

        let y_sum: f64 = y.iter().sum();
        let y_count: f64 = y.len() as f64;
        let y_mean: f64 = y_sum / y_count;

        for term in x.iter() {
            let term_sum: f64 = term.iter().sum();
            let term_count: f64 = term.len() as f64;
            let term_mean: f64 = term_sum / term_count;
     
            let mut numerator: f64 = 0.0;
            for (index, val) in term.iter().enumerate() {
                numerator += (val - term_mean) * (y[index] - y_mean);
            }

            let mut denom: f64 = 0.0;
            for val in term.iter() {
                denom += (val - term_mean) * (val - term_mean);
            }
            
            x_means.push(term_mean);
            coeffs.push(numerator / denom);
        }

        let mut a_hat: f64 = y_mean;
        
        for (index, coeff) in coeffs.iter().enumerate() {
            a_hat = &a_hat - coeff * x_means[index];
        }

        self.coeffs = coeffs;
        self.intercept = a_hat;
    }

    //fn predict(&self, x: Vec<f64>) {
    
    //}
}
