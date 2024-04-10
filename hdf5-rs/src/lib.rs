pub mod h5sys;
pub mod types;
pub mod utils;

#[cfg(test)]
mod test {
    use crate::types::{AttrOpener, DatasetOwner, File, GroupOpener};

    const FILENAME: &str = 
            "/home/leonardo/Documents/unige/raw data/raw_test.h5";

    #[test]
    fn open_file() {
        let file = crate::types::File::open(
            FILENAME,
        ).unwrap();
        println!("{file}");
    }

    #[test]
    fn open_group() {
        let file = crate::types::File::open(
            FILENAME,
        ).unwrap();
        println!("{file}");

        let group = crate::types::Group::open(file.get_fid(),
            "/Data/Recording_0/AnalogStream/Stream_0").expect("Failed to open group");
        println!("{group}");
    }

    #[test]
    fn open_attribute() -> Result<(), String> {
        let file = File::open(
            FILENAME,
        ).unwrap();
        println!("{file}");

        let group = file.open_group("Data")?;
        println!("{group}");

        let attr = group.open_attr("Date")?;
        println!("{attr}");

        let group = group.open_group("Recording_0")?;
        let attr = group.open_attr("Duration")?;
        println!("{attr}");

        Ok(())
    }
    
    #[test]
    fn open_dataset() {
        let file = crate::types::File::open(
            FILENAME,
        ).unwrap();
        println!("{file}");

        let group = crate::types::Group::open(file.get_fid(),
            "/Data/Recording_0/AnalogStream/Stream_0").expect("Failed to open group");
        println!("{group}");

        let dataset = group.get_dataset("ChannelData").unwrap();
        println!("{dataset}");

        println!("--------------------------------------------------");
    }

    #[test]
    fn open_twice() {
        open_dataset();
        open_dataset();
    }
}
