DATABASE_URL ?= postgresql://tally:tally@localhost:5433/tally

.PHONY: migrate seed-demo lint test

migrate:
	@for f in $(sort $(wildcard migrations/*.sql)); do \
		echo "applying $$f"; psql "$(DATABASE_URL)" -v ON_ERROR_STOP=1 -q -f $$f; \
	done

seed-demo:
	python -m tally.cli seed-demo

lint:
	ruff check src tests

test:
	pytest -q
