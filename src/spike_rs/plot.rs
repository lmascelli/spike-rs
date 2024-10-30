use std::collections::HashMap;
use std::fs::write;
use std::path::PathBuf;

pub trait ToPyList
where
    Self: IntoIterator,
{
    fn to_py_list(&self, filename: &str);
}

impl ToPyList for &[f32] {
    fn to_py_list(&self, filename: &str) {
        let filename = &PathBuf::from(filename);
        let mut content = String::new();
        content.push_str(
            r#"
import matplotlib.pyplot as plt
data = ["#,
        );
        for el in self.iter() {
            content.push_str(&format!("{}, ", el));
        }
        content.push_str(
            r#"]
plt.plot(data)
plt.show()
"#,
        );
        write(filename, content).expect("Failed to write");
    }
}

impl ToPyList for HashMap<String, Vec<f32>> {
    fn to_py_list(&self, filename: &str) {
        let filename = &PathBuf::from(filename);
        let mut content = String::new();
        content.push_str("import matplotlib.pyplot as plt\n");
        for (v, d) in self.iter() {
            content.push_str(&format!("{} = [ ", v));
            for el in d.iter() {
                content.push_str(&format!("{}, ", el));
            }
            content.push_str(&format!(
                r#"]
plt.figure()
plt.title("{}")
plt.plot({})
"#,
                v, v
            ));
        }
        content.push_str(
            r#"
plt.show()
"#,
        );
        write(filename, content).expect("Failed to write");
    }
}
