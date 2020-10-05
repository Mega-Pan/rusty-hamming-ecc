pub mod rustyham{
    use std::iter::repeat;
    use std::convert::TryInto;

    pub enum Hamming { Encode, Decode }

    pub fn hamming(action:Hamming, s:String)->String {
        match action {
            Hamming::Encode => {
                // convert ASCII string to binary
                let bytes = s.into_bytes();
                // covert into bit stream
                let bytes_str = 
                    bytes.iter().map(|&c| format!("{:0>1$b}",c,7))
                    .collect::<Vec<String>>().concat();
                let mut bytes_iter = bytes_str.chars();

                // compute block and message length;
                let mlen = bytes_str.len() as u32;
                let lenpow = (2..).find(|&r| 2u32.pow(r)-r-1 >= mlen).unwrap();
                let len = 2usize.pow(lenpow-1);

                // the object where we store encoded hamming code
                let mut code: Vec<bool> = repeat(false).take(len).collect();
                
                // set data bits
                for i in 1..len {
                    if (i & (i-1)) != 0 {   // if i is not a power of 2 
                        code [i-1] = bytes_iter.next().unwrap_or('0') == '1';
                    }
                }
                
                // set parity bits
                for i in 0..lenpow {
                    code[2usize.pow(i) -1] = calc_parity(&code,i);
                }

                return code.into_iter().map(|x| if x {"1"} else {"0"})
                    .collect::<Vec<_>>().concat()
            }
            Hamming::Decode => {
                // verify parity bits, fix 1-bit-flipped error if any
                let len = s.len();
                let lenpow = ((len+1)as f32).sqrt().round() as u32;
                let mut chars = s.chars().map(|x| x == '1').collect::<Vec<bool>>();
                let mut flipped_bit = -1i32;
                while(0..lenpow).any(|i|calc_parity(&chars, i)){
                    if flipped_bit != -1 {
                        chars[flipped_bit as usize] = !chars[flipped_bit as usize];
                    }
                    flipped_bit += 1;
                    chars[flipped_bit as usize] = !chars[flipped_bit as usize];
                }

                // collect all bits non-power of 2
                let data = chars.iter().enumerate()
                    .filter(|x|((x.0+1)&x.0) !=0)
                    .map(|x| if *x.1{'1'} else {'0'});

                // remove '0' padding
                let cslice = &data.collect::<Vec<char>>()[..];
                let mut chunks = cslice.chunks(7).map(|x| {
                    x.iter().cloned().collect::<String>()
                }).collect::<Vec<String>>();
                
                if chunks[chunks.len()-1].len() < 7 {chunks.pop();}
                while chunks[chunks.len()-1] == "0000000" {chunks.pop();}

                let chars = chunks.iter()
                    .map(|x| u8::from_str_radix(&x[..], 2).unwrap())
                    .collect::<Vec<u8>>();

                String::from_utf8(chars).unwrap()
            }
        }
    }

    fn calc_parity(code: &Vec<bool>, i:u32) -> bool {
        let bi = 2usize.pow(i) -1;
        let (mut parity, mut ignore, mut counter) = (false, false, 0);
        for j in bi..code.len(){
            if !ignore && code[j] {parity = !parity }
            counter += 1;
            if counter >= 2u32.pow(i.try_into().unwrap()) {
                ignore = !ignore;
                counter = 1;
            }
        }
        parity
    }
}

