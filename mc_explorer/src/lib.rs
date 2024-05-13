pub mod old;
mod h5content;

pub use h5content::H5Content;

#[cfg(test)]
mod tests {
    use super::*;
    use h5content::H5Content;
}
