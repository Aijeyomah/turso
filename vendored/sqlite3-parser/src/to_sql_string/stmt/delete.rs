#[cfg(test)]
mod tests {
    use crate::to_sql_string_test;

    // Basic DELETE from a single table
    to_sql_string_test!(test_delete_all, "DELETE FROM employees");

    // DELETE with a simple WHERE clause
    to_sql_string_test!(test_delete_with_where, "DELETE FROM employees WHERE id = 1");

    // DELETE with multiple WHERE conditions
    to_sql_string_test!(
        test_delete_with_multi_where,
        "DELETE FROM employees WHERE salary < 50000 AND department_id = 3"
    );

    // DELETE with IN clause
    to_sql_string_test!(
        test_delete_with_in,
        "DELETE FROM employees WHERE id IN (1, 2, 3)"
    );

    // DELETE with subquery in WHERE
    to_sql_string_test!(
        test_delete_with_subquery,
        "DELETE FROM employees WHERE department_id IN (SELECT id FROM departments WHERE name = 'Sales')"
    );

    // DELETE with EXISTS clause
    to_sql_string_test!(
        test_delete_with_exists,
        "DELETE FROM employees WHERE EXISTS (SELECT 1 FROM orders WHERE orders.employee_id = employees.id AND orders.status = 'pending')"
    );

    // DELETE with RETURNING clause
    to_sql_string_test!(
        test_delete_with_returning,
        "DELETE FROM employees WHERE salary < 30000 RETURNING id, name"
    );

    // DELETE with LIMIT clause
    to_sql_string_test!(
        test_delete_with_limit,
        "DELETE FROM employees WHERE salary < 40000 LIMIT 5"
    );

    // DELETE with ORDER BY and LIMIT
    to_sql_string_test!(
        test_delete_with_order_by_limit,
        "DELETE FROM employees WHERE salary < 40000 ORDER BY id DESC LIMIT 5"
    );

    // DELETE from schema-qualified table
    to_sql_string_test!(
        test_delete_schema_qualified,
        "DELETE FROM main.employees WHERE id = 1"
    );

    // DELETE with BETWEEN clause
    to_sql_string_test!(
        test_delete_with_between,
        "DELETE FROM employees WHERE salary BETWEEN 30000 AND 50000"
    );

    // DELETE with NULL check
    to_sql_string_test!(
        test_delete_with_null,
        "DELETE FROM employees WHERE department_id IS NULL"
    );

    // DELETE with LIKE clause
    to_sql_string_test!(
        test_delete_with_like,
        "DELETE FROM employees WHERE name LIKE 'J%'"
    );

    // DELETE with complex expression in WHERE
    to_sql_string_test!(
        test_delete_with_complex_expression,
        "DELETE FROM employees WHERE (salary * 1.1) > 60000 AND department_id != 1"
    );
}
