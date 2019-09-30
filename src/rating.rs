use chrono::prelude::*;

pub fn score(upvotes: i32, downvotes: i32) -> f32 {
    const A: f64 = 3.0;
    const B: f64 = 3.0;
    const ZA2: f64 = 0.56;
    const N: f64 = 6.0;
    const R0: f64 = 0.55;
    const N0: f64 = 1.0;

    let (upv, dwv) = (upvotes as f64, downvotes as f64);

    let n = upv + dwv + A + B;
    let p = (upv + A) / n;

    let r = (p + (ZA2*ZA2)/(2.0*n) - ZA2 * ((p * (1.0 - p) + ZA2*ZA2/(4.0*n))/n).sqrt())/(1.0 + ZA2*ZA2/n);

    let nw = upv + dwv + N0;

    (r * nw / (nw + N) + R0 * N / (nw + N)) as f32
}

const HEAT_START: f32 = 10.0;
const HEAT_POS_INCREASE: f32 = 1.0;
const HEAT_DECAY_COEFF: f32 = - 0.00015;

pub fn heat_decay(heat: f32, time: NaiveDateTime, now: NaiveDateTime) -> f32 {
    let timespan = now
        .signed_duration_since(time)
        .num_minutes();

    heat * (HEAT_DECAY_COEFF * timespan as f32).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0_10_orders() {
        let mut v: Vec<(i32, i32, f32)> = Vec::with_capacity(11*11);
        for u in 0..11 {
            for d in 0..11 {
                v.push((u,d,score(u, d)));
            }
        }
        v.sort_unstable_by(|b, a| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        for (i, x) in v.iter().enumerate() {
            println!("{}: {:?}", i, x);
        }
    }

    #[test]
    fn test_decay_two_weeks() {
        let now = Local::now().naive_local();
        println!("Now: {}", now);
        for i in 0..15 {
            let postdate = now - chrono::Duration::days(i);
            let h = heat_decay(10.0, postdate, now);
            println!("Posted {}: {}", postdate, h);
        }
    }
}