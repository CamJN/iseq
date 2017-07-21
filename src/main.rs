fn main() {
    let default = "Failed to get panic message.".to_string();
    std::panic::set_hook(Box::new(move |p|{
        eprintln!("{}",p.payload().downcast_ref::<std::string::String>().unwrap_or(&default));
        std::process::exit(-1);
    }));
    let mut a = std::env::args();
    if a.len() < 3 || a.len() > 4 {
        panic!("Two or three arguments needed");
    }else{
        let beg:i64 = a.nth(1).unwrap().parse().expect("first arg (beg) wasn't an integer");
        let end:i64 = a.next().unwrap().parse().expect("second arg (end) wasn't an integer");
        let forward = beg < end;
        let mut step:i64 = match a.next() {
            Some(step) => step.parse().unwrap(),
            None => if forward { 1 } else { -1 }
        };
        if step == 0 {
            panic!("step can't be zero");
        } else if beg == end {
            panic!("beg cannot equal end");
        } else if forward && (step < 0) {
            eprintln!("if beg < end: step must be positive, swapping sign");
            step *=-1;
        } else if !forward && (step > 0) {
            eprintln!("if beg > end: step must be negative, swapping sign");
            step *=-1;
        }
        let count = ((end - beg)/step).abs();

        // for i in (beg..end).step_by(step){
        //     println!("{}", i);
        // }
        for i in 0..(1+count) {
            println!("{}", beg + i*step);
        }
    }
}
