from pathlib import Path
import subprocess
import shutil

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
        if luksKeysDir.exists():
            luksKeys = [key.name.removesuffix(".age") for key in luksKeysDir.iterdir()]
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

    sbOutDir: Path = outDir / "secureBoot"
    sbInDir: Path = inDir / "hosts" / f"{args.host}" / "secureBootKeys"
    if sbInDir.exists():
        print("Decrypting SB keys...")
        sbOutDir.mkdir()
        decryptSopsKey(
            input=sbInDir / "GUID.age",
            output=sbOutDir / "GUID",
            keyFile=outDir / "host.masterKey.txt",
        )
        for type in ["KEK", "db", "PK"]:
            (sbOutDir / f"{type}").mkdir()
            for ext in ["pem", "key"]:
                decryptSopsKey(
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
        except Exception as _:
            print("An error occured!")
            print("Most likely, you did not enter Setup Mode. Reboot to firmware and enable it")
            exit(1)

    subprocess.run(["nixos-install", "--flake", f"{str(inDir)}#{args.host}"])
