SELECT
    i.schemaname,
    i.tablename,
    i.indexname,
    i.indexdef
FROM
    pg_indexes i
JOIN
    pg_tables t ON i.schemaname = t.schemaname AND i.tablename = t.tablename
WHERE
    i.schemaname = $1
ORDER BY
    i.tablename,
    i.indexname
