import argparse


def parseCli():
    parser = argparse.ArgumentParser(description="Installation script for NixOS")

    parser.add_argument("--host", type=str, help="Target host", required=True)
    parser.add_argument(
        "--users", type=str, nargs="+", help="Users list", required=True
    )

    parser.add_argument(
        "--disko",
        action="store_true",
        help="Whether to launch disko(declarative partitioning tool) before installation",
    )

    return parser.parse_args()
