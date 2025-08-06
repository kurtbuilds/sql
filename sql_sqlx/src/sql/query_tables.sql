SELECT
	table_schema,
	table_name
FROM
	information_schema.tables
WHERE
	table_schema = $1
ORDER BY
table_schema
, table_name
