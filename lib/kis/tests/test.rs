#[cfg(test)]
mod integrated_test {
    use kis::kis::KIS;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_integrated_domestic() -> Result<(), Box<dyn std::error::Error>> {
        let kis = KIS::new().await?;

        let code = "005930";
        let start_date = NaiveDate::parse_from_str("2022-12-31", "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str("2024-04-06", "%Y-%m-%d")?;

        let daily = kis.domestic.inquire_daily_stock_price(code, start_date, end_date);
        let weekly = kis.domestic.inquire_weekly_stock_price(code, start_date, end_date);
        let monthly = kis.domestic.inquire_monthly_stock_price(code, start_date, end_date);

        let recent_daily = kis.domestic.inquire_recent_daily_stock_price(code);
        let recent_weekly = kis.domestic.inquire_recent_weekly_stock_price(code);
        let recent_monthly = kis.domestic.inquire_recent_monthly_stock_price(code);

        let current_price = kis.domestic.inquire_stock_price(code);

        tokio::try_join!(
            daily, weekly, monthly,
            recent_daily, recent_weekly, recent_monthly,
            current_price
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_integrated_overseas() -> Result<(), Box<dyn std::error::Error>> {
        let kis = KIS::new().await?;

        let code = "RDDT";
        let market_code = kis::MarketCode::NYS;

        let start_date = NaiveDate::parse_from_str("2023-12-31", "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str("2024-04-06", "%Y-%m-%d")?;

        let stock_price = kis.overseas.inquire_stock_price(code, market_code);
        let stock_day_price = kis.overseas.inquire_daily_stock_price(
            code,
            market_code,
            start_date,
            end_date
        );

        kis.overseas.inquire_daily_forex_value("FX@USD", start_date, end_date).await?;
        kis.overseas.inquire_daily_forex_value("FX@KRW", start_date, end_date).await?;

        tokio::try_join!(stock_price, stock_day_price)?;

        Ok(())
    }
}