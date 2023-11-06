# kalshi-rust

## Rust Wrapper for the Kalshi trading API

This is a wrapper for the [Kalshi](https://kalshi.com/) trading API written by and for those using Rust. 
This wrapper is asynchronous and typically more performant than the official Python API provided by the developers, presented here: [*KalshiDevAPI*](https://github.com/Kalshi/kalshi-python)

## Featurelist + Roadmap

### HTTP Requests: ✅ 
As of now the project supports interacting with Kalshi's RESTful API **fully**.
The only features missing from that part of the project are as follows:
- Advanced API features (Waiting for the devs to give me access)

However any user of this library can build a rust trading bot using this library 
if they wish!

Every function present in the library wraps around the [Kalshi Trading API](https://trading-api.readme.io/reference/getting-started).

#### HTTP Feature Table

| Feature                | Description                           | Status      |
|------------------------|---------------------------------------|-------------|
| **Auth/Login**          | Retreiving your user token       |  ✅         |
| **Auth/Logout**         | Deleting your user token        |    ✅     |
| **Exchange/GetSchedule**          | Retrieve Exchange Schedule     |   ✅    |
| **Exchange/GetExchangeStatus**          | Retreive Exchange Status   |   ✅        |
| **Portfolio/GetBalance** | Get User Balance |     ✅  |
| **Portfolio/GetFills** | Get User's Fills that fit certain criteria|  ✅        |
| **Portfolio/GetOrders** | Get User's orders that fit certain criteria |  ✅       |
| **Portfolio/CreateOrder** | Submit an Order |✅         |
| **Portfolio/BatchCreateOrders** | Submit multiple Orders |❌          |
| **Portfolio/BatchCancelOrders** | Cancel Multiple Orders (Advanced Users Only) |❌          |
| **Portfolio/GetOrder** | Get a single Order | ✅          |
| **Portfolio/CancelOrder** | Cancel an order |✅          |
| **Portfolio/DecreaseOrder** | Decrease Order amount |✅          |
| **Portfolio/GetPositions** | Get Positions (Get all the positions of logged in user) |✅           |
| **Portfolio/GetPortfolioSettlements** | Get Portfolio Settlements (Get settlement history) |✅         |
| **Market/GetEvents** | Get data about all events |✅         |
| **Market/GetEvent** | Get data about a single event |✅         |
| **Market/GetMarkets** | Get data about all markets |✅       |
| **Market/GetTrades** | Get data about trades fitting certain criteria |✅           |
| **Market/GetMarket** | Get data about a single market |✅          |
| **Market/GetMarketHistory** | Get data about a single market's historical data |✅           |
| **Market/GetMarketOrderBook** | Get a market's order book |✅         |
| **Market/GetSeries** | Get data about a series |✅         |

### More verbose errors + QOL: ✅
I completed revamping the errors provided by the API to provide more support
for the user to debug their work. I also completed revamping the datatypes of the project
to take advantage of Rust's enums to ensure that the user minimizes error-prone requests to the server. 


### Writing detailed docs:  🟡 
I'm writing detailed docs + a user manual  to support a user's implementation of a Kalshi bot.

### Websocket wrapper: ❌    
As of now the kalshi_api doesn't host a lot of functionality through a websocket
connection, in fact it doesn't even support submitting / altering orders through it.
However a goal of mine for this project is to successfully write a websocket
implementation for anyone using the API. 







