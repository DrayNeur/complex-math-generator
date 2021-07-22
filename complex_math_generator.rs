use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut result: String = String::new();
    let mut last_integer: i64 = 5;

    let mut parent_count: i64 = 0;
    for i in 1..100 {
        let operation = rng.gen_range(0..3);
        if operation == 1 {
            let _minus = rng.gen_range(1..999999);
            if last_integer-_minus == 0 {
                continue;
            }
            result +=  &("(".to_owned()+&(last_integer-_minus).to_string()+"+");
            last_integer = _minus;
            parent_count+=1;
        } else if operation == 2 {
            let _add = rng.gen_range(1..999999);
            if last_integer+_add == 0 {
                continue;
            }
            result += &("(".to_owned()+&(last_integer+_add).to_string()+"-");
            last_integer = _add;
            parent_count+=1;
        }
    }
    result += &last_integer.to_string();
    for i in 0..parent_count {
        result+= ")";
    }
    println!("{}", result);
}