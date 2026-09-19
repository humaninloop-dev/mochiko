"""Service entry point: load the config and print the address it would bind."""
import sys

from config import load_config


def main(argv):
    path = argv[1] if len(argv) > 1 else "app.ini"
    settings = load_config(path)
    print(f"binding {settings['host']}:{settings['port']} (debug={settings['debug']})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
