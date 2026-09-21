## Complete Pipeline: Hanzi -> 64-bit Cell -> Sem -> Font -> MachineCode
import std/strutils
import cross_isa_map

type
  Cell64* = uint64
  Pol* = enum plO = 0'u64, plX = 1'u64

const SM48 = ["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w"]

proc s48(s:string):int = for i in 0..<SM48.len: if SM48[i]==s: return i; -1

proc mkCell(hz, ini, fin:string; sem:SemKind; pol:Pol):Cell64 =
  let ic = if ini.len > 0: uint8(s48(ini) mod 256) else: 0'u8
  let fc = uint8(fin.len)
  let sid = uint8(ord(sem))
  let yc = uint16(ic) * 256 + uint16(fc)
  let zc = uint16(sid) * 16 + uint16(ic)
  result = (uint64(pol) shl 63) or (uint64(sid) shl 55) or (uint64(ic) shl 47) or
           (uint64(fc) shl 39) or (uint64(yc) shl 23) or (uint64(zc) shl 7)

proc deCell(c:Cell64):tuple[sem:uint8, ini:uint8, fin:uint8, y:uint16, z:uint16] =
  result.sem = uint8((c shr 55) and 0xFF)
  result.ini = uint8((c shr 47) and 0xFF)
  result.fin = uint8((c shr 39) and 0xFF)
  result.y = uint16((c shr 23) and 0xFFFF)
  result.z = uint16((c shr 7) and 0xFFFF)

proc c2font(c:Cell64):uint32 = let d = deCell(c); (if d.sem < 16: 0xF100'u32 + d.sem else: 0xF000'u32)

type HRec = object ch, py, ini, fin: string; sem: SemKind

const HT: seq<HRec] = [
  HRec(ch:"加", py:"jia", ini:"j", fin:"ia", sem:semAdd),
  HRec(ch:"减", py:"jian", ini:"j", fin:"ian", sem:semSub),
  HRec(ch:"与", py:"yu", ini:"y", fin:"u", sem:semAnd),
  HRec(ch:"或", py:"huo", ini:"h", fin:"uo", sem:semOr),
  HRec(ch:"异", py:"yi", ini:"y", fin:"i", sem:semXor),
  HRec(ch:"读", py:"du", ini:"d", fin:"u", sem:semLoad),
  HRec(ch:"写", py:"xie", ini:"x", fin:"ie", sem:semStore),
  HRec(ch:"跳", py:"tiao", ini:"t", fin:"iao", sem:semJump),
  HRec(ch:"等", py:"deng", ini:"d", fin:"eng", sem:semBranchEq),
  HRec(ch:"比", py:"bi", ini:"b", fin:"i", sem:semSlt),
  HRec(ch:"返", py:"fan", ini:"f", fin:"an", sem:semJumpReg),
  HRec(ch:"数", py:"shu", ini:"sh", fin:"u", sem:semLui),
]

proc hz2c(hz:string):Cell64 = for e in HT: if e.ch == hz: return mkCell(e.ch, e.ini, e.fin, e.sem, plX); return 0

proc stopo(e:SemKind):string = case e
  of semAdd, semSub, semSll, semSrl, semSra, semSlt: "sphere"
  of semAnd, semOr, semXor: "double-cone"
  of semLoad, semStore: "box"
  of semBranchEq, semBranchNe, semJump, semJumpReg: "torus"
  of semLui: "tube"

when is_main_module:
  echo "=== Hanzi -> Cell64 -> Sem -> Font -> MachineCode ==="
  let encs = build_cross_isa_map()

  echo "\n[1] Hanzi -> 64-bit Cell"
  for e in HT:
    let c = hz2c(e.ch)
    echo "  ", e.ch, " ", e.py.alignLeft(6), " ", ($e.sem)[3..^1].alignLeft(8),
         " 0x", toHex(c).toUpperAscii.alignLeft(16), " pol=", (c shr 63)

  echo "\n[2] Cell -> 3D Coord (X=sem Y=initial+final Z=time)"
  for e in HT:
    let c = hz2c(e.ch)
    let d = deCell(c)
    echo "  ", e.ch, " X=", toHex(d.sem).toUpperAscii.alignLeft(3),
         " Y=0x", toHex(d.y).toUpperAscii.alignLeft(6),
         " Z=0x", toHex(d.z).toUpperAscii.alignLeft(6),
         " topo=", stopo(e.sem)

  echo "\n[3] Sem -> Font Codepoint"
  for e in HT:
    echo "  ", e.ch, " ", ($e.sem)[3..^1].alignLeft(8),
         " U+", toHex(c2font(hz2c(e.ch))).toUpperAscii

  echo "\n[4] 'ADD' (加) in 4 ISAs"
  for isa in IsaKind:
    for e in encs:
      if e.sem == semAdd and e.isa == isa:
        echo "  ", ($isa).alignLeft(10), " ", e.machine_hex, "  ", e.asm_text
        break

  echo "\n[5] Full trace for '加' (add)"
  let cc = hz2c("加")
  let dd = deCell(cc)
  echo "  Input: 加 (jia)"
  echo "  SM48 idx(j) = ", s48("j")
  echo "  Cell: 0x", toHex(cc).toUpperAscii
  echo "    bit63 polarity = 1 (owner)"
  echo "    sem_id = ", dd.sem, " (semAdd)"
  echo "    initial = ", dd.ini, " final = ", dd.fin
  echo "  3D: X = ", dd.sem, " Y = 0x", toHex(dd.y).toUpperAscii, " Z = 0x", toHex(dd.z).toUpperAscii
  echo "  Font: U+", toHex(c2font(cc)).toUpperAscii
  echo "  Machine codes:"
  for isa in IsaKind:
    for e in encs:
      if e.sem == semAdd and e.isa == isa:
        echo "    ", ($isa).alignLeft(8), " -> ", e.machine_hex
        break

  echo "\n=== DONE ==="
