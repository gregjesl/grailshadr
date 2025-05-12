use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::task;

async fn execute(file: &str, varname: &str) {
    let base_url = "https://pds-geosciences.wustl.edu/grail/grail-l-lgrs-5-rdr-v1/grail_1001/shadr/";
    let url = format!("{}{}", base_url, file);

    // Path to save the downloaded file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join(file);

    // Variable to hold file contents
    let contents;

    // Check if the file already exists
    if file_path.exists() {
        // Read the file contents
        contents = fs::read_to_string(&file_path).expect("Failed to read file");
    } else {
        // Download the file using reqwest
        let response = reqwest::get(url).await.expect("Failed to download file");
        contents = response.text().await.expect("Failed to read response text");

        // Write the contents to a file
        fs::write(&file_path, &contents).expect("Failed to write file");
    }

    // Write the contents to a Rust source file
    let dest_path = Path::new(&out_dir).join(file.replace(".tab", ".rs"));
    let mut dest_file = fs::File::create(&dest_path).expect("Failed to create file");
    write!(dest_file, "pub const {}: &str = r#\"{}\"#;", varname, contents).expect("Failed to write to file");
}

#[tokio::main]
async fn main() {
    let mut files = vec![];

    #[cfg(feature = "gggrx_0660pm_sha")]
    files.push(("gggrx_0660pm_sha.tab", "GGGRX_0660PM_SHA"));

    #[cfg(feature = "gggrx_0900c_bouguer_sha")]
    files.push(("gggrx_0900c_bouguer_sha.tab", "GGGRX_0900C_BOUGUER_SHA"));

    #[cfg(feature = "gggrx_0900c_sha")]
    files.push(("gggrx_0900c_sha.tab", "GGGRX_0900C_SHA"));

    #[cfg(feature = "gggrx_1200a_bouguer_sha")]
    files.push(("gggrx_1200a_bouguer_sha.tab", "GGGRX_1200A_BOUGUER_SHA"));

    #[cfg(feature = "gggrx_1200a_sha")]
    files.push(("gggrx_1200a_sha.tab", "GGGRX_1200A_SHA"));

    #[cfg(feature = "gggrx_1200b_lambda1_sha")]
    files.push(("gggrx_1200b_lambda1_sha.tab", "GGGRX_1200B_LAMBDA1_SHA"));

    #[cfg(feature = "gggrx_1200b_sha")]
    files.push(("gggrx_1200b_sha.tab", "GGGRX_1200B_SHA"));

    #[cfg(feature = "gggrx_1200l_bouguer_sha")]
    files.push(("gggrx_1200l_bouguer_sha.tab", "GGGRX_1200L_BOUGUER_SHA"));

    #[cfg(feature = "gggrx_1200l_sha")]
    files.push(("gggrx_1200l_sha.tab", "GGGRX_1200L_SHA"));

    #[cfg(feature = "jggrx_0420a_sha")]
    files.push(("jggrx_0420a_sha.tab", "JGGRX_0420A_SHA"));

    #[cfg(feature = "jggrx_0660b_sha")]
    files.push(("jggrx_0660b_sha.tab", "JGGRX_0660B_SHA"));

    #[cfg(feature = "jggrx_0900c_sha")]
    files.push(("jggrx_0900c_sha.tab", "JGGRX_0900C_SHA"));

    #[cfg(feature = "jggrx_0900d_sha")]
    files.push(("jggrx_0900d_sha.tab", "JGGRX_0900D_SHA"));

    #[cfg(feature = "jggrx_1500e_sha")]
    files.push(("jggrx_1500e_sha.tab", "JGGRX_1500E_SHA"));

    #[cfg(feature = "jggrx_1800f_me_sha")]
    files.push(("jggrx_1800f_me_sha.tab", "JGGRX_1800F_ME_SHA"));

    #[cfg(feature = "jggrx_1800f_sha")]
    files.push(("jggrx_1800f_sha.tab", "JGGRX_1800F_SHA"));

    let mut handles = vec![];

    for file in files.clone() {
        let handle = task::spawn(async move {
            execute(file.0, file.1).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // Write the available features to a Rust source file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("available.rs");
    let mut dest_file = fs::File::create(&dest_path).expect("Failed to create available.rs");
    write!(dest_file, "pub const AVAILABLE_SHADRS: &[&str] = &[").expect("Failed to write to available.rs");
    let mut available_features = vec![];
    
    for (_, varname) in files {
        // write!(dest_file, "use {};", varname).expect("Failed to write to available.rs");
        available_features.push(varname);
    }
    for feature in available_features {
        write!(dest_file, "{} as &str,", feature).expect("Failed to write to available.rs");
    }
    write!(dest_file, "];").expect("Failed to write to available.rs");
}
