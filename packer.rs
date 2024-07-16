const bits: [u8;18] = [4, 7, 10, 14, 17, 20, 24, 27, 30, 34, 37, 40, 44, 47, 50, 54, 57, 60];
const typebits: [u8;5] = [1, 8, 16, 32, 64];

#[derive(Debug)]
struct Item {
    f32: Vec<f32>,
    f64: Vec<f64>,
    u8: Vec<u8>,
    u16: Vec<u16>,
    u32: Vec<u32>,
    u64: Vec<u64>,
    bool: Vec<bool>
}

impl Item {
    fn new () -> Item {
        Item {f32: Vec::new(), f64: Vec::new(), u8: Vec::new(), u16: Vec::new(), u32: Vec::new(), u64: Vec::new(), bool: Vec::new()}
    }
    fn concat (&mut self, item: Item) -> Item {
        let f64 = [&self.f64[..], &item.f64[..]].concat();
        let f32 = [&self.f32[..], &item.f32[..]].concat();
        let u64 = [&self.u64[..], &item.u64[..]].concat();
        let u32 = [&self.u32[..], &item.u32[..]].concat();
        let u16 = [&self.u16[..], &item.u16[..]].concat();
        let u8 = [&self.u8[..], &item.u8[..]].concat();
        let bool = [&self.bool[..], &item.bool[..]].concat();
        Item {f64, f32, u64, u32, u16, u8, bool}
    }
}

fn unpack(value: u64, fields: &Vec<u8>) -> Item {
    let mut pos = 64;
    let mut results = Item::new();
    for i in (0..fields.len()).step_by(2) {
        pos -= fields[i];
        let int = ((value >> pos) & (u64::pow(2, fields[i].into()) - 1)) as f64;
        let floating = fields[i + 1];
        if floating > 0 {
        pos -= floating;
        let float = ((value >> pos) & (u64::pow(2, floating.into()) - 1)) as f64 * f64::powf(0.1, (bits.iter().position(|&num| num > floating).unwrap_or(usize::MAX)) as f64);
        let value = int + float; 
        if fields[i] + fields[i + 1] <= 32 {
        results.f32.push(value as f32);
        } else {
        results.f64.push(value);
        }
        } else {
        match typebits.iter().position(|&num| num >= fields[i]).unwrap_or(usize::MAX) {
            0 => {
                results.bool.push(int != 0.0);
            },
            1 => {
                results.u8.push(int as u8);  
            },
            2 => {
                results.u16.push(int as u16);
            },
            3 => {
                results.u32.push(int as u32);
            },
            4 => {
                results.u64.push(int as u64);
            },
            _ => ()
        }
        }
    }
    results
}

fn calculate_int(value: u64, intbits: usize) -> String {
    format!("{:0>intbits$}", format!("{:b}", value))
}

fn calculate_float(value: f64, floatbits: usize) -> String {
    format!("{:0>floatbits$}", format!("{:b}", (value.fract() * f64::powf(10.0, (bits.iter().position(|&num| num > floatbits as u8).unwrap_or(usize::MAX)) as f64)) as u64))
}

fn pack(item: Item, fields: &Vec<u8>) -> u64 {
    let mut result = "".to_string();
    let mut ptr = 0;
    for i in item.f64 {
        let int = calculate_int(i as u64, fields[ptr] as usize);
        let float = calculate_float(i.fract(), fields[ptr + 1] as usize);
        ptr += 2;
        result += (int + &float).as_str();
    }
    for i in item.f32 {
        let int = calculate_int(i as u64, fields[ptr] as usize);
        let float = calculate_float((i as f64).fract(), fields[ptr + 1] as usize);
        ptr += 2;
        result += (int + &float).as_str();
    }
    for i in item.u64 {
        let int = calculate_int(i, fields[ptr] as usize);
        ptr += 2;
        result += int.as_str();
    }
    for i in item.u32 {
        let int = calculate_int(i as u64, fields[ptr] as usize);
        ptr += 2;
        result += int.as_str();
    }
    for i in item.u16 {
        let int = calculate_int(i as u64, fields[ptr] as usize);
        ptr += 2;
        result += int.as_str();
    }
    for i in item.u8 {
        let int = calculate_int(i as u64, fields[ptr] as usize);
        ptr += 2;
        result += int.as_str();
    }
    for i in item.bool {
        ptr += 2;
        result += (i as u8).to_string().as_str();
    }
    u64::from_str_radix(result.as_str(), 2).unwrap()
}

fn main() {
    let packer = vec![12, 7, 12, 7, 3, 7, 15, 0, 1, 0];
    let packed = pack(Item { f32: vec![108.34, 109.12, 7.32], f64: Vec::new(), u8: Vec::new(), u16: vec![50], u32: Vec::new(), u64: Vec::new(), bool: vec![true] }, &packer);
    println!("{:?}", packed);
    println!("{:?}", unpack(packed, &packer));
}
