pub fn score(upvotes: i32, downvotes: i32) -> f32 {
    const A: f64 = 3.0;
    const B: f64 = 3.0;
    const ZA2: f64 = 0.56;
    const N: f64 = 6.0;
    const R0: f64 = 0.6;
    const N0: f64 = 1.0;

    let (upv, dwv) = (upvotes as f64, downvotes as f64);

    let n = upv + dwv + A + B;
    let p = (upv + A) / n;

    let r = (p + (ZA2*ZA2)/(2.0*n) - ZA2 * ((p * (1.0 - p) + ZA2*ZA2/(4.0*n))/n).sqrt())/(1.0 + ZA2*ZA2/n);

    let nw = upv + dwv + N0;

    (r * nw / (nw + N) + R0 * N / (nw + N)) as f32
}