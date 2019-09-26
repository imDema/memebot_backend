pub fn score(upvotes: i32, downvotes: i32) -> f32 {
    const a: f64 = 3.0;
    const b: f64 = 3.0;
    const za2: f64 = 0.56;
    const N: f64 = 6.0;
    const r0: f64 = 0.6;
    const n0: f64 = 1.0;

    let (upv, dwv) = (upvotes as f64, downvotes as f64);

    let n = upv + dwv + a + b;
    let p = (upv + a) / n;

    let r = (p + (za2*za2)/(2.0*n) - za2 * ((p * (1.0 - p) + za2*za2/(4.0*n))/n).sqrt())/(1.0 + za2*za2/n);

    let nw = upv + dwv + n0;

    (r * nw / (nw + N) + r0 * N / (nw + N)) as f32
}