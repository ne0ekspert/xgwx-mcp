use rmcp::ServiceExt;
use xgwx_mcp::{server::XgwxServer, storage::FileStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let startup = std::env::current_dir()?;
    let mut roots = vec![startup.clone()];
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => roots.push(args.next().ok_or("--root requires a directory")?.into()),
            "--help" | "-h" => {
                eprintln!(
                    "xgwx-mcp [--root DIRECTORY]...\nLocal stdio MCP server. Startup directory is always an allowed root."
                );
                return Ok(());
            }
            "--version" | "-V" => {
                eprintln!("xgwx-mcp {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }
    let server = XgwxServer::new(FileStore::new(startup, roots)?);
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
