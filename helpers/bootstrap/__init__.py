from pathlib import Path
import subprocess
import shutil

from tempfile import mkdtemp as mkTempDir
from .decrypt import decryptMasterKey, decryptSecret
from .cli import parseCli
from os import environ as env
from rich.console import Console

def log(*args): # pyright: ignore[reportMissingParameterType, reportUnknownParameterType]
    Console().log(*args)

def main():
    args = parseCli()

    inDir = Path(env.get("SELF") or ".") / "secrets"
    log(f"[bold italic blue]$flake[/bold italic blue]: {str(inDir).removesuffix('secrets')}")
    outDir = Path(mkTempDir(prefix="secrets."))
    log(f"[bold italic blue]$OUT[/bold italic blue]: {str(outDir)}")

    log("Decrypting master keys...")
    decryptMasterKey(
        input=inDir / "hosts" / f"{args.host}" / "masterKey.gpg",
        output=outDir / "host.masterKey.txt",
    )
    for user in args.users:
        decryptMasterKey(
            input=inDir / "users" / f"{user}" / "masterKey.gpg",
            output=outDir / f"users.{user}.masterKey.txt",
        )

    if args.disko:
        log("Launching disk partitioning...")
        luksKeysDir = inDir / "hosts" / f"{args.host}" / "luksKeys"
        if luksKeysDir.exists():
            luksKeys = [key.name.removesuffix(".age") for key in luksKeysDir.iterdir()]
            log("Decrypting LUKS password files")
            for key in luksKeys:
                decryptSecret(
                    keyFile=outDir / "host.masterKey.txt",
                    input=luksKeysDir / f"{key}",
                    output=outDir / f"host.luksKeys.{key}.txt",
                )
        log(
            "Warning! This will completely overwrite current partition table! Continue? [YES/NO]"
        )
        while True:
            proceed: str = input("[YES/NO]").strip()
            match proceed:
                case "YES":
                    subprocess.run(
                        [
                            "disko",
                            "-m",
                            "destroy,format,mount",
                            "--yes-wipe-all-disks",
                            "--arg",
                            "secretsDir",
                            f"{outDir}",
                            f"configurations/hosts/{args.host}/hardware/disks.nix",
                        ] 
                    ) # pyright: ignore[reportUnusedCallResult]
                    break
                case "NO":
                    log("Cancelled")
                    exit(0)
                case unknown:
                    log(f"Unknown value: {unknown}")

    sbOutDir: Path = outDir / "secureBoot"
    sbInDir: Path = inDir / "hosts" / f"{args.host}" / "secureBootKeys"
    if sbInDir.exists():
        log("Decrypting SB keys...")
        sbOutDir.mkdir()
        decryptSecret(
            input=sbInDir / "GUID.age",
            output=sbOutDir / "GUID",
            keyFile=outDir / "host.masterKey.txt",
        )
        for type in ["KEK", "db", "PK"]:
            (sbOutDir / f"{type}").mkdir()
            for ext in ["pem", "key"]:
                decryptSecret(
                    input=sbInDir / f"{type}" / f"{ext}.age",
                    output=sbOutDir / f"{type}" / f"{type}.{ext}",
                    keyFile=outDir / "host.masterKey.txt",
                )
        Path("/mnt/var/lib/sbctl").mkdir(parents=True)
        shutil.move(src=sbOutDir, dst="/mnt/var/lib/sbctl")
        # Move also to current installation because sbctl does not provide option
        # for overriding keys location
        shutil.move(src=sbOutDir, dst="/var/lib/sbctl")

        try:
            # ==TODO==: get rid of MS keys
            subprocess.run(["sbctl", "enroll-keys", "--microsoft"])
        except Exception as e:
            log(f"An error occured: {e}")
            log(
                "Most likely, you did not enter Setup Mode. Reboot to firmware and enable it"
            )
            exit(1)

    subprocess.run(["nixos-install", "--flake", f"{str(inDir)}#{args.host}"])
