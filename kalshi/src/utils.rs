use crate::TradingEnvironment;
// MACROS

#[macro_export]
#[doc(hidden)]
macro_rules! add_param {
    ($params:ident, $param_name:expr, $param_value:expr) => {
        if let Some(param) = $param_value {
            $params.push(($param_name, param.to_string()));
        }
    };
}

// Helper to build the base url

pub const fn build_base_url(trading_env: TradingEnvironment) -> &'static str {
    match trading_env {
        TradingEnvironment::LiveMarketMode => "https://trading-api.kalshi.com/trade-api/v2",
        TradingEnvironment::DemoMode => "https://demo-api.kalshi.co/trade-api/v2",
    }
}

pub const fn build_ws_url(trading_env: TradingEnvironment) -> &'static str {
    match trading_env {
        TradingEnvironment::LiveMarketMode => "wss://trading-api.kalshi.com/trade-api/ws/v2",
        TradingEnvironment::DemoMode => "wss://demo-api.kalshi.co/trade-api/ws/v2",
    }
}
