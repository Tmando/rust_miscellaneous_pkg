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
            if tag == "table" {
              docx = add_table(&docx,elem);
            }
        }
        docx.build().pack(file)?;
        Ok(())
    }
    pub fn add_table(word_document:&Box<Docx>, elem:&Value)->Box<Docx>{
      let word_document = word_document.clone();
      let mut table_rows_vec:Vec<TableRow> = Vec::new();
      if !elem.get("header").is_none(){
        let header = elem.get("header").unwrap().as_array().unwrap();
        let mut rowVec : Vec<TableCell> = Vec::new();
        for elem in header{
          rowVec.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(elem.as_str().unwrap()))));
        }
        let header_row = TableRow::new(rowVec);
        table_rows_vec.push(header_row);
      }
      if !elem.get("data").is_none(){
        let data = elem.get("data").unwrap().as_array().unwrap();
        for row in data{
          let curRow = row.as_array().unwrap();
          let mut row_vec : Vec<TableCell> = Vec::new();
          for cell in curRow{
            let mut cell_content:String = String::from("");
            if cell.is_number() && cell.is_i64(){
              cell_content = cell.as_i64().unwrap().to_string();
            } else if cell.is_number() && cell.is_f64() {
              cell_content = cell.as_f64().unwrap().to_string();
            } else if cell.is_boolean() {
              cell_content = cell.as_bool().unwrap().to_string();
            } else if cell.is_string() {
              cell_content = cell.as_str().unwrap().to_string();
            }else{
              panic!("cannot be parsed");
            }
            row_vec.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(cell_content))));
          }
          table_rows_vec.push(TableRow::new(row_vec));
        }
      }
      let mut table = Table::new(table_rows_vec);
      return Box::new(word_document.add_table(table));
    }

    pub fn add_paragraph_to_document(word_document:&Box<Docx>, elem:&Value)->Box<Docx>{
        let data = elem.get("data").unwrap().as_str().unwrap();
        let split_str = data.split("\n");
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
    #[test]
    fn table_test(){
      let json_value = r#"{
        "content": [
          {
            "tag":"table",
            "header": ["mpg","cyl","disp","hp","drat","wt","qsec","vs","am","gear","carb"],
            "width":[],
            "data":[
              [21,6,160,110,3.9,2.62,16.46,0,1,4,4],
              [21,6,160,110,3.9,2.875,17.02,0,1,4,4],
              [22.8,4,108,93,3.85,2.32,18.61,1,1,4,1],
              [21.4,6,258,110,3.08,3.215,19.44,1,0,3,1],
              [18.7,8,360,175,3.15,3.44,17.02,0,0,3,2],
              [18.1,6,225,105,2.76,3.46,20.22,1,0,3,1],
              [14.3,8,360,245,3.21,3.57,15.84,0,0,3,4],
              [24.4,4,146.7,62,3.69,3.19,20,1,0,4,2],
              [22.8,4,140.8,95,3.92,3.15,22.9,1,0,4,2],
              [19.2,6,167.6,123,3.92,3.44,18.3,1,0,4,4],
              [17.8,6,167.6,123,3.92,3.44,18.9,1,0,4,4],
              [16.4,8,275.8,180,3.07,4.07,17.4,0,0,3,3],
              [17.3,8,275.8,180,3.07,3.73,17.6,0,0,3,3],
              [15.2,8,275.8,180,3.07,3.78,18,0,0,3,3],
              [10.4,8,472,205,2.93,5.25,17.98,0,0,3,4],
              [10.4,8,460,215,3,5.424,17.82,0,0,3,4],
              [14.7,8,440,230,3.23,5.345,17.42,0,0,3,4],
              [32.4,4,78.7,66,4.08,2.2,19.47,1,1,4,1],
              [30.4,4,75.7,52,4.93,1.615,18.52,1,1,4,2],
              [33.9,4,71.1,65,4.22,1.835,19.9,1,1,4,1],
              [21.5,4,120.1,97,3.7,2.465,20.01,1,0,3,1],
              [15.5,8,318,150,2.76,3.52,16.87,0,0,3,2],
              [15.2,8,304,150,3.15,3.435,17.3,0,0,3,2],
              [13.3,8,350,245,3.73,3.84,15.41,0,0,3,4],
              [19.2,8,400,175,3.08,3.845,17.05,0,0,3,2],
              [27.3,4,79,66,4.08,1.935,18.9,1,1,4,1],
              [26,4,120.3,91,4.43,2.14,16.7,0,1,5,2],
              [30.4,4,95.1,113,3.77,1.513,16.9,1,1,5,2],
              [15.8,8,351,264,4.22,3.17,14.5,0,1,5,4],
              [19.7,6,145,175,3.62,2.77,15.5,0,1,5,6],
              [15,8,301,335,3.54,3.57,14.6,0,1,5,8],
              [21.4,4,121,109,4.11,2.78,18.6,1,1,4,2]
            ]
          }
        ]
      }"#;
      super::docx_mod::create_document(serde_json::from_str(json_value).unwrap(),"test_table_document.docx").unwrap();
    }
}