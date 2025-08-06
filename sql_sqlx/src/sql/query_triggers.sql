SELECT
    trigger_schema,
    trigger_name,
    event_manipulation,
    event_object_table,
    action_timing,
    action_statement
FROM
    information_schema.triggers
WHERE
    trigger_schema = $1
ORDER BY
    event_object_table,
    trigger_name