"""
## Decryption
`decrypt` module provides functions for keys decryptions.
- `decryptMasterKey` for decrypting GPG-encrypted master key
- `decryptSopsKey` for decrypting SOPS-encrypted file
#### TODO
rewrite without using `subprocess.run`
"""

from pathlib import Path
from subprocess import run
from os import environ


def decryptMasterKey(input: Path, output: Path) -> None:
    """
    ## GPG
    Attempts to decrypt GPG-encrypted master key.
    User have 3 attempts to enter correct password,
    or execution is terminated.

    #### Note
    Current approach is unsound, as it uses symmetric cipher to
    encrypt assymetric keys, thereby negating SOPS' reliability.\n
    In future, master keys will still be stored in repository,
    but rotated and encrypted with hardware keys (e.g. Tillitis)
    """

    attempts = 0
    in_ = str(input)
    out = str(output)
    while attempts < 3:
        try:
            run(
                [
                    "gpg",
                    "--pinentry-mode",
                    "loopback",
                    "--outputs",
                    out,
                    in_,
                ]
            )  # pyright: ignore[reportUnusedCallResult]
            print(f"Decrypted file {input} to {output}")
            return
        except KeyboardInterrupt:
            print("Interrupted, exiting...")
            exit(0)
        except Exception as e:
            print(f"An error occured: {str(e)}")
            attempts += 1

    print(f"Failed to decrypt: {in_}")
    exit(1)


def decryptSecret(input: Path, output: Path, keyFile: Path) -> None:
    """
    ## SOPS
    """

    in_ = str(input)
    out = str(output)

    env = environ.copy()
    env["SOPS_AGE_KEY_FILE"] = str(keyFile)

    try:
        run(["sops", "--output", out, "-d", in_], env=env)  # pyright: ignore[reportUnusedCallResult]
        print(f"Decrypted file {in_}")
    except Exception as e:
        print(f"Unexpected error occured: {e}")
