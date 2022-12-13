use netscape_to_universal::convert;
use std::fs::File;
use std::str;

fn assert_conversion(input_path: &str, correct_output: &str) {
    // Write sample file to string
    let mut input = File::open(format!("res/{}", input_path)).unwrap();
    // Convert string and output to another string
    let mut output = Vec::new();
    convert(&mut input, &mut output).unwrap();
    let output_str = str::from_utf8(output.as_slice()).unwrap();
    // Compare the two strings
    assert_eq!(output_str, correct_output);
}

#[test]
fn convert_chrome_empty() {
    assert_conversion("chrome-empty.html", "");
}

#[test]
fn convert_chrome_basic() {
    assert_conversion(
        "chrome-basic.html",
        "http://example.org/ #Bookmarks\nhttp://example.org/ #Bookmarks #New-Folder\nhttp://example.org/ #New-Folder\nhttp://example.org/\n",
    );
}

#[test]
fn convert_firefox_empty() {
    assert_conversion("firefox-empty.html", "");
}

#[test]
fn convert_firefox_basic() {
    assert_conversion("firefox-basic.html",
        "http://example.org/\nhttp://example.org/ #New-Folder\nhttp://example.org/ #Bookmarks-Toolbar\nhttp://example.org/ #Bookmarks-Toolbar #New-Folder\nhttp://example.org/ #Other-Bookmarks #New-Folder\nhttp://example.org/ #Other-Bookmarks\n");
}
