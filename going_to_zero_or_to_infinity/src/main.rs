fn main() {
    println!("{}", going(500000000));
}

fn going(n: i32) -> f64 {
    let mut res: f64 = 1.0;
    let mut inter: f64 = 1.0;
    let mut i: i32 = n;

    while i >= 2 {
        inter = inter * (1.0 / i as f64);
        res += inter;
        i -= 1
    }

    (res * 1e6).floor() / 1e6
}

//const TEN: f64 = 10f64;
//
//struct BigDecimal {
//    value: f64,
//    scale: i32
//}
//
//impl BigDecimal {
//    fn new(value: f64, scale: i32) -> BigDecimal {
//        BigDecimal { value, scale }
//    }
//
//    fn mul(&self, other: i32) -> BigDecimal {
//        let mut value = self.value * other as f64;
//        let mut scale = self.scale;
//
//        while value > TEN {
//            value /= TEN;
//            scale += 1;
//        }
//
//        BigDecimal { value, scale }
//    }
//
//    fn pls(&self, other: &BigDecimal) -> BigDecimal {
//        let difference = other.scale - self.scale;
//
//        if difference == 0 {
//            let mut value = self.value + other.value;
//            let mut scale = self.scale;
//
//            while value > TEN {
//                value /= TEN;
//                scale += 1;
//            }
//
//            return BigDecimal { value, scale };
//        }
//
//        let mut value = other.value + self.value / TEN.powi(difference);
//        let mut scale = other.scale;
//
//        while value > TEN {
//            value /= TEN;
//            scale += 1;
//        }
//
//        BigDecimal { value, scale }
//    }
//
//    fn div(&self, other: &BigDecimal) -> f64 {
//        self.value / other.value * TEN.powi(self.scale - other.scale)
//    }
//}
//
//fn going(n: i32) -> f64 {
//    let mut nominator = BigDecimal::new(0.1, 1);
//    let mut denominator = BigDecimal::new(0.1, 1);
//
//    for x in 2..n + 1 {
//        denominator = denominator.mul(x);
//        nominator = nominator.pls(&denominator);
//    }
//
//    return nominator.div(&denominator);
//}
