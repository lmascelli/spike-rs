use std::result::Result;

pub mod math {
    pub fn mean(range: &[f32]) -> f32 {
        let mut sum = 0.0f32;
        range.iter().for_each( |x| { sum += x; });
        sum/range.len() as f32
    }

    pub fn stdev(range: &[f32]) -> f32 {
        let mut sum = 0.0f32;
        let _mean = mean(range);
        range.iter().for_each( |x| {
            sum += (x - _mean) * (x - _mean);
        });
        (sum/(range.len() as f32 - 1.0f32)).sqrt()
    }
}

pub fn compute_threshold(range: &[f32], sampling_frequency: f32,
                         multiplier: f32) -> Result<f32, String> {
    const NUMBER_OF_WINDOWS: usize  = 30;
    const WINDOW_DURATION_TIME: f32 = 200e-3; // s
    const START_THRESHOLD: f32      = 100e-3; // V

    let window_duration_sample: usize = WINDOW_DURATION_TIME as usize * 
        sampling_frequency as usize;
    let windows_distance : usize = range.len() / NUMBER_OF_WINDOWS;

    if range.len() < (window_duration_sample * NUMBER_OF_WINDOWS) {
        return Err(format!("compute_threshold: too few samples ({}) to
        automatically compute threshold; needed at least {}",
            range.len(), window_duration_sample * NUMBER_OF_WINDOWS));
    }

    let mut threshold = START_THRESHOLD;

    for i in 0..NUMBER_OF_WINDOWS {
        let starting_point = windows_distance * i;
        let ending_point   = starting_point + window_duration_sample;
        let new_threshold = math::stdev(&range[starting_point..ending_point]);

        if new_threshold < threshold {
            threshold = new_threshold;
        }
    }

    Ok(threshold * multiplier)
}
