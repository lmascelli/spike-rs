mod h5content;


#[cfg(test)]
mod tests {
    use super::*;
    use h5content::H5Content;

    const FILENAME: &str = 
    "/home/leonardo/Documents/unige/raw data/raw_test.h5";

    #[test]
    fn it_works() -> Result<(), String> {
        println!("\n------------------------------------------------------------");
        let content = H5Content::open(FILENAME)?;
        println!("{content}");
        println!("------------------------------------------------------------\n");
        Ok(())
    }
}
