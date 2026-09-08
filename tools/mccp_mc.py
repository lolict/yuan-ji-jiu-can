#!/usr/bin/env python3
"""
MCCP Machine Code CLI tool
Encode semantic ID + operands → machine code hex
Decode machine code hex → semantic ID + operands

Usage:
    python3 mccp_mc.py encode <semantic_id> <isa> [operands...]
    python3 mccp_mc.py decode <hex_string> <isa>
    python3 mccp_mc.py lookup <mnemonic> [isa]
    python3 mccp_mc.py list [semantic_id]
"""

import json
import sys
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
JSON_PATH = os.path.join(os.path.dirname(SCRIPT_DIR), "docs", "machine_code_db.json")

def load_db():
    with open(JSON_PATH, "r") as f:
        return json.load(f)

def encode Instruction
def decode: ISA-only lookup
def lookup: mnemonic-based search
def list_all / list by sem_id

if __name__ == "__main__":
    main()
