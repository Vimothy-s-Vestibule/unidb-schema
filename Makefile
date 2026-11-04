.PHONY: clean rebuild schema.sql test-db test contract

schema.sql: schema/*.sql
	@echo "Generating schema.sql from schema/*.sql..."
	@cat schema/*.sql > schema.sql
	@echo "Generated schema.sql ($$(wc -l < schema.sql | tr -d ' ') lines)"

rebuild:
	@rm -f schema.sql
	@$(MAKE) schema.sql

clean:
	rm -f schema.sql

# --- contract tests --------------------------------------------------------
# Disposable Postgres for the integration tests (never point these at a real DB:
# they CREATE/DROP databases).
TEST_DB_PORT ?= 55433
TEST_DB_IMAGE ?= pgvector/pgvector:pg17
export UNIDB_TEST_DATABASE_URL ?= postgres://postgres:postgres@127.0.0.1:$(TEST_DB_PORT)/postgres

test-db:
	docker run -d --rm --name unidb-test-db -e POSTGRES_PASSWORD=postgres \
		-p 127.0.0.1:$(TEST_DB_PORT):5432 $(TEST_DB_IMAGE)
	@until docker exec unidb-test-db pg_isready -U postgres >/dev/null 2>&1; do sleep 1; done

test:
	cargo test

# Regenerate contract/schema.json after changing schema/*.sql
contract:
	UNIDB_UPDATE_CONTRACT=1 cargo test --test contract_snapshot
