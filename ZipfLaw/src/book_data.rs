pub mod book_data {
  use csv::Writer;
  use std::error::Error;

  pub struct BookData {
    pub ranks: Vec<i64>,
    pub words: Vec<String>,
    pub counts: Vec<i64>,
    pub frequencies: Vec<f64>,
  }

  impl BookData {
    pub fn save_results(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(file_path)?;

        // Write the header
        wtr.write_record(&["rank", "word", "count", "frequency"])?;

        // Write the data
        for i in 0..self.ranks.len() {
            wtr.write_record(&[
                self.ranks[i].to_string(),
                self.words[i].clone(),
                self.counts[i].to_string(),
                self.frequencies[i].to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
  }
}