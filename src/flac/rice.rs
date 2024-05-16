pub struct RiceEncoder;

impl RiceEncoder {

    pub fn encode(mut num: u64) -> Vec<u8> {

        let param = 16;
        let k = 4;
        let mut rice_encoding: Vec<u8> = Vec::new();

        let unary = num >> k;
        let mut bin = num & (param - 1);

        for i in 0..unary {
            rice_encoding.push(1);
        }
        
        rice_encoding.push(0);

        println!("{:?}", rice_encoding);

        for i in (0..k).rev() {
            rice_encoding.push((bin/(1<<i)) as u8);
            bin = bin - (bin/(1<<i))*(1<<i);
        }

        return rice_encoding;

    }

}