from pathlib import Path
import subprocess


# This approach is using child processes, so
# ==TODO== rewrite using native libraries
# ==TODO== RIIR
# This approach is also kind of dangerous.
# In future, password encryption is going to be superseded by Tillitis key
def decryptMasterKeyFile(input: Path, output: Path):
    attempts = 0
    exec = "gpg"
    while attempts < 3:
        try:
            subprocess.run(
                [
                    exec,
                    "--pinentry-mode",
                    "loopback",
                    "--output",
                    str(output),
                    str(input),
                ]
            )
            print(
                f"\nDecrypted: {str(output)}\n",
            )
            return 0
        except KeyboardInterrupt:
            exit(0)
        except Exception as e:
            print(f"An error occured: {str(e)}")
            attempts += 1

    exit(1)
