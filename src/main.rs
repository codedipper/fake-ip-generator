use rand::Rng;

fn main(){
    let mut ipv4 = String::new();
    let mut ipv6 = String::new();
    let ipv6_chars = [
        "0","1", "2", "3", "4", "5", "6", "7", "8", "9",
        "a", "b", "c", "d", "e", "f"
    ];

    for i in 0..4 {
        ipv4 += &gen(0, 255).to_string();
        if i != 3 { ipv4 += "."; }
    }

    for i in 1..33 {
        ipv6 += ipv6_chars[gen(0, 15)];
        if i % 4 == 0 && i != 32 { ipv6 += ":"; }
    }

    println!("{}\n{}", ipv4, ipv6);
}

fn gen(min: u32, max: u32)-> usize {
    return rand::thread_rng()
        .gen_range(min..=max)
        .try_into()
        .unwrap();
}
