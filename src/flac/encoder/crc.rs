/// Represents a kind of CRC encoding
/// 
/// This struct is used to configure the type of CRC encoding to use.
/// For example, if the generator polynomial for a CRC8 encoding is:
/// 
/// `x^8 + x^2 + x^1 + 1`
/// 
/// Then, the value of `poly` should be 0b0000_0111 (note the missing
/// MSB `1` bit) and `poly_len` should be `u8`.
pub struct CrcOptions <T> {
    poly: T,
    poly_len: T,
}


impl <T> CrcOptions <T> {
    /// Create a builder to the CRC encoder
    pub fn new(poly: T, poly_len: T) -> Self {
        
        CrcOptions {poly: poly, poly_len: poly_len,}

    }

}

impl CrcOptions <u8> {
    /// Encode data using CRC8 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u8`.
    pub fn build_crc8(&self, data: &Vec <u8>) -> u8 {

        let mut div_orig: Vec<u8> = Vec::new(); // contains the dividend
        div_orig.append(&mut data.clone());
        div_orig.append(&mut vec![0; self.poly_len.into()]);

        let mut div = self.bin_to_int(&div_orig[0..self.poly_len as usize].to_vec());
        let base_two: u8 = 2;

        for i in 0..data.len() {

            if div >= base_two.pow((self.poly_len-1) as u32) { 

                // if MSB is 1, XOR div with poly
                div = div - base_two.pow((self.poly_len-1) as u32);
                div = div*2 + div_orig[self.poly_len as usize + i];
                div = div^self.poly;
                
            } else {

                // if MSB is 0, XOR div with 0
                div = div*2 + div_orig[self.poly_len as usize + i];
                div = div^0;

            }

        }

        return div;

    }

    pub fn combine_crc8(&self, mut data: Vec<u8>, mut checksum: u8) -> Vec<u8> {

        let base_two: u8 = 2;
        let mut data_with_crc: Vec<u8> = Vec::new();

        for i in 0..self.poly_len {

            let bit = checksum/base_two.pow((self.poly_len-1) as u32);
            checksum = checksum<<1;

            data.push(bit);

        }

        return data

    }

    fn bin_to_int(&self, bin_fmt: &Vec <u8>) -> u8 {

        let mut int_fmt: u8 = 0;
        let base_two: u8 = 2;

        for i in 0..self.poly_len {

            if bin_fmt[i as usize] == 1 {
                int_fmt = int_fmt + base_two.pow((self.poly_len-1-i) as u32);
            }
        
        }

        return int_fmt;

    }

}

impl CrcOptions <u16> {
    /// Encode data using CRC16 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u16`.
    pub fn build_crc16(&self, data: &Vec <u16>) -> u16 {

        let mut div_orig: Vec<u16> = Vec::new(); // contains the dividend
        div_orig.append(&mut data.clone());
        div_orig.append(&mut vec![0; self.poly_len.into()]);

        let mut div = self.bin_to_int(&div_orig[0..self.poly_len as usize].to_vec());
        let base_two: u16 = 2;

        for i in 0..data.len() {

            if div >= base_two.pow((self.poly_len-1) as u32) { 

                // if MSB is 1, XOR div with poly
                div = div - base_two.pow((self.poly_len-1) as u32);
                div = div*2 + div_orig[self.poly_len as usize + i];
                div = div^self.poly;
                
            } else {

                // if MSB is 0, XOR div with 0
                div = div*2 + div_orig[self.poly_len as usize + i];
                div = div^0;

            }

        }

        return div;

    }

    pub fn combine_crc16(&self, mut data: Vec<u16>, mut checksum: u16) -> Vec<u16> {

        let base_two: u16 = 2;
        let mut data_with_crc: Vec<u16> = Vec::new();

        for i in 0..self.poly_len {

            let bit = checksum/base_two.pow((self.poly_len-1) as u32);
            checksum = checksum<<1;

            data.push(bit);

        }

        return data

    }

    fn bin_to_int(&self, bin_fmt: &Vec <u16>) -> u16 {

        let mut int_fmt: u16 = 0;
        let base_two: u16 = 2;

        for i in 0..self.poly_len {

            if bin_fmt[i as usize] == 1 {
                int_fmt = int_fmt + base_two.pow((self.poly_len-1-i) as u32);
            }
        
        }

        return int_fmt;

    }

}

pub fn crc8_encode() {
    
    // CRC8 Portion

    let poly_crc8: u8 = 0b00000111;
    let poly_len_crc8: u8 = 8;

    let data_crc8: Vec<u8> = vec![1,0,1,0,1,0,1,0,1,0];
    // let data: Vec<u8> = vec![1,0,1,1];

    let builder_crc8 = CrcOptions::new(poly_crc8, poly_len_crc8);

    let checksum_crc8 = builder_crc8.build_crc8(&data_crc8);
    println!("{}", checksum_crc8);

    let data_with_crc8 = builder_crc8.combine_crc8(data_crc8.clone(), checksum_crc8);
    println!("{:?}", data_with_crc8);

    let check_crc8 = builder_crc8.build_crc8(&data_with_crc8);
    println!("{:?}", check_crc8);

    // CRC16 Portion

    let poly_crc16: u16 = 0b1000000000000101;
    let poly_len_crc16: u16 = 16;

    let data_crc16: Vec<u16> = vec![1,0,1,0,1,0,1,0,1,0];
    // let data: Vec<u16> = vec![1,0,1,1];

    let builder_crc16 = CrcOptions::new(poly_crc16, poly_len_crc16);

    let checksum_crc16 = builder_crc16.build_crc16(&data_crc16);
    println!("{}", checksum_crc16);

    let data_with_crc16 = builder_crc16.combine_crc16(data_crc16.clone(), checksum_crc16);
    println!("{:?}", data_with_crc16);

    let check_crc16 = builder_crc16.build_crc16(&data_with_crc16);
    println!("{:?}", check_crc16);

}   