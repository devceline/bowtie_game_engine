use std::collections::HashMap;

pub const PI: f32 = 3.14159265358979;

fn factorial_memoized(num: i32, memo_arr: *mut HashMap<i32, i32>) -> i32 {
  if num == 1 || num == 0 {
    return 1;
  }

  unsafe {
  let result: i32 = match (*memo_arr).get(&num) {
    Some(res) => res.to_owned(),
    None => {
      let n_less_1 = factorial_memoized(num.to_owned() - 1, memo_arr) * num.to_owned();
      (*memo_arr)
        .insert(num.to_owned(), n_less_1);
      return n_less_1;
    }
  };

  return result;
  }
}

pub fn factorial(num: i32) -> i32 {
  let mut memo_arr = HashMap::<i32, i32>::new();
  println!("HERE");
  return factorial_memoized(num, &mut memo_arr);
}

pub fn get_radian(theta: f32) -> f32 {
  (theta / 180.0) * PI
}

pub fn get_sin(theta: f32) -> f32 {
  let radian = get_radian(theta);
  let mut sin = 0.0;
  let mut sub = true;
  for i in 0..7 {
    let res = radian.powi(1 + i) / (1 + i) as f32;
    sin = if sub { sin - res } else { sin + res };
    sub = !sub;
  }
  sin
}
