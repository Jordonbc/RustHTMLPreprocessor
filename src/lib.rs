use std::{fs, path::{PathBuf, Path}, io::Write};

#[allow(dead_code)]
fn process_files(base_dir: &str, template_dir: &str, out_dir: &str) {
    
    // Redefine all input strings to be path buffers
    let base_dir = PathBuf::from(base_dir);
    let template_dir = PathBuf::from(template_dir);
    let out_dir = PathBuf::from(out_dir);

    fs::create_dir_all(&out_dir).expect("Failed to create output directory");
    println!("Created output directory: {:?}", out_dir);

    // Recursively copy all files from base_dir to out_dir
    copy_dir_recursively(&base_dir, &out_dir).expect("Failed to copy files");
    println!("Copied files from {:?} to {:?}", base_dir, out_dir);

    for entry in fs::read_dir(&base_dir).expect("Failed to read base directory") {
        let entry = entry.expect("Failed to read base entry");
        let path = entry.path();

        if path.extension().unwrap_or_default() == "html" {
            let mut base_html = fs::read_to_string(&path).expect("Failed to read base HTML file");
            println!("Read base HTML file: {:?}", path);

            let extends_tag_start = "{% extends ";
            let extends_tag_end = " %}";

            if let Some(start) = base_html.find(extends_tag_start) {
                if let Some(end) = base_html.find(extends_tag_end) {
                    let template_file = base_html[start + extends_tag_start.len()..end].trim().to_string();
                    base_html = base_html.replace(&format!("{}{}{}", extends_tag_start, template_file, extends_tag_end), "");

                    let template_path = template_dir.join(&template_file);
                    let template_html = fs::read_to_string(&template_path).expect("Failed to read template HTML file");
                    println!("Read template HTML file: {:?}", template_path);

                    let new_html = process_html(base_html, template_html);

                    let out_path = out_dir.join(path.file_name().unwrap());
                    let mut file = fs::File::create(out_path.clone()).expect("Failed to create output file");
                    file.write_all(new_html.as_bytes()).expect("Failed to write to output file");
                    println!("Created output file: {:?}", out_path);
                }
            }
        }
    }
}

fn copy_dir_recursively<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> std::io::Result<()> {
  let from = from.as_ref();
  let to = to.as_ref();
  println!("Copying directory recursively from {:?} to {:?}", from, to);

  if !from.is_dir() {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Expected a directory"));
  }

  if let Some(parent) = to.parent() {
      fs::create_dir_all(parent)?;
      println!("Created parent directory: {:?}", parent);
  }
  if !to.exists() {
      fs::create_dir(to)?;
      println!("Created directory: {:?}", to);
  }

  for entry_result in fs::read_dir(from)? {
      let entry = entry_result?;
      let path = entry.path();
      let name = entry.file_name();
      let new_path = to.join(name);
      if path.is_dir() {
          println!("Copying directory recursively: {:?}", path);
          copy_dir_recursively(&path, new_path)?;
      } else {
          println!("Copying file: {:?} -> {:?}", path, new_path);
          fs::copy(&path, new_path)?;
      }
  }

  Ok(())
}


fn process_html(mut base_html: String, index_html: String) -> String {
    let blocks = ["header", "content"];

    for block in &blocks {
        let block_start_tag = format!("{{% block {} %}}", block);
        let block_end_tag = format!("{{% endblock {} %}}", block);

        let index_start = index_html.find(&block_start_tag).expect(&format!("Failed to find '{}' in index file", block_start_tag)) + block_start_tag.len();
        let index_end = index_html.find(&block_end_tag).expect(&format!("Failed to find '{}' in index file", block_end_tag));
        let index_block_content = &index_html[index_start..index_end].trim();

        let base_start = base_html.find(&block_start_tag).expect(&format!("Failed to find '{}' in base file", block_start_tag));
        let base_end = base_html.find(&block_end_tag).expect(&format!("Failed to find '{}' in base file", block_end_tag)) + block_end_tag.len();

        base_html.replace_range(base_start..base_end, index_block_content);
    }

    base_html
}
