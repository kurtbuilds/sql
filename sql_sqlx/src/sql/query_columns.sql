SELECT
	c.table_name,
	c.column_name,
	ordinal_position,
	is_nullable,
	CASE WHEN data_type = 'USER-DEFINED' THEN
		udt_name
	ELSE
		data_type
	END AS data_type,
	numeric_precision,
	numeric_scale,
	CASE WHEN data_type = 'ARRAY' THEN
		substr(udt_name, 2)
	END AS inner_type,
	CASE WHEN pk.column_name IS NOT NULL THEN
		TRUE
	ELSE
		FALSE
	END AS primary_key,
	is_generated as generation_time,
	generation_expression,
	identity_generation
FROM
	information_schema.columns c
LEFT JOIN (
	SELECT
		kcu.table_name,
		kcu.column_name
	FROM
		information_schema.table_constraints tc
	JOIN
		information_schema.key_column_usage kcu
		ON tc.constraint_name = kcu.constraint_name
		AND tc.table_schema = kcu.table_schema
	WHERE
		tc.constraint_type = 'PRIMARY KEY'
		AND tc.table_schema = $1
) pk ON c.table_name = pk.table_name AND c.column_name = pk.column_name
WHERE
	c.table_schema = $1
ORDER BY
	table_name,
	ordinal_position
