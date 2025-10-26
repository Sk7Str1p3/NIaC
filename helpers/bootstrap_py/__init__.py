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
    if (
        subprocess.run(["nixos-install", "--flake", f"{str(inDir)}#{host}"]).returncode
        != 0
    ):
        print(
            '[red bold]An error occured:[/] Command [blue underline]"nixos-install"[/] failed'
        )


if __name__ == "__main__":
    main()
