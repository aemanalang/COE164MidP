pub struct VarPredictor;

impl VarPredictor {
    /// Get the correlation of a vector of data
    pub fn get_autocorrelation(data: &Vec <i32>, lag: u32) -> Vec <f64> {
        todo!()
    }

    /// Get the predictor coefficients
    /// 
    /// The coefficients are computed using the Levinson-Durbin algorithm.
    pub fn get_predictor_coeffs(autoc: &Vec <f64>, predictor_order: u32) -> Vec <f64> {
        todo!()
    }

    pub fn quantize_coeffs(lpc_coefs: &Vec <f64>, mut precision: u32) -> (Vec <u32>, u32) {
        todo!()
    }

    /// Compute the residuals from a given linear predictor
    /// 
    /// The residuals are computed with the provided quantized coefficients
    /// `qlp_coefs` and shift factor `qlp_shift`.
    pub fn get_residuals(data: &Vec <i32>, qlp_coefs: &Vec <u32>, predictor_order: u32, qlp_shift: u32) -> Option <Vec <i32>> {
        todo!()
    }

    /// Get the best coefficient precision
    /// 
    /// FLAC uses the bit depth and block size to determine the best coefficient
    /// precision. By default, the precision is 14 bits but can be one of the
    /// following depending on several parameters:
    /// 
    /// | Bit depth | Block size |     Best precision      |
    /// |-----------|------------|-------------------------|
    /// |   < 16    |     any    | max(1, 2 + bit_depth/2) |
    /// |     16    |     192    |           7             |
    /// |     16    |     384    |           8             |
    /// |     16    |     576    |           9             |
    /// |     16    |    1152    |          10             |
    /// |     16    |    2304    |          11             |
    /// |     16    |    4608    |          12             |
    /// |     16    |     any    |          13             |
    /// |   > 16    |     384    |          12             |
    /// |   > 16    |    1152    |          13             |
    /// |   > 16    |     any    |          14             |
    pub fn get_best_precision(bps: u32, block_size: u32) -> u32 {
        todo!()
    }
}