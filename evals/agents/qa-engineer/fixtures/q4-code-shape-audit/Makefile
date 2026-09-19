DATABASE_URL ?= postgresql://postbox:postbox@localhost:5436/postbox

.PHONY: migrate

migrate:
	@for f in $(sort $(wildcard migrations/*.sql)); do \
		echo "applying $$f"; psql "$(DATABASE_URL)" -v ON_ERROR_STOP=1 -q -f $$f; \
	done
