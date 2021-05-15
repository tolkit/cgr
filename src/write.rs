pub mod write {
    use crate::cgr::cgr::CGR;
    use std::fs::File;
    use std::io::{BufWriter, Result, Write};

    pub fn write(file: &str, cgr: CGR) -> Result<()> {
        let temp_output_file_name = format!("./cgr_out/data/{}.tsv", file);
        let temp_output_file = File::create(&temp_output_file_name).unwrap();
        // orders of magnitude faster than LineWriter
        let mut temp_output_file = BufWriter::new(temp_output_file);

        let x = cgr.x;
        let y = cgr.y;

        for row in x.iter().zip(y) {
            writeln!(temp_output_file, "{}\t{}", row.0, row.1)
                .unwrap_or_else(|_| println!("[-]\tError in writing to file."));
        }
        Ok(())
    }
}
