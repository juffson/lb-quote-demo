use std::sync::Arc;

use longport::{
    quote::{QuoteContext, SubFlags},
    Config,
};

pub async fn sub_by_symbols(s: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::from_env()?);
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ctx.subscribe(s, SubFlags::QUOTE, true).await?;
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_sub() {
        let _ = sub_by_symbols(vec!["700.HK"]).await;
        // sleep
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
