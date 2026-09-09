DATABASE_URL ?= postgresql://shiftbook:shiftbook@localhost:5435/shiftbook

.PHONY: migrate seed-demo

migrate:
	@for f in $(sort $(wildcard migrations/*.sql)); do \
		echo "applying $$f"; psql "$(DATABASE_URL)" -v ON_ERROR_STOP=1 -q -f $$f; \
	done

seed-demo:
	python -m shiftbook.cli seed-demo
