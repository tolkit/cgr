pub mod cgr {

    const C: (f64, f64) = (-1f64, 1f64);
    const G: (f64, f64) = (1f64, 1f64);
    const A: (f64, f64) = (-1f64, -1f64);
    const T: (f64, f64) = (1f64, -1f64);

    #[derive(Debug, Clone)]
    pub struct CGR {
        pub x: Vec<f64>,
        pub y: Vec<f64>,
    }

    // based on https://github.com/WillCheney/Chaos-Game-Representation/blob/master/CGR%20and%20Classification%20of%20genomes/DNA_CGR_generation.py

    pub fn generate_cgr(dna: &[u8]) -> CGR {
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();

        x.push(0f64);
        y.push(0f64);

        // 71 == G; 103 == g
        // 67 == C; 99 == c
        // 65 == A; 97 == a
        // 84 == T; 116 == t
        // 78 == N; 110 == n

        for base in dna {
            match base {
                78 | 110 => {
                    continue;
                }
                71 | 103 => {
                    let xlast = x.last().unwrap_or(&0f64);
                    let ylast = y.last().unwrap_or(&0f64);
                    let nextx = (xlast + G.0) / 2f64;
                    let nexty = (ylast + G.1) / 2f64;
                    x.push(nextx);
                    y.push(nexty);
                }
                67 | 99 => {
                    let xlast = x.last().unwrap_or(&0f64);
                    let ylast = y.last().unwrap_or(&0f64);
                    let nextx = (xlast + C.0) / 2f64;
                    let nexty = (ylast + C.1) / 2f64;
                    x.push(nextx);
                    y.push(nexty);
                }
                65 | 97 => {
                    let xlast = x.last().unwrap_or(&0f64);
                    let ylast = y.last().unwrap_or(&0f64);
                    let nextx = (xlast + A.0) / 2f64;
                    let nexty = (ylast + A.1) / 2f64;
                    x.push(nextx);
                    y.push(nexty);
                }
                84 | 116 => {
                    let xlast = x.last().unwrap_or(&0f64);
                    let ylast = y.last().unwrap_or(&0f64);
                    let nextx = (xlast + T.0) / 2f64;
                    let nexty = (ylast + T.1) / 2f64;
                    x.push(nextx);
                    y.push(nexty);
                }
                _ => {
                    continue;
                }
            }
        }
        CGR { x: x, y: y }
    }
}
