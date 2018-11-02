# \DefaultApi

All URIs are relative to *https://api.binance.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agg_trades**](DefaultApi.md#agg_trades) | **Get** /aggTrades | Get compressed, aggregate trades. Trades that fill at the time, from the same order, with the same price will have the quantity aggregated.
[**book_ticker**](DefaultApi.md#book_ticker) | **Get** /ticker/bookTicker | Best price/qty on the order book for a symbol or symbols.
[**depth**](DefaultApi.md#depth) | **Get** /depth | Order book. Weight is Adjusted based on the limit
[**exchange_info**](DefaultApi.md#exchange_info) | **Get** /exchangeInfo | Current exchange trading rules and symbol information
[**klines**](DefaultApi.md#klines) | **Get** /klines | Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
[**ticker24hr**](DefaultApi.md#ticker24hr) | **Get** /ticker/24hr | 24 hour price change statistics. Careful when accessing this with no symbol.
[**trades**](DefaultApi.md#trades) | **Get** /trades | Get recent trades (up to last 500)


# **agg_trades**
> ::models::AggTrades agg_trades(symbol, optional)
Get compressed, aggregate trades. Trades that fill at the time, from the same order, with the same price will have the quantity aggregated.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | **String**| Market Symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 
 **limit** | **i32**| Default 500; max 1000. | 
 **from_id** | **f32**| ID to get aggregate trades from INCLUSIVE. | 
 **start_time** | **f32**| Timestamp in ms to get aggregate trades from INCLUSIVE. | 
 **end_time** | **f32**| Timestamp in ms to get aggregate trades until INCLUSIVE. | 

### Return type

[**::models::AggTrades**](aggTrades.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **book_ticker**
> Value book_ticker(optional)
Best price/qty on the order book for a symbol or symbols.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **depth**
> ::models::Depth depth(symbol, optional)
Order book. Weight is Adjusted based on the limit

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | **String**| Market Symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 
 **limit** | **i32**| Default 100; max 1000. Valid limits:[5, 10, 20, 50, 100, 500, 1000], setting limit=0 can return a lot of data | 

### Return type

[**::models::Depth**](depth.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **exchange_info**
> ::models::ExchangeInfo exchange_info()
Current exchange trading rules and symbol information

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ExchangeInfo**](exchangeInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **klines**
> ::models::Klines klines(symbol, interval, optional)
Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | **String**| Market Symbol | 
  **interval** | [**::models::CandlestickIntervals**](.md)| Default 500; max 1000. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 
 **interval** | [**::models::CandlestickIntervals**](.md)| Default 500; max 1000. | 
 **limit** | **i32**| Default 500; max 1000. | 
 **start_time** | **f32**| Timestamp in ms to get candlestick bars from INCLUSIVE. | 
 **end_time** | **f32**| Timestamp in ms to get candlestick bars until INCLUSIVE. | 

### Return type

[**::models::Klines**](klines.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ticker24hr**
> Value ticker24hr(optional)
24 hour price change statistics. Careful when accessing this with no symbol.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trades**
> ::models::Trades trades(symbol, optional)
Get recent trades (up to last 500)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | **String**| Market Symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Market Symbol | 
 **limit** | **i32**| Default 500; max 1000. | 

### Return type

[**::models::Trades**](trades.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

