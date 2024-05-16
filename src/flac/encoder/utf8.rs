pub struct Utf8Encoder;

impl Utf8Encoder {
    /// Encode a number into its UTF-9 equivalent encoding
    /// 
    /// Although UTF-8 encoding is for characters, characters are
    /// mapped to certain numbers.
    pub fn encode(mut num: u64) -> Vec<u8> {
        
        let num_vec: Vec<u8> = int_to_bin(num);
        let mut bin_temp: Vec<u8> = Vec::new();

        println!("{:?}", num_vec);

        if num_vec.len() <= 7 {
            bin_temp = vec![0,2,2,2,2,2,2];
        } else if num_vec.len() <= 11 {
            bin_temp = vec![1,1,0,2,2,2,2,2,1,0,2,2,2,2,2,2];
        } else if num_vec.len() <= 16 {
            bin_temp = vec![1,1,1,0,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2];
        } else if num_vec.len() <= 21 {
            bin_temp = vec![1,1,1,1,0,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2];
        } else if num_vec.len() <= 26 {
            bin_temp = vec![1,1,1,1,1,0,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2];
        } else if num_vec.len() <= 31 {
            bin_temp = vec![1,1,1,1,1,1,0,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2];
        } else if num_vec.len() <= 40 {
            bin_temp = vec![1,1,1,1,1,1,1,0,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2];
        }

        let mut bit_sel = bin_temp.len()-1;

        for i in 0..num_vec.len() {

            while bin_temp[bit_sel] != 2 {
                bit_sel -= 1;
            }

            bin_temp[bit_sel] = num_vec[i];

        }

        for i in 0..bin_temp.len() {

            if bin_temp[i] == 2 {
                bin_temp[i] = 0;
            }

        }

        return bin_temp;

    }
    
}

pub fn int_to_bin(mut int_fmt: u64) -> Vec<u8> {

    let mut bin_fmt: Vec<u8> = Vec::new();

    while int_fmt > 0 {

        bin_fmt.push((int_fmt - int_fmt/2*2) as u8);
        int_fmt = int_fmt/2;

    }
    
    return bin_fmt;

}