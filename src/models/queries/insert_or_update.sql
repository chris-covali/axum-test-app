INSERT INTO kline(symbol, `interval`, open_time, open, high, low, close, volume, close_time, quote_asset_volume,
                  number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
on DUPLICATE KEY UPDATE open                         = VALUES(open),
                        high                         = VALUES(high),
                        low                          = VALUES(low),
                        close                        = VALUES(close),
                        volume                       = VALUES(volume),
                        close_time                   = VALUES(close_time),
                        quote_asset_volume           = VALUES(quote_asset_volume),
                        number_of_trades             = VALUES(number_of_trades),
                        taker_buy_base_asset_volume  = VALUES(taker_buy_base_asset_volume),
                        taker_buy_quote_asset_volume = VALUES(taker_buy_quote_asset_volume);
