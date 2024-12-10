/// Build a sequence of couples of values (start, end) from a digital signal
/// representing the boundaries of active periods
///
/// # Arguments
///
/// * `digital` - the digital signal
pub fn get_digital_intervals(digital: &[f32]) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut start = 0usize;
    let mut in_interval = false;

    for (i, value) in digital.iter().enumerate() {
        if in_interval {
            if *value == 0f32 {
                ret.push((start, i));
                in_interval = false;
            }
        } else if *value != 0f32 {
            start = i;
            in_interval = true;
        }
    }

    if in_interval {
        ret.push((start, digital.len()));
    }
    ret
}
