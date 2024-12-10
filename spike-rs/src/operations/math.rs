pub fn mean(range: &[f32]) -> f32 {
    let mut sum = 0f32;
    range.iter().for_each(|x| {
        sum += x;
    });
    sum / (range.len() as f32)
}

pub fn stdev(range: &[f32]) -> f32 {
    let mut sum = 0f32;
    let _mean = mean(range);
    range.iter().for_each(|x| {
        sum += (x - _mean) * (x - _mean);
    });
    (sum / (range.len() as f32 - 1.0f32)).sqrt()
}

pub fn min(range: &[f32]) -> f32 {
    let mut min = range[0];
    for value in range {
        if *value < min {
            min = *value;
        }
    }
    min
}

pub fn max(range: &[f32]) -> f32 {
    let mut max = range[0];
    for value in range {
        if *value > max {
            max = *value;
        }
    }
    max
}

pub fn train(n: usize, step: usize, offset: usize) -> Vec<f32> {
    let mut ret = vec![0f32; step * n + offset];
    for i in 0..n {
        ret[i * step + offset] = 1f32;
    }
    ret
}

pub fn convolve(s1: &[f32], s2: &[f32]) -> Vec<f32> {
    let (signal, filter) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    // let filter: Vec<f32> = filter.iter().rev().map(|x| *x).collect();
    let slen = signal.len();
    let flen = filter.len();

    let mut ret = vec![0f32; slen];

    // head
    for i in 0..flen {
        for j in (0..i).rev() {
            ret[i] += signal[i - j] * filter[j];
        }
    }

    // body
    for i in flen..(slen - flen) {
        for j in 0..flen {
            ret[i] += signal[i - j] * filter[j];
        }
    }

    // tail
    for i in (slen - flen)..slen {
        for j in slen - i..=0 {
            ret[i] += signal[i - j] * filter[j];
        }
    }

    ret
}

pub fn diff<T>(data: &[T]) -> Vec<T>
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    let mut ret = vec![T::default(); data.len() - 1];

    for i in 0..data.len() - 1 {
        ret[i] = data[i + 1] - data[i];
    }

    ret
}

pub fn logspace(start: f32, end: f32, n_points: usize) -> Vec<f32> {
    let mut ret = vec![0f32; n_points];

    let step = (end - start) / (n_points - 1) as f32;
    for i in 0..n_points {
        let exp = start + (i as f32 * step);
        let val = 10f32.powf(exp);
        ret[i] = val;
    }

    ret
}

pub fn tricube_weight(distance: f32, samples_fraction: f32, total_samples: usize) -> f32 {
    let span = (total_samples - 1) as f32 * samples_fraction;
    let d = distance.abs() / span;
    if d < 1f32 {
        (1f32 - d.powf(3f32)).powf(3f32)
    } else {
        0f32
    }
}

#[allow(non_snake_case)]
pub fn weighted_linear_regression(x: &[f32], y: &[f32], w: &[f32], x0: f32) -> f32 {
    let mut W = 0f32;
    let mut XW = 0f32;
    let mut YW = 0f32;
    let mut XXW = 0f32;
    let mut XYW = 0f32;

    for i in 0..x.len() {
        W += w[i];
        XW += x[i] * w[i];
        YW += y[i] * w[i];
        XXW += x[i] * x[i] * w[i];
        XYW += x[i] * y[i] * w[i];
    }

    let denominator = W * XXW - XW * XW;
    if denominator == 0f32 {
        return 0f32;
    } else {
        let slope = (W * XYW - XW * YW) / denominator;
        let intercept = (XXW * YW - XW * XYW) / denominator;

        return slope * x0 + intercept;
    }
}

pub fn lowess(data: &[f32], samples_fraction: f32) -> Vec<f32> {
    #[allow(non_snake_case)]
    let N = data.len();
    let mut ret = vec![0f32; N];
    let x: Vec<f32> = (0..N).map(|x| x as f32).collect();
    let mut w = vec![0f32; N];

    for i in 0..N {
        for j in 0..N {
            w[j] = tricube_weight(x[i] - j as f32, samples_fraction, N);
        }
        ret[i] = weighted_linear_regression(x[..].as_ref(), data, w[..].as_ref(), x[i]);
    }

    ret
}
