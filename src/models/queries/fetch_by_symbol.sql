SELECT *
FROM kline
WHERE symbol = ?
  AND kline.`interval` = ?
ORDER BY open_time DESC
LIMIT ? OFFSET ?