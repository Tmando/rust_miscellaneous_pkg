pub mod docx_mod {
    use std::ffi::OsStr;

    use docx_rs::*;
    use docx_rs::Docx;
    use serde_json::Value;

    pub fn create_document(json_content:Value,outputfile_path: impl AsRef<OsStr>) -> Result<(), DocxError> {
        let path = std::path::Path::new(&outputfile_path);
        let file = std::fs::File::create(&path).unwrap();
        let mut docx = Box::new(Docx::new());
        let content = json_content.get("content").unwrap().as_array().unwrap();
        for elem in content{
            let tag = elem.get("tag").unwrap().as_str().unwrap();
            if tag == "p" {
                docx = add_paragraph_to_document(&docx,elem); 
            }
        }
        docx.build().pack(file)?;
        Ok(())
    }

    pub fn add_paragraph_to_document(word_document:&Box<Docx>, elem:&Value)->Box<Docx>{
        let data = elem.get("data").unwrap().as_str().unwrap();
        println!("{:?}",data);
        let mut split_str = data.split("\n");
        let word_document = word_document.clone();
        let mut paragraph = Paragraph::new();
        for s in split_str{
            paragraph = paragraph.clone().add_run(Run::new().add_text(s));
        }
        return Box::new(word_document.add_paragraph(paragraph));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_document(){
        let json_value = r#"{
            "content": [
              {
                "tag":"p",
                "data":"Hallo Welt"
              },
              {
                "tag":"p",
                "data":"Hallo Welt 1"
              },
              {
                "tag":"p",
                "data":"Hallo Welt 2"
              },
              {
                "tag":"p",
                "data":"Hallo Welt 3"
              },
              {
                "tag":"p",
                "data":"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren,"
              }
            ]
          }"#;
          super::docx_mod::create_document(serde_json::from_str(json_value).unwrap(),"test_document.docx").unwrap();
    }
}