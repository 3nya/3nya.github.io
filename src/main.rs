use pulldown_cmark::{Parser, Options, Event, TextMergeStream};
use std::fs::{self, read_to_string};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::path::Path;
use serde_json::json;

pub struct Files {
    md: Vec<String>,
    html: Vec<String>,
}
impl Files {
    fn new() -> Result<Files, Box<dyn std::error::Error>> {
        Ok(Files {
            md: Vec::new(),
            html: Vec::new(),
        })
        
    }
}
fn main()->std::io::Result<()> {
    let mut site_files: Files = match Files::new() {
        Ok(t) => t,
        Err(_) => todo!()
    };


    let content_directory: fs::ReadDir = fs::read_dir("./content").unwrap();
    let output_dir = Path::new("blogs");
    fs::create_dir_all(output_dir)?;

    // parse md files in /content and make html files in /blogs
    for file in content_directory {
        // println!("Name: {}", file.unwrap().path().display())
        // let entry = file.clone()?; // handle Result<DirEntry, Error>
        let path = file?.path();

        if path.ends_with("readme.txt") {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("expected file");


        // literally forgot what this was supposed to do
        // site_files.html.push("<li><a href=\"".to_owned() + &path.file_stem().unwrap().to_str().unwrap() + ".html\">"
        //  + &path.file_stem().unwrap().to_str().unwrap() + "        <i>" + &path.to_str().unwrap().to_string() + "</i></a></li>");

        // let list_items = site_files.html.join("\n");
        
        let parser = pulldown_cmark::Parser::new(&contents);
        let mut html_output = String::new();

        // let json_data = json!(site_files.html);
        let header = "<!DOCTYPE html>
        <html>
        <head>
        <title>blog</title>
        <link rel=\"stylesheet\" href=\"../style.css\">
        </head>
        <body>

        <table id=\"container\">
        <tr>
            <td id=\"sidebar\">
            <h3>links</h3>
            <a href=\"https://github.com/3nya\"> github</a> <br>
            <a href=\"https://www.linkedin.com/in/enya-chen2/\"> linkedin </a> <br>
            <a href=\"blog.html\">blog</a><br>
            <!-- <a href=\"about.html\">about</a><br> -->
            </td>

            <td id=\"content\">
                <h1> <a href =\"index.html\"> /home/enya </a> </h1> 
            </td>
        </tr>
        </table>


        <table id=\"container\">
        <tr>
            <td id=\"content\"> \n";
        html_output.push_str(header);
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output.push_str("</td>\n");
        html_output.push_str("</tr>\n");
        html_output.push_str("</table>\n");

        let file_stem = path.file_stem().unwrap();
        let mut new_filename = PathBuf::from(file_stem);

        site_files.md.push(file_stem.to_str().unwrap().to_string());
        new_filename.set_extension("html");

        let output_path : PathBuf;
        // homepage different output dir
        if path.ends_with("index.md") {
            output_path = new_filename;
        } else {
            output_path = output_dir.join(new_filename);
        }
        let mut html_file = File::create(output_path)?;



        write!(html_file, "{}", html_output)?;



    }

    // remove deleted html files
    let site_directory: fs::ReadDir = fs::read_dir("./blogs").unwrap();

    for file in site_directory {
        let path = file?.path();
        let file_stem = path.file_stem().unwrap();

        if path.ends_with("styles.css") {
            continue;
        }

        if !site_files.md.contains(&file_stem.to_str().unwrap().to_string()) {
            
            let _ = fs::remove_file(path);

            // remove it from path


        }
    }


    Ok(())


}