SELECT
    i.schemaname as schema,
    i.tablename as table,
    i.indexname as name,
    i.indexdef as statement,
    idx.indisunique as unique,
    am.amname as kind,
    array_agg(a.attname ORDER BY a.attnum) as columns
FROM
    pg_indexes i
JOIN
    pg_tables t ON i.schemaname = t.schemaname AND i.tablename = t.tablename
JOIN
    pg_class c ON c.relname = i.tablename
JOIN
    pg_namespace n ON n.nspname = i.schemaname AND c.relnamespace = n.oid
JOIN
    pg_index idx ON idx.indexrelid = (
        SELECT oid
        FROM pg_class
        WHERE relname = i.indexname
        AND relnamespace = n.oid
    )
JOIN
    pg_class idx_class ON idx_class.oid = idx.indexrelid
JOIN
    pg_am am ON am.oid = idx_class.relam
JOIN
    pg_attribute a ON a.attrelid = c.oid
    AND a.attnum = ANY(idx.indkey)
WHERE
    i.schemaname = $1
    AND NOT idx.indisprimary
GROUP BY
    i.schemaname,
    i.tablename,
    i.indexname,
    i.indexdef,
    idx.indisunique,
    am.amname
ORDER BY
    i.tablename,
    i.indexname
