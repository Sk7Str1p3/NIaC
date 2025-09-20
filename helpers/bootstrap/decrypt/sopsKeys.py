from pathlib import Path
import subprocess
import os


def decryptSopsKey(input: Path, output: Path, keyFile: Path):
    exec = "sops"

    env = os.environ.copy()
    env["SOPS_AGE_KEY_FILE"] = str(keyFile)
    subprocess.run([exec, "--output", str(output), "-d", str(input)], env=env)
