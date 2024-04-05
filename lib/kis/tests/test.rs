#[cfg(test)]
mod integrated_test {
    use kis::kis::KIS;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_integrated_domestic() -> Result<(), Box<dyn std::error::Error>> {
        let kis = KIS::new()?;

        let code = "005930";
        let crit_date = NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d")?;

        let daily = kis.domestic.inquire_daily_stock_price(code, crit_date);
        let weekly = kis.domestic.inquire_weekly_stock_price(code, crit_date);
        let monthly = kis.domestic.inquire_monthly_stock_price(code, crit_date);

        let recent_daily = kis.domestic.inquire_recent_daily_stock_price(code);
        let recent_weekly = kis.domestic.inquire_recent_weekly_stock_price(code);
        let recent_monthly = kis.domestic.inquire_recent_monthly_stock_price(code);

        let current_price = kis.domestic.inquire_stock_price(code);

        let _ = tokio::try_join!(
            daily, weekly, monthly,
            recent_daily, recent_weekly, recent_monthly,
            current_price
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_integrated_overseas() -> Result<(), Box<dyn std::error::Error>> {
        let kis = KIS::new()?;

        let code = "APPL";
        let market_code = kis::MarketCode::NAS;

        let stock_price = kis.overseas.inquire_stock_price(code, market_code);

        let result = tokio::try_join!(stock_price)?;

        Ok(())
    }
}