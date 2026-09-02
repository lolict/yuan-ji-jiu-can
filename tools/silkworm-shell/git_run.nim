# Git command runner - direct process execution, no shell intermediance

import os, osproc, strutils, streams

proc exec_git*(cwd: string; args: seq[string]): tuple[exitcode: int, output: string] =
  var cmd_args: seq[string] = @["-C", cwd]
  cmd_args.add(args)
  let p = startProcess("git", args = cmd_args, options = {poUsePath, poStdErrToStdOut})
  let outs = p.outputStream().readAll()
  let ec = p.waitForExit()
  p.close()
  result = (ec, outs.strip())
