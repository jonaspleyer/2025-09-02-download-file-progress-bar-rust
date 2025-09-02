use anyhow::Result;

const DB_DUMP_LINK: &str = "https://static.crates.io/db-dump.tar.gz";

// This applies the `main` macro of the smol_macro crate.
#[macro_rules_attribute::apply(smol_macros::main)]
async fn main() -> Result<()> {
    // Open a file to store the downloaded data.
    let mut file = smol::fs::File::create("/tmp/output_file").await?;

    // Create request and obtain response
    // Note: We need to use async_compat for compatibility with tokio.
    let request = async_compat::Compat::new(reqwest::get(DB_DUMP_LINK));
    let mut response = request.await?;

    // Obtain the number of total bytes in the response
    let total_bytes = response.content_length().unwrap_or_default() as usize;

    // Define the format of the progress bar
    let bar_format = "{desc}{percentage:3.0}%|{animation}| {count}/{total} [{elapsed} {rate:.2}{unit}/s{postfix}]";

    // Construct a new progress bar
    let mut pb = kdam::BarBuilder::default()
        .total(total_bytes)
        .unit_scale(true)
        .unit("Mb")
        .bar_format(bar_format)
        .build()
        .unwrap();

    // Main loop to obtain chunks iteratively
    while let Some(chunk) = response.chunk().await? {
        // Write next chunk to file
        use smol::io::AsyncWriteExt;
        file.write_all(&chunk).await?;

        // Update progress bar
        use kdam::BarExt;
        pb.update(chunk.len())?;
    }

    Ok(())
}
