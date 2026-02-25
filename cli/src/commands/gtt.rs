//! GTT command handlers

use anyhow::Result;
use serde_json;
use zerodha_cli_core::api::KiteConnectClient;

use super::{GttCommands, GttSubcommands};

/// Parameters for creating a GTT order
pub(crate) struct GTTCreateParams {
    symbol: String,
    order_type: String,
    quantity: i32,
    price: f64,
    trigger_price: f64,
    order_type_enum: Option<String>,
    product: Option<String>,
}

pub async fn run_gtt(
    cmd: GttCommands,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        GttSubcommands::List => run_gtt_list(output_format, api_client).await,
        GttSubcommands::Get { trigger_id } => {
            run_gtt_get(trigger_id, output_format, api_client).await
        }
        GttSubcommands::Create {
            symbol,
            order_type,
            quantity,
            price,
            trigger_price,
            trigger_type: _,
            order_type_enum,
            product,
        } => {
            let params = GTTCreateParams {
                symbol,
                order_type,
                quantity,
                price,
                trigger_price,
                order_type_enum,
                product,
            };
            run_gtt_create(params, api_client).await
        }
        GttSubcommands::Modify {
            trigger_id,
            price,
            trigger_price,
        } => run_gtt_modify(trigger_id, price, trigger_price, api_client).await,
        GttSubcommands::Delete { trigger_id } => run_gtt_delete(trigger_id, api_client).await,
    }
}

pub async fn run_gtt_list(output_format: &str, api_client: &KiteConnectClient) -> Result<()> {
    let gtt_list = api_client.list_gtt().await?;

    if gtt_list.is_empty() {
        println!("No GTT orders found.");
        return Ok(());
    }

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&gtt_list)?);
    } else {
        print_gtt_table(&gtt_list);
    }

    Ok(())
}

pub async fn run_gtt_get(
    trigger_id: String,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let id: u64 = trigger_id
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid trigger ID. Must be a number"))?;

    let gtt = api_client.get_gtt(id).await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&gtt)?);
    } else {
        print_gtt_details(&gtt);
    }

    Ok(())
}

pub async fn run_gtt_create(params: GTTCreateParams, api_client: &KiteConnectClient) -> Result<()> {
    use zerodha_cli_core::models::{OrderType, Product, TransactionType};

    let symbol = params.symbol;
    let order_type = params.order_type;
    let quantity = params.quantity;
    let price = params.price;
    let trigger_price = params.trigger_price;
    let order_type_enum = params.order_type_enum;
    let product = params.product;

    // Validate symbol
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid symbol format. Expected: EXCHANGE:SYMBOL (e.g., NSE:INFY)");
    }

    let tx_type = match order_type.to_uppercase().as_str() {
        "BUY" => TransactionType::Buy,
        "SELL" => TransactionType::Sell,
        _ => anyhow::bail!("Invalid transaction type. Use BUY or SELL"),
    };

    let ord_type = match order_type_enum
        .as_deref()
        .unwrap_or("LIMIT")
        .to_uppercase()
        .as_str()
    {
        "MARKET" => OrderType::Market,
        "LIMIT" => OrderType::Limit,
        _ => anyhow::bail!("Invalid order type. Use MARKET or LIMIT"),
    };

    let prod = match product.as_deref().unwrap_or("CNC").to_uppercase().as_str() {
        "CNC" => Product::CNC,
        "MIS" => Product::MIS,
        "NRML" => Product::NRML,
        _ => anyhow::bail!("Invalid product. Use CNC, MIS, or NRML"),
    };

    let request = zerodha_cli_core::models::PlaceGTT {
        tradingsymbol: parts[1].to_string(),
        exchange: parts[0].to_string(),
        transaction_type: tx_type,
        product: prod,
        order_type: ord_type,
        quantity: quantity as u32,
        price,
        trigger_price,
        trailing_stoploss: None,
        stoploss: None,
        squareoff: None,
    };

    let response = api_client.create_gtt(&request).await?;
    println!("✓ GTT order created successfully!");
    println!("  Trigger ID: {}", response.trigger_id);
    println!("  Status: {}", response.status);

    Ok(())
}

pub async fn run_gtt_modify(
    trigger_id: String,
    price: Option<f64>,
    trigger_price: Option<f64>,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let id: u64 = trigger_id
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid trigger ID. Must be a number"))?;

    let request = zerodha_cli_core::models::ModifyGTT {
        order_type: None,
        quantity: None,
        price,
        trigger_price,
        trailing_stoploss: None,
        stoploss: None,
        squareoff: None,
    };

    let response = api_client.modify_gtt(id, &request).await?;
    println!("✓ GTT order modified successfully!");
    println!("  Trigger ID: {}", response.trigger_id);
    println!("  Status: {}", response.status);

    Ok(())
}

pub async fn run_gtt_delete(trigger_id: String, api_client: &KiteConnectClient) -> Result<()> {
    let id: u64 = trigger_id
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid trigger ID. Must be a number"))?;

    // Confirm
    print!("Delete GTT order {}? [y/N]: ", trigger_id);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Deletion cancelled.");
        return Ok(());
    }

    api_client.delete_gtt(id).await?;
    println!("✓ GTT order deleted successfully!");

    Ok(())
}

fn print_gtt_table(gtt_list: &[zerodha_cli_core::models::GTTTrigger]) {
    use comfy_table::{Cell, Color, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "Symbol",
        "Type",
        "Trigger Price",
        "Status",
        "Generated",
    ]);

    for gtt in gtt_list {
        let status_cell = match gtt.status.to_lowercase().as_str() {
            "active" => Cell::new("ACTIVE").fg(Color::Green),
            "triggered" => Cell::new("TRIGGERED").fg(Color::Yellow),
            "disabled" => Cell::new("DISABLED").fg(Color::Red),
            "expired" => Cell::new("EXPIRED").fg(Color::Red),
            _ => Cell::new(&gtt.status),
        };

        table.add_row(vec![
            Cell::new(gtt.id.to_string()),
            Cell::new(&gtt.tradingsymbol),
            Cell::new(format!("{:?}", gtt.transaction_type)),
            Cell::new(format!("₹{:.2}", gtt.trigger_price)),
            status_cell,
            Cell::new(&gtt.generated_at),
        ]);
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}

fn print_gtt_details(gtt: &zerodha_cli_core::models::GTTTrigger) {
    println!("GTT Order: {}", gtt.id);
    println!();
    println!("Symbol: {} ({})", gtt.tradingsymbol, gtt.exchange);
    println!("Status: {}", gtt.status);
    println!("Type: {:?}", gtt.transaction_type);
    println!("Order Type: {:?}", gtt.order_type);
    println!("Product: {:?}", gtt.product);
    println!("Quantity: {}", gtt.quantity);
    println!("Price: ₹{:.2}", gtt.price);
    println!("Trigger Price: ₹{:.2}", gtt.trigger_price);
    println!("Last Price: ₹{:.2}", gtt.last_price);

    println!();
    println!("Generated: {}", gtt.generated_at);
    if let Some(updated) = &gtt.updated_at {
        println!("Updated: {}", updated);
    }
    if let Some(expires) = &gtt.expires_at {
        println!("Expires: {}", expires);
    }
}
