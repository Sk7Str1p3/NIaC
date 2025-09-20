from pathlib import Path
import subprocess

from tempfile import mkdtemp as mkTempDir
from .decrypt.masterKeys import decryptMasterKeyFile
from .decrypt.sopsKeys import decryptSopsKey
from .cli import parseCli
from os import environ as env


def main():
    args = parseCli()

    inDir = Path(env.get("SELF") or ".") / "secrets"
    print(f"flake: {str(inDir).removesuffix("secrets")}")
    outDir = Path(mkTempDir(prefix="secrets."))
    print(f"Setting $OUT to {str(outDir)}")

    print("Decrypting master keys...")
    decryptMasterKeyFile(
        input=inDir / "hosts" / f"{args.host}" / "masterKey.gpg",
        output=outDir / "host.masterKey.txt",
    )
    for user in args.users:
        decryptMasterKeyFile(
            input=inDir / "users" / f"{user}" / "masterKey.gpg",
            output=outDir / f"users.{user}.masterKey.txt",
        )

    if args.disko:
        print("Launching disk partitioning...")
        luksKeysDir = inDir / "hosts" / f"{args.host}" / "luksKeys"
        luksKeys: list[str] = (
            [
                str(item).removeprefix(str(luksKeysDir) + "/").removesuffix(".age")
                for item in luksKeysDir.iterdir()
                if item.is_file()
            ]
            if luksKeysDir.exists() and luksKeysDir.is_dir()
            else []
        )
        if luksKeys != []:
            print("Decrypting LUKS password files")
            for key in luksKeys:
                decryptSopsKey(
                    keyFile=outDir / "host.masterKey.txt",
                    input=luksKeysDir / f"{key}",
                    output=outDir / f"host.luksKeys.{key}.txt",
                )
        print("Warning! This will completely overwrite current partition table! Continue? [YES/NO]")
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
                            "secretsDir"
                            f"{outDir}"
                            f"configurations/hosts/{args.host}/hardware/disks.nix",
                        ]
                    )
                    break
                case "NO":
                    print("Cancelled")
                    exit(0)
                case unknown:
                    print(f"Unknown value: {unknown}")

    subprocess.run(["nixos-install", "--flake", f"{str(inDir)}#{args.host}"])
