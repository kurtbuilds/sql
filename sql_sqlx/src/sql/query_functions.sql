SELECT
    routine_schema,
    routine_name,
    routine_type,
    data_type,
    routine_definition
FROM
    information_schema.routines
WHERE
    routine_schema = $1
ORDER BY
    routine_name