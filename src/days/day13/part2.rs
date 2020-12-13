use crate::days::day13::default_input;

pub fn run() {
    println!("{}", busses_str(default_input()).unwrap())
}

pub fn busses_str(input : &str) -> Result<i64, ()> {
    let it = input.lines();
    let busses: Vec<_> = it.skip(1).next().unwrap()
        .split(",")
        .map(|x| match x {
            "x" => None,
            _ => Some(x.parse::<i64>().unwrap())
        })
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| {
            if i == 0 {
                (i, x.unwrap())
            } else {
                (x.unwrap() as usize - (i % x.unwrap() as usize), x.unwrap())                                   //Convert enumerated value to correct positive remainder.
            }                                                                                                   //Should be the opposite of mod
        })
        .collect();
    let mods : Vec<_> = busses.iter().map(|(_, x)| *x).collect();                                    //Split of only modulos
    let product = mods.iter().fold(1 as i64, |acc, x| acc * x);                         //Calculate product of modulos
    let coefficients: Vec<_> = mods.iter().map(|x| product / *x).collect();                          //Get values to calculate the coefficients
    let inverses: Vec<_> = coefficients.iter().zip(mods.iter()).map(|(b, modulo)| {     //Get the inverses
        inverse_mod(*b, *modulo)
    }).collect();
    let res : i64 = busses.iter().map(|(x, _)| *x as i64).enumerate().map(|(i, bus)| {      //Final summation
        bus * coefficients.get(i).unwrap() * inverses.get(i).unwrap()
    }).sum();
    Ok(res % product)                                                                                           //Get smallest possible value
}


fn inverse_mod(mut x: i64, mut m: i64) -> i64 {                                                                 //Calculates the inverse mod using extended Euclid
    if m == 1 {
        return 0;
    }
    let m0 = m;
    let mut x0 = 0;
    let mut x1 = 1;
    while x > 1 {
        let q = x / m;
        let mut t = m;
        m = x % m;
        x = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 = x1 + m0
    }
    x1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part2_answer() {
        assert_eq!(busses_str(default_input()).unwrap(), 327300950120029)
    }

    #[test]
    pub fn example() {
        assert_eq!(busses_str("939
17,x,13,19").unwrap(), 3417)
    }
}
