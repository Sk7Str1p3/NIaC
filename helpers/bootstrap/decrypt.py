"""
## Secrets
Main purpose of this script in general is decrypting secrets.
This module provides functions for this actions
"""

# pyright: strict
# pyright: reportUnusedCallResult = false
# pyright: reportUnknownMemberType = false
# pyright: reportMissingTypeStubs = false

from pathlib import Path
from os import environ as env
import rich
import gpg, sopsy
import tty, sys, termios


def _getPass() -> str:
    def getch() -> str:
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)
        try:
            tty.setraw(fd)
            ch = sys.stdin.read(1)
        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
        return ch

    passwd: list[str] = []
    sys.stdout.flush()
    while True:
        key = ord(getch())
        if key == 13:  # return
            sys.stdout.write("\n")
            return "".join(passwd)
        elif key in (8, 127):  # Backspace/Del
            if len(passwd) > 0:
                sys.stdout.write("\b \b")
                sys.stdout.flush()
                passwd = passwd[:-1]
        elif key == 3:  # Ctrl + C
            sys.stdout.write("^C")
            raise KeyboardInterrupt
        elif 0 <= key <= 31:  # Unprintable (Esc, F1-F12, etc)
            pass
        else:
            char = chr(key)
            sys.stdout.write("*")
            sys.stdout.flush()
            passwd.append(char)


def decryptMasterKey(inPath: Path, outPath: Path) -> None:
    """
    ### Master Key
    Decrypts GPG-encrypted SOPS' master key using `gpgme`.

    ***Note***: For now, script uses symmetric sipher for encrypting
    assymetric key which is unsound and nullifies SOPS' reliability.
    In future, passpharase is going to be replaced with some kind of FIDO key
    (Tillitis I hope)
    """

    c = rich.console.Console()
    attempts = 0

    def cb(
        _uidHint: str | None,
        _passphraseInfo: str | None,
        prevWasBad: bool,
        _fd: int | None = None,
    ):
        if prevWasBad:
            rich.print("[bold red]Wrong password! Try again")
        try:
            c.print(
                f"[blue underline]Password[/] ([bold red]Attempt[/bold red]: {attempts + 1}/3): ",
                end="",
            )
            password = _getPass()
            return password
        except KeyboardInterrupt:
            print()
            c.log("Interrupted by user. Exiting...")
            exit(0)

    ctx = gpg.Context()
    ctx.set_passphrase_cb(cb)
    ctx.pinentry_mode = (
        gpg.constants.PINENTRY_MODE_LOOPBACK  # pyright: ignore[reportAttributeAccessIssue]
    )

    input = str(inPath)
    output = str(outPath)

    c.log(f"[white]Decrypting: {input}...")
    while attempts < 3:
        try:
            with open(input, "rb") as source, open(output, "wb") as target:
                ctx.decrypt(source, target)
                c.log(
                    f"[bold green]Successfully decrypted[/bold green]: {input} -> {output}"
                )
            return
        except Exception as e:
            rich.print(f"[bold red]Error:[/] {e}")
            attempts += 1

    exit(1)


def decryptSecret(inPath: Path, outPath: Path, keyFile: Path):
    """
    ### SOPS secret
    Decrypts SOPS-encrypted secret.
    """

    c = rich.console.Console()

    input = str(inPath)
    output = str(outPath)
    key = str(keyFile)

    c.log(f"[white]Decrypting: {input}")
    env["SOPS_AGE_KEY_FILE"] = key
    try:
        decrypted = sopsy.Sops(input).decrypt(to_dict=False)
        with open(outPath, "wb") as f:
            if isinstance(decrypted, bytes):
                f.write(decrypted)
            elif isinstance(decrypted, str):
                f.write(decrypted.encode())
    except Exception as e:
        print(e)
        exit(128)

    c.log(f"[bold green]Successfully decrypted[/bold green]: {input} -> {output}")
