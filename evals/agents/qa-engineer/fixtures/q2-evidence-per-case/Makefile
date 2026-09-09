DATABASE_URL ?= postgresql://quill:quill@localhost:5434/quill

.PHONY: migrate seed-demo

migrate:
	@for f in $(sort $(wildcard migrations/*.sql)); do \
		echo "applying $$f"; psql "$(DATABASE_URL)" -v ON_ERROR_STOP=1 -q -f $$f; \
	done

seed-demo:
	python -m quill.cli seed --notebook demo --from fixtures/demo-notebook
