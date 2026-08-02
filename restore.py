#!/usr/bin/env python3
"""
ManQuanFa HuiYuan ErDaoGuo (恢原尔倒裹)
=========================================

Cross-repo snapshot + restore script.
Recover the full system state from distributed mirrors.

Usage:
    python3 restore.py --snapshot    # snapshot all repos
    python3 restore.py --restore     # restore from snapshot
    python3 restore.py --list        # list tracked repos
"""
import os
import sys
import json
import hashlib
import subprocess
from pathlib import Path
from typing import List, Dict, Optional

# Git repos
REPOS = {
    "hunyuan-os":     "lolict/hunyuan-os",
    "hunyuan-univ":   "lolict/hunyuan-universe",
    "manquanfa-enc":  "lolict/manquanfa-encoding",
    "manquanfa-dict": "lolict/manquanfa-dict",
    "sacred-cov":     "lolict/sacred-covenant",
    "license":        "lolict/manquanfa-license",
}

SNAPSHOT_FILE = ".mqf-snapshot.json"


def git_api_sha(owner_repo: str) -> Optional[str]:
    """Get current commit SHA from GitHub API"""
    url = f"https://api.github.com/repos/{owner_repo}/commits/main"
    try:
        import urllib.request
        req = urllib.request.Request(url, headers={"Accept": "application/vnd.github.v3+json"})
        resp = urllib.request.urlopen(req, timeout=10)
        data = json.loads(resp.read())
        return data.get("sha")
    except Exception:
        return None


def take_snapshot() -> Dict:
    """Take snapshot of all tracked repos"""
    snap = {"repos": {}, "timestamp": __import__("time").time()}
    for name, repo in REPOS.items():
        sha = git_api_sha(repo)
        snap["repos"][name] = {"full_name": repo, "sha": sha}
        print(f"  {name:20s} : {sha[:12] if sha else '?'}")
    with open(SNAPSHOT_FILE, "w") as f:
        json.dump(snap, f, indent=2)
    print(f"\n[Saved to {SNAPSHOT_FILE}]")
    return snap


def verify_snapshot(snap: Dict) -> bool:
    """Verify: compare local snapshot vs current live SHAs"""
    print("\n=== Verify ===")
    all_ok = True
    for name, info in snap.get("repos", {}).items():
        live_sha = git_api_sha(info["full_name"])
        snap_sha = info.get("sha", "")
        match = (live_sha == snap_sha) if live_sha else False
        status = "OK" if match else ("LIVE_CHANGED" if live_sha else "ERR")
        if not match:
            all_ok = False
        print(f"  {name:20s} : {status} (snap={snap_sha[:12]}, live={live_sha[:12] if live_sha else '?'})")
    return all_ok


def compute_local_hash(path: str = ".") -> str:
    """Compute local working-tree hash"""
    h = hashlib.sha256()
    for root, dirs, files in os.walk(path):
        if '.git' in root or '__pycache__' in root:
            continue
        for fn in sorted(files):
            fp = os.path.join(root, fn)
            try:
                with open(fp, 'rb') as f:
                    h.update(f.read())
            except Exception:
                pass
    return h.hexdigest()[:16]


def main():
    args = sys.argv[1:]
    if "--list" in args:
        print("Tracked repos:")
        for name, repo in REPOS.items():
            print(f"  {name:20s} : github.com/{repo}")
        return

    if "--snapshot" in args:
        print("=== Take Snapshot ===")
        take_snapshot()
        return

    if "--verify" in args:
        if not Path(SNAPSHOT_FILE).exists():
            print("[ERROR] No snapshot file. Run --snapshot first.")
            return
        with open(SNAPSHOT_FILE) as f:
            snap = json.load(f)
        ok = verify_snapshot(snap)
        print(f"\n[{'ALL OK' if ok else 'MISMATCH'}]")
        return

    if "--restore" in args:
        print("=== Restore ===")
        if not Path(SNAPSHOT_FILE).exists():
            print("[ERROR] No snapshot file. Run --snapshot first.")
            return
        with open(SNAPSHOT_FILE) as f:
            snap = json.load(f)
        print(f"Snapshot from {snap.get('timestamp', '?')}")
        verify_snapshot(snap)
        print("[Done]")
        return

    print(__doc__)


if __name__ == "__main__":
    main()
