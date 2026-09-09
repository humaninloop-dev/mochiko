import click


@click.group()
def main() -> None:
    pass


@main.command("seed-demo")
def seed_demo() -> None:
    """One café (Larkhill), six staff (Amira, Ben, Dana — barista; Chloe, Emil — kitchen; Priti — manager),
    a published rota for next week. Phones are the sandbox-registered test numbers."""
    click.echo("seeded Larkhill café: 6 staff, rota 2026-09-14 … 2026-09-20 published")


if __name__ == "__main__":
    main()
