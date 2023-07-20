// migrations, vec of (id, stmt) tuples
// on app load, db.select latest migration
// get migrations_to_do slice starting at latest_migration + 1
// for each stmt, execute
// update migrations table with latest migration
