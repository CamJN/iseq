fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() < 2 || a.len() > 3 {
        panic!("Two or three arguments needed");
    }else{
        let beg = a.get(0).unwrap().parse::<i64>().unwrap();
        let end = a.get(1).unwrap().parse::<i64>().unwrap();
        let forward = beg < end;
        let step = match a.get(2) {
            Some(step) => step.parse::<i64>().unwrap(),
            None => if forward { 1 } else { -1 }
        };
        if step == 0 {
            panic!("step can't be zero");
        }else if forward && step < 0 {
            panic!("if beg > end: step must be positive");
        }else if !forward && step > 0 {
            step *=-1;
        }
        let count = ((end - beg)/step).abs();

        //for i in (beg..end).step_by(step){
        for i in 0..(1+count) {
            println!("{}", beg + i*step);
        }
    }
}
