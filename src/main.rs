use std::process::Command;

use anyhow::{bail, Result};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, disable_help_flag = true)]
struct Cli {
    #[arg(long)]
    input_token: String,

    #[arg(long)]
    output_token: String,

    #[arg(long)]
    price: f64,

    #[arg(long)]
    amount: f64,

    #[arg(long, default_value_t = false)]
    clear_all_orders: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra_args: Vec<String>,
}

fn bracketize_csv(csv: &str, hex_elems: bool) -> Result<String> {
    let mut out = String::from("[");
    for (i, raw) in csv.split(',').map(|s| s.trim()).enumerate() {
        if raw.is_empty() {
            continue;
        }
        if i > 0 {
            out.push(',');
        }
        if hex_elems && !(raw.starts_with("0x") || raw.starts_with("0X")) {
            bail!("expected hex starting with 0x, got: {raw}");
        }
        out.push_str(raw);
    }
    out.push(']');
    Ok(out)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let controller = "0x08feDaACe14EB141E51282441b05182519D853D1";
    let signature = "execute(uint8[],bytes[],address[],(address,uint256,(uint256,uint8,bytes32,bytes32))[],(uint256,(uint256,uint8,bytes32,bytes32))[],uint64)";

    let actions_str = bracketize_csv("1,1", false)?;
    let params_str = bracketize_csv(
        "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000dfb568f507882bf0fcc54851d1878d10cc545dd301dc25a0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcf1880000000000000000000000000000000000000000000000000000000000b71b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000,0x00000000000000000000000000000000000000000000000000000000000000200000000000000000dfb568f507882bf0fcc54851d1878d10cc545dd301dc25a0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcf1880000000000000000000000000000000000000000000000000000000000b71b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000",
        true,
    )?;
    let token_to_settles_str =
        bracketize_csv(&format!("{},{}", cli.input_token, cli.output_token), true)?;
    let clear_all_orders_str =
        if cli.clear_all_orders { "[1]".to_string() } else { "[]".to_string() };
    let deadline = u64::MAX.to_string();

    let mut args: Vec<String> = vec![
        "send".into(),
        controller.into(),
        signature.into(),
        actions_str,
        params_str,
        token_to_settles_str,
        clear_all_orders_str,
        "[]".into(),
        deadline,
    ];
    args.extend(cli.extra_args);

    let out = Command::new("cast").args(&args).output()?;
    if !out.status.success() {
        bail!("cast failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    print!("{}", String::from_utf8_lossy(&out.stdout));
    Ok(())
}
