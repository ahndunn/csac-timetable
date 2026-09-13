#!/usr/bin/env python3
# /// script
# dependencies = [
#     "argon2-cffi>=23.1.0",
# ]
# ///
"""
CSAC Timetable Studio - Argon2id Password Cryptography Utility
Compliant with Rust csac-common password hashing parameters.
"""

import sys
from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError

# Standard Argon2id configuration
ph = PasswordHasher(
    time_cost=3,
    memory_cost=65536,
    parallelism=4,
    hash_len=32,
    salt_len=16
)

def hash_password(password: str) -> str:
    """Hash a cleartext password with Argon2id."""
    return ph.hash(password)

def verify_password(password: str, hashed: str) -> bool:
    """Verify a cleartext password against an Argon2id hash."""
    try:
        return ph.verify(hashed, password)
    except (VerifyMismatchError, Exception):
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: uv run scripts/hash_password.py <password> [hash_to_verify]")
        print("Example: uv run scripts/hash_password.py seed")
        sys.exit(1)

    password = sys.argv[1]
    if len(sys.argv) >= 3:
        hash_to_verify = sys.argv[2]
        is_valid = verify_password(password, hash_to_verify)
        print(f"Password '{password}' verification: {'VALID ✅' if is_valid else 'INVALID ❌'}")
    else:
        hashed = hash_password(password)
        print(f"Password: {password}")
        print(f"Argon2id Hash: {hashed}")

if __name__ == "__main__":
    main()
