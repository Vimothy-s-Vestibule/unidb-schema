.PHONY: clean rebuild schema

schema: schema/*.sql
	@echo "Generating schema.sql from schema/*.sql..."
	@cat schema/*.sql > schema.sql
	@echo "Generated schema.sql ($$(wc -l < schema.sql | tr -d ' ') lines)"

rebuild:
	@rm -f schema.sql
	@$(MAKE) schema.sql

clean:
	rm -f schema.sql
