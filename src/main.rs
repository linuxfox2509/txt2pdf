use std::fs::File;
use std::io::{self, Read, BufWriter};
use std::path::{Path, PathBuf};
use printpdf::*;
use clap::Parser;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input .txt
    #[arg(short, long)]
    input: String, // <-- Fix: just String, not Option

    /// Output .pdf
    #[arg(short, long)]
    output: Option<String>,

    /// Font name (Helvetica, Times)
    #[arg(short, long, default_value = "Helvetica")]
    font: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Check if input is provided and not empty
    let input_file = args.input.trim();
    if input_file.is_empty() {
        eprintln!("Error: You must provide an input file with -i or --input.");
        std::process::exit(1);
    }

    // Check file extension
    if !input_file.ends_with(".txt") {
        eprintln!("Error: The input file must have a .txt extension.");
        std::process::exit(1);
    }

    // Check if file exists
    if !Path::new(input_file).exists() {
        eprintln!("Error: The input file '{}' does not exist.", input_file);
        std::process::exit(1);
    }

    // Determine output file name
    let output_file = match args.output {
        Some(ref o) => o.clone(),
        None => {
            // Use input filename, change extension to .pdf
            let input_path = Path::new(input_file);
            let stem = input_path.file_stem().unwrap_or_default();
            let parent = input_path.parent().unwrap_or_else(|| Path::new(""));
            let mut output_path = PathBuf::from(parent);
            output_path.push(format!("{}.pdf", stem.to_string_lossy()));
            output_path.to_string_lossy().to_string()
        }
    };

    // PDF layout settings
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_left = Mm(20.0);
    let margin_top = Mm(10.0);
    let margin_right = Mm(10.0);
    let margin_bottom = Mm(10.0);
    let font_size = 12.0;

    // Read content from text file
    let mut text_content = String::new();
    let mut file = File::open(input_file)?;
    file.read_to_string(&mut text_content)?;

    // Check if file is empty
if text_content.trim().is_empty() {
    eprintln!("Error: The input text file is empty.");
    std::process::exit(1);
}

// Create new PDF
let (doc, mut page, mut layer) = PdfDocument::new("Text to PDF", page_width, page_height, "Layer 1");

// Select font (Courier removed because it looks bad in the PDF and I'm too lazy to fix)
let font_choice = match args.font.to_lowercase().as_str() {
    "helvetica" => BuiltinFont::Helvetica,
    "times" | "times-roman" => BuiltinFont::TimesRoman,
    //"courier" => BuiltinFont::Courier,
    "helvetica-bold" => BuiltinFont::HelveticaBold,
    "times-bold" => BuiltinFont::TimesBold,
    //"courier-bold" => BuiltinFont::CourierBold,
    "helvetica-oblique" => BuiltinFont::HelveticaOblique,
    "times-italic" => BuiltinFont::TimesItalic,
    //"courier-oblique" => BuiltinFont::CourierOblique,
    _ => {
        eprintln!("Error: Unsupported font '{}'. Supported fonts: Helvetica, Times.", args.font);
        std::process::exit(1);
    }
};

let font = doc.add_builtin_font(font_choice).unwrap();



    // Calculate usable width and height
    let usable_width = page_width.0 - margin_left.0 - margin_right.0;
    let usable_height = page_height.0 - margin_top.0 - margin_bottom.0;

    // Estimate line height
    let line_height = font_size * 0.5;
    let max_lines_per_page = (usable_height / line_height) as usize;

    // Split text into words for wrapping
    let mut lines: Vec<String> = Vec::new();
    for paragraph in text_content.lines() {
        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            // Estimate width
            let avg_char_width = font_size * 0.17f32;
            let text_width = test_line.chars().count() as f32 * avg_char_width;
            if text_width > usable_width {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }
        lines.push(current_line);
    }

    // Write lines to PDF, handling page breaks
    let mut y = page_height.0 - margin_top.0;
    let mut line_count = 0;
    let mut current_layer = doc.get_page(page).get_layer(layer);

    for line in lines {
        if line_count >= max_lines_per_page {
            // New page
            let (new_page, new_layer) = doc.add_page(page_width, page_height, "Layer");
            page = new_page;
            layer = new_layer;
            current_layer = doc.get_page(page).get_layer(layer);
            y = page_height.0 - margin_top.0;
            line_count = 0;
        }
        current_layer.use_text(
            &line,
            font_size,
            Mm(margin_left.0),
            Mm(y - font_size),
            &font,
        );
        y -= line_height;
        line_count += 1;
    }

    // Save the PDF file
    let output = File::create(output_file)?;
    let mut buf_writer = BufWriter::new(output);
    doc.save(&mut buf_writer).unwrap();

    Ok(())
}