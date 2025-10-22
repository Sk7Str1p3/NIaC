#!/usr/bin/env python3

# pyright: reportUnusedCallResult = false

"""Bootstrap script for my NixOS configuration"""

from pathlib import Path
import subprocess
import shutil
from os import environ as env
from rich import print
from rich.prompt import Prompt, Confirm
from rich.console import Console

from tempfile import mkdtemp as mkTempDir
from .decrypt import decryptMasterKey, decryptSecret


def main():
    c = Console()

    inDir = Path(env.get("SELF") or str(env.get("PWD"))) / "secrets"

    c.log(f"[blue]Flake[/]: [white]{str(inDir).removesuffix('/secrets')}")
    outDir = Path(mkTempDir(prefix="secrets."))
    c.log(f"[blue]OUT[/]: [white]{str(outDir)}[/]")

    def get_host():
        while True:
            try:
                host: str = Prompt().ask("[blue bold underline]Host[/]")
                if not host:
                    c.log("[white]No hostname entered[/]")
                    continue
                c.log("[white]Checking if host configuration exists...")
                hostDir: Path = inDir / "hosts" / host
                if hostDir.exists():
                    return host
                else:
                    c.log(
                        f'[white]Folder "[underline]{str(hostDir)}[/]" [red bold]not found![/]'
                    )
                    print(
                        f'Hostname [underline red]"{host}"[/] is [red bold]invalid[/]! Try again'
                    )
            except KeyboardInterrupt:
                print()
                c.log("[white]Interrupted by user. Exiting...")
                exit(0)

    # TODO: maybe store info about host in .nix/.json file?
    def get_users():
        while True:
            try:
                userList: list[str] = (
                    Prompt().ask("[blue bold underline]Users[/]").split()
                )
                if not userList:
                    c.log("[white]No users entered")
                    continue

                invalid: list[str] = []
                c.log("[white]Checking if user configurations exists...")
                for user in userList:
                    userDir: Path = inDir / "users" / user
                    if userDir.exists():
                        continue
                    else:
                        c.log(
                            f'[white]Folder "[underline]{str(userDir)}[/]" [red bold]not found![/]'
                        )
                        invalid.append(user)
                if not invalid:
                    return userList
                else:
                    print(
                        f"Usernames [[underline red]{'[/], [underline red]'.join(invalid)}[/]] are [red bold]invalid[/]! Try again"
                    )
            except KeyboardInterrupt:
                print()
                c.log("[white]Interrupted by user. Exiting...")
                exit(0)

    host = get_host()
    users = get_users()

    print(f"Decrypting master keys...")
    decryptMasterKey(
        inPath=inDir / "hosts" / host / "masterKey.gpg",
        outPath=outDir / "host.masterKey.txt",
    )
    for user in users:
        decryptMasterKey(
            inPath=inDir / "users" / user / "masterKey.gpg",
            outPath=outDir / f"users.{user}.masterKey.txt",
        )

    try:
        if Confirm.ask("Launch disks partitioning?"):
            if Confirm.ask(
                "[yellow bold]WARNING[/]: This will completely overwrite current partition table! Continue?"
            ):
                c.log("[white]Launching disk partitioning...")
                luksKeysDir = inDir / "hosts" / f"{host}" / "luksKeys"
                if luksKeysDir.exists():
                    c.log("[white] Found disks secrets, decrypting...")
                    luksKeys = [key.name for key in luksKeysDir.iterdir()]
                    for key in luksKeys:
                        decryptSecret(
                            keyFile=outDir / "host.masterKey.txt",
                            inPath=luksKeysDir / f"{key}",
                            outPath=outDir / f"host.luksKeys.{key}.txt",
                        )
                try:
                    subprocess.run(
                        [
                            "disko",
                            "-m",
                            "destroy,format,mount",
                            "--yes-wipe-all-disks",
                            "--arg",
                            "secretsDir",
                            outDir,
                            f"{str(inDir)}/configurations/hosts/{host}/hardware/disks.nix",
                        ]
                    )
                except Exception as _:
                    print(
                        '[red bold]An error occured[/]: Command [blue underline]"Disko[/]" failed!'
                    )
                    exit(1)

    except KeyboardInterrupt:
        print()
        c.log("[white]Interrupted by user. Exiting...")
        exit(0)

    sbOutDir: Path = outDir / "secureBoot"
    sbInDir: Path = inDir / "hosts" / host / "secureBootKeys"

    if sbInDir.exists():
        c.log("[white]Found secureBoot keys, decrypting...")
        sbOutDir.mkdir()
        (sbOutDir / "keys").mkdir()
        decryptSecret(
            inPath=sbInDir / "GUID.age",
            outPath=sbOutDir / "GUID",
            keyFile=outDir / "host.masterKey.txt",
        )
        for type in ["KEK", "db", "PK"]:
            (sbOutDir / "keys" / type).mkdir()
            for ext in ["pem", "key"]:
                decryptSecret(
                    inPath=sbInDir / type / f"{ext}.age",
                    outPath=sbOutDir / "keys" / type / f"{type}.{ext}",
                    keyFile=outDir / "host.masterKey.txt",
                )
        c.log("[white]Moving SecureBoot keys...")
        try:
            shutil.rmtree(path="/mnt/var/lib/sbctl", ignore_errors=True)
            shutil.rmtree(path="/tmp/pki", ignore_errors=True)
            shutil.copytree(src=sbOutDir, dst="/mnt/var/lib/sbctl")
            shutil.copytree(src=sbOutDir, dst="/tmp/pki")
        except Exception as e:
            print(f"[bold red]An error occured:[/] {e}")
            exit(1)

        # ==TODO==: get rid of MS keys
        if subprocess.run(["sbctl", "enroll-keys", "--microsoft"]).returncode != 0:
            exit(1)

    c.log("[white]Running installation...")
    try:
        if (
            subprocess.run(
                ["nixos-install", "--flake", f"{str(inDir)}#{host}"]
            ).returncode
            != 0
        ):
            print(
                '[red bold]An error occured:[/] Command [blue underline]"nixos-install"[/] failed'
            )
    except KeyboardInterrupt:
        exit(0)

if __name__ == "__main__":
    main()
