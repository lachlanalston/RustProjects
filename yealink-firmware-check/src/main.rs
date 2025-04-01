use reqwest;
use scraper::{Html, Selector};

#[derive(Debug)] // This will allow you to print the struct
struct TableRow {
    model: String,
    version: String,
    info: String,
    firmware: String, // URL of the firmware download
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL to scrape
    let url = "https://www.3cx.com/docs/phone-firmwares/"; // Replace with your target URL

    // Fetch the HTML content
    let response = reqwest::get(url).await?.text().await?;

    // Parse the HTML
    let document = Html::parse_document(&response);

    // Define a CSS selector for the table with class "table-1"
    let selector = Selector::parse(".table-1").unwrap(); // Select the table with class "table-1"

    // Create an empty vector to hold the rows of the table
    let mut rows: Vec<TableRow> = Vec::new();

    // Find the table
    if let Some(table) = document.select(&selector).next() {
        // Loop through all the table rows
        for row in table.select(&Selector::parse("tr").unwrap()).skip(1) { // Skip the header row
            let mut model = String::new();
            let mut version = String::new();
            let mut info = String::new();
            let mut firmware = String::new();

            // Loop through the columns (td) of the current row
            let columns: Vec<String> = row.select(&Selector::parse("td").unwrap())
                .map(|cell| cell.text().collect::<Vec<_>>().join(" "))
                .collect();

            // If the row has exactly 4 columns (Model, Version, Info, Firmware)
            if columns.len() == 4 {
                model = columns[0].clone();
                version = columns[1].clone();
                info = columns[2].clone();

                // For the firmware column, extract the hyperlink (href attribute)
                if let Some(firmware_cell) = row.select(&Selector::parse("td a").unwrap()).next() {
                    firmware = firmware_cell.value().attr("href").unwrap_or("").to_string();
                }

                // Add the row to the vector as a TableRow
                rows.push(TableRow { model, version, info, firmware });
            }
        }
    } else {
        println!("Table not found!");
    }

    // Print the resulting vector of rows
    for row in &rows {
        println!("{:?}", row);
    }

    Ok(())
}
