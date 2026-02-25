//! Orders command handlers

use anyhow::{Context, Result};
use zerodha_cli_core::{
    api::KiteConnectClient,
    config::Config,
    models::{Order, OrderType, Product, TransactionType, Validity},
};

use super::OrdersCommands;

/// Parameters for placing an order
pub(crate) struct OrderParams {
    symbol: String,
    transaction_type: String,
    order_type_enum: Option<String>,
    quantity: i32,
    price: f64,
    product: Option<String>,
    validity: Option<String>,
    dry_run: bool,
    variety: String,
}

pub async fn run_orders(
    cmd: OrdersCommands,
    config: &Config,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        super::OrdersSubcommands::List { status } => {
            run_orders_list(status, output_format, api_client).await?
        }
        super::OrdersSubcommands::Get { order_id } => {
            run_orders_get(order_id, output_format, api_client).await?
        }
        super::OrdersSubcommands::Place {
            symbol,
            order_type,
            order_type_enum,
            quantity,
            price,
            product,
            validity,
            dry_run,
            variety,
        } => {
            let params = OrderParams {
                symbol,
                transaction_type: order_type,
                order_type_enum,
                quantity,
                price,
                product,
                validity,
                dry_run,
                variety,
            };
            run_orders_place(params, config, api_client).await?
        }
        super::OrdersSubcommands::Market {
            symbol,
            order_type,
            quantity,
            product,
            dry_run,
        } => {
            run_orders_market(
                symbol, order_type, quantity, product, dry_run, config, api_client,
            )
            .await?
        }
        super::OrdersSubcommands::Modify {
            order_id,
            price,
            quantity,
            trigger_price,
            validity,
            disclosed_quantity,
        } => {
            run_orders_modify(
                order_id,
                price,
                quantity,
                trigger_price,
                validity,
                disclosed_quantity,
                api_client,
            )
            .await?
        }
        super::OrdersSubcommands::Cancel { order_id, variety } => {
            run_orders_cancel(order_id, variety, api_client).await?
        }
        super::OrdersSubcommands::CancelAll => run_orders_cancel_all(api_client).await?,
        super::OrdersSubcommands::Trades { order_id } => {
            run_orders_trades(order_id, output_format, api_client).await?
        }
    }
    Ok(())
}

pub async fn run_orders_list(
    status_filter: Option<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let orders = api_client.list_orders().await?;

    let filtered = if let Some(status) = status_filter {
        orders
            .into_iter()
            .filter(|o| format!("{:?}", o.status).to_lowercase() == status.to_lowercase())
            .collect()
    } else {
        orders
    };

    if filtered.is_empty() {
        println!("No orders found.");
        return Ok(());
    }

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&filtered)?);
    } else {
        print_orders_table(&filtered);
    }

    Ok(())
}

pub async fn run_orders_get(
    order_id: String,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let order = api_client.get_order(&order_id).await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&order)?);
    } else {
        print_order_details(&order);
    }

    Ok(())
}

pub async fn run_orders_place(
    params: OrderParams,
    config: &Config,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let symbol = params.symbol;
    let transaction_type = params.transaction_type;
    let order_type_enum = params.order_type_enum;
    let quantity = params.quantity;
    let price = params.price;
    let product = params.product;
    let validity = params.validity;
    let dry_run = params.dry_run;
    let _variety = params.variety;

    // Validate symbol
    let (exchange, tradingsymbol) = validate_symbol(&symbol)?;

    // Parse enums
    let tx_type = parse_transaction_type(&transaction_type)?;
    let order_type = parse_order_type(order_type_enum.as_deref().unwrap_or("LIMIT"))?;
    let prod = parse_product(product.as_deref().unwrap_or(&config.defaults.product))?;
    let val = parse_validity(validity.as_deref().unwrap_or("DAY"))?;

    // Validate order (clone values for validation since they get moved)
    zerodha_cli_core::validation::validate_order(
        order_type.clone(),
        quantity,
        price,
        None,
        prod.clone(),
    )
    .context("Invalid order parameters")?;

    // Build request
    let request = zerodha_cli_core::models::PlaceOrder {
        exchange,
        tradingsymbol,
        transaction_type: tx_type,
        quantity: quantity as u32,
        order_type,
        product: prod,
        price: Some(price),
        trigger_price: None,
        validity: Some(val),
        disclosed_quantity: None,
        variety: Some(_variety.to_string()),
    };

    if dry_run {
        println!("[DRY RUN] Would place order:");
        println!("  Symbol: {}", symbol);
        println!("  Type: {}", transaction_type);
        println!(
            "  Order Type: {}",
            order_type_enum.unwrap_or_else(|| "LIMIT".to_string())
        );
        println!("  Quantity: {}", quantity);
        println!("  Price: ₹{:.2}", price);
        return Ok(());
    }

    // Confirm
    print!("Confirm order? [y/N]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Order cancelled.");
        return Ok(());
    }

    let response = api_client.place_order(&request).await?;
    println!("✓ Order placed successfully!");
    println!("  Order ID: {}", response.order_id);
    println!("  Status: {:?}", response.status);

    Ok(())
}

pub async fn run_orders_market(
    symbol: String,
    transaction_type: String,
    quantity: i32,
    product: Option<String>,
    dry_run: bool,
    config: &Config,
    api_client: &KiteConnectClient,
) -> Result<()> {
    use zerodha_cli_core::models::Validity;

    // Validate symbol
    let (exchange, tradingsymbol) = validate_symbol(&symbol)?;

    // Parse enums
    let tx_type = parse_transaction_type(&transaction_type)?;
    let prod = parse_product(product.as_deref().unwrap_or(&config.defaults.product))?;

    // Build request
    let request = zerodha_cli_core::models::PlaceOrder {
        tradingsymbol,
        exchange,
        transaction_type: tx_type,
        quantity: quantity as u32,
        order_type: OrderType::Market,
        product: prod,
        price: None,
        trigger_price: None,
        validity: Some(Validity::Day),
        disclosed_quantity: None,
        variety: Some("regular".to_string()),
    };

    if dry_run {
        println!("[DRY RUN] Would place market order:");
        println!("  Symbol: {}", symbol);
        println!("  Type: {}", transaction_type);
        println!("  Quantity: {}", quantity);
        return Ok(());
    }

    // Confirm
    print!("Confirm market order? [y/N]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Order cancelled.");
        return Ok(());
    }

    let response = api_client.place_order(&request).await?;
    println!("✓ Market order placed successfully!");
    println!("  Order ID: {}", response.order_id);
    println!("  Status: {:?}", response.status);

    Ok(())
}

pub async fn run_orders_modify(
    order_id: String,
    price: Option<f64>,
    quantity: Option<i32>,
    trigger_price: Option<f64>,
    validity: Option<String>,
    _disclosed_quantity: Option<i32>,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let val = validity.map(|v| parse_validity(&v)).transpose()?;

    let request = zerodha_cli_core::models::ModifyOrder {
        quantity: quantity.map(|q| q as u32),
        price,
        trigger_price,
        validity: val,
        disclosed_quantity: _disclosed_quantity.map(|q| q as u32),
    };

    let response = api_client.modify_order(&order_id, &request).await?;
    println!("✓ Order modified successfully!");
    println!("  Order ID: {}", response.order_id);
    println!("  Status: {:?}", response.status);

    Ok(())
}

pub async fn run_orders_cancel(
    order_id: String,
    _variety: String,
    api_client: &KiteConnectClient,
) -> Result<()> {
    // Confirm
    print!("Cancel order {}? [y/N]: ", order_id);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Cancellation aborted.");
        return Ok(());
    }

    api_client.cancel_order(&order_id, &_variety).await?;
    println!("✓ Order cancelled successfully!");

    Ok(())
}

pub async fn run_orders_cancel_all(api_client: &KiteConnectClient) -> Result<()> {
    let orders = api_client.list_orders().await?;
    let open_orders: Vec<_> = orders
        .into_iter()
        .filter(|o| format!("{:?}", o.status) == "Open")
        .collect();

    if open_orders.is_empty() {
        println!("No open orders to cancel.");
        return Ok(());
    }

    println!("Found {} open orders:", open_orders.len());
    for order in &open_orders {
        println!(
            "  {} - {} {:?} @ ₹{:.2}",
            order.order_id, order.tradingsymbol, order.transaction_type, order.price
        );
    }

    print!("\nCancel all open orders? [y/N]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Cancellation aborted.");
        return Ok(());
    }

    for order in open_orders {
        let variety_str = format!("{:?}", order.variety).to_lowercase();
        match api_client.cancel_order(&order.order_id, &variety_str).await {
            Ok(_) => println!("✓ Cancelled {}", order.order_id),
            Err(e) => println!("✗ Failed to cancel {}: {}", order.order_id, e),
        }
    }

    Ok(())
}

pub async fn run_orders_trades(
    order_id: Option<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let trades = api_client.list_trades(order_id.as_deref()).await?;

    if trades.is_empty() {
        println!("No trades found.");
        return Ok(());
    }

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&trades)?);
    } else {
        print_trades_table(&trades);
    }

    Ok(())
}

fn print_orders_table(orders: &[Order]) {
    use comfy_table::{Cell, Color, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec![
        "Order ID", "Symbol", "Type", "Qty", "Price", "Status", "Time",
    ]);

    for order in orders {
        let status_cell = match &order.status {
            zerodha_cli_core::models::OrderStatus::Complete => {
                Cell::new("COMPLETE").fg(Color::Green)
            }
            zerodha_cli_core::models::OrderStatus::Open => Cell::new("OPEN").fg(Color::Yellow),
            zerodha_cli_core::models::OrderStatus::Cancelled => {
                Cell::new("CANCELLED").fg(Color::Red)
            }
            zerodha_cli_core::models::OrderStatus::Rejected => Cell::new("REJECTED").fg(Color::Red),
            _ => Cell::new(order.status.to_string()),
        };

        table.add_row(vec![
            Cell::new(&order.order_id),
            Cell::new(&order.tradingsymbol),
            Cell::new(order.transaction_type.to_string()),
            Cell::new(order.quantity.to_string()),
            Cell::new(format!("₹{:.2}", order.price)),
            status_cell,
            Cell::new(format!("{:?}", order.order_timestamp)),
        ]);
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}

fn print_order_details(order: &Order) {
    println!("Order: {}", order.order_id);
    println!();
    println!("Symbol: {} ({})", order.tradingsymbol, order.exchange);
    println!("Type: {}", order.transaction_type);
    println!("Order Type: {}", order.order_type);
    println!("Product: {}", order.product);
    println!("Variety: {}", order.variety);
    println!("Validity: {}", order.validity);
    println!("Quantity: {}", order.quantity);
    println!("Price: ₹{:.2}", order.price);

    if let Some(trigger) = &order.trigger_price {
        println!("Trigger Price: ₹{:.2}", trigger);
    }

    if let Some(avg_price) = order.average_price {
        println!("Average Price: ₹{:.2}", avg_price);
    }

    println!();
    println!("Status: {}", order.status);
    if let Some(msg) = &order.status_message {
        println!("Status Message: {}", msg);
    }
    println!("Placed At: {:?}", order.order_timestamp);
}

fn print_trades_table(trades: &[zerodha_cli_core::models::Trade]) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec![
        "Trade ID", "Order ID", "Symbol", "Type", "Quantity", "Price", "Time",
    ]);

    for trade in trades {
        table.add_row(vec![
            Cell::new(&trade.trade_id),
            Cell::new(&trade.order_id),
            Cell::new(&trade.tradingsymbol),
            Cell::new(trade.transaction_type.to_string()),
            Cell::new(trade.quantity.to_string()),
            Cell::new(format!("₹{:.2}", trade.average_price)),
            Cell::new(format!("{:?}", trade.trade_timestamp)),
        ]);
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}

fn validate_symbol(symbol: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid symbol format. Expected: EXCHANGE:SYMBOL (e.g., NSE:INFY)");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn parse_transaction_type(s: &str) -> Result<TransactionType> {
    Ok(serde_json::from_str(&format!("\"{}\"", s.to_uppercase()))?)
}

fn parse_order_type(s: &str) -> Result<OrderType> {
    let s_upper = s.to_uppercase();
    Ok(if s_upper == "MARKET" {
        OrderType::Market
    } else if s_upper == "LIMIT" {
        OrderType::Limit
    } else if s_upper == "SL" {
        OrderType::SL
    } else if s_upper == "SL-M" {
        OrderType::SLM
    } else {
        anyhow::bail!("Invalid order type. Use MARKET, LIMIT, SL, or SL-M")
    })
}

fn parse_product(s: &str) -> Result<Product> {
    let s_upper = s.to_uppercase();
    Ok(if s_upper == "CNC" {
        Product::CNC
    } else if s_upper == "MIS" {
        Product::MIS
    } else if s_upper == "NRML" {
        Product::NRML
    } else if s_upper == "BO" {
        Product::BO
    } else {
        anyhow::bail!("Invalid product. Use CNC, MIS, NRML, or BO")
    })
}

fn parse_validity(s: &str) -> Result<Validity> {
    let s_upper = s.to_uppercase();
    Ok(if s_upper == "DAY" {
        Validity::Day
    } else if s_upper == "IOC" {
        Validity::IOC
    } else {
        anyhow::bail!("Invalid validity. Use DAY or IOC")
    })
}
