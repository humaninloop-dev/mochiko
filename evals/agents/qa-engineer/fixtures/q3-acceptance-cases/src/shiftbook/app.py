from fastapi import FastAPI

app = FastAPI(title="Shiftbook")


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


# Rota, shift page, and manager pages are mounted from shiftbook.pages (existing).
# The swap board, swap requests, and approvals arrive with C1–C3.
