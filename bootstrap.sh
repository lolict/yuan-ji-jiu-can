#!/bin/sh
# ============================================================
# 满全法 · 自由寄生壳 · 最小自举器 (bootstrap.sh)
# ============================================================
set -e
MQF_VERSION="0.1.0"
SHRINE_DIR=""
LOG_FD=2

if [ -t 2 ]; then
    C_CYAN='\033[36m'; C_YLW='\033[33m'; C_GRN='\033[32m'
    C_RED='\033[31m'; C_RST='\033[0m'; C_BLD='\033[1m'
else
    C_CYAN=""; C_YLW=""; C_GRN=""; C_RED=""; C_RST=""; C_BLD=""
fi

log_info() { printf "${C_CYAN}[--]${C_RST} %s\n" "$*" >&${LOG_FD}; }
log_ok()   { printf "${C_GRN}[ok]${C_RST} %s\n" "$*" >&${LOG_FD}; }
log_warn() { printf "${C_YLW}[!]${C_RST}  %s\n" "$*" >&${LOG_FD}; }
log_err()  { printf "${C_RED}[FAIL]${C_RST} %s\n" "$*" >&${LOG_FD}; }

banner() {
    printf "${C_CYAN}${C_BLD}"
    cat <<'BANNER_EOF'

      满全法  ·  自由寄生壳  ·  最小自举器
      ManQuanFa  ·  Bootstrap

      chrysalis  ->  molt  ->  wear  ->  work

BANNER_EOF
    printf "${C_RST}"
    log_info "version ${MQF_VERSION}"
}

probe_env() {
    OS="unknown"; SHELL_CAP="stub"
    case "$(uname -s)" in
        Linux*)  OS="linux"   ;;
        Darwin*) OS="macos"   ;;
    esac
    if [ -d "/data/harmony" ] || [ -n "${HARMONY_SDK:-}" ]; then OS="harmony"; fi
    if [ -d "/system/bin" ] && [ -z "${SHELL:-}" ]; then OS="android"; fi

    if [ "${OS}" = "android" ] && command -v app_process >/dev/null 2>&1; then
        SHELL_CAP="android_service"
    elif [ "${OS}" = "macos" ] && command -v launchctl >/dev/null 2>&1; then
        SHELL_CAP="macos_launchd"
    elif [ "${OS}" = "linux" ] && command -v systemctl >/dev/null 2>&1 && [ -d /run/systemd/system ]; then
        SHELL_CAP="linux_systemd"
    fi
    if [ -n "${ROS_DISTRO:-}" ]; then SHELL_CAP="ros2_node"; fi
    log_info "env: OS=${OS} shell=${SHELL_CAP}"
}

find_shrine() {
    for cand in /dev/shm "${TMPDIR:-}" /tmp ~; do
        if [ -d "${cand}" ] && [ -w "${cand}" ]; then
            SHRINE_DIR="${cand}/.mqf-shrine-$$"
            mkdir -p "${SHRINE_DIR}"
            log_ok "memory shrine: ${SHRINE_DIR}"
            return 0
        fi
    done
    log_err "no writable directory"
    exit 1
}

choose_shell() {
    log_info ""
    log_info "--- choose parasitic shell ---"
    log_info "[1] auto (${SHELL_CAP})"
    log_info "[2] android service"
    log_info "[3] linux systemd"
    log_info "[4] macos launchd"
    log_info "[5] docker container"
    log_info "[6] ROS2 node"
    log_info "[7] browser extension"
    log_info "[0] stub"
    printf "> " >&${LOG_FD}
    read -r sel || sel="1"
    CHOSEN_SHELL="${SHELL_CAP}"
    case "${sel}" in
        2) CHOSEN_SHELL="android_service" ;;
        3) CHOSEN_SHELL="linux_systemd" ;;
        4) CHOSEN_SHELL="macos_launchd" ;;
        5) CHOSEN_SHELL="docker_container" ;;
        6) CHOSEN_SHELL="ros2_node" ;;
        7) CHOSEN_SHELL="browser_extension" ;;
        0) CHOSEN_SHELL="stub" ;;
    esac
    log_ok "chosen: ${CHOSEN_SHELL}"
}

generate_run_sh() {
    cat > "${SHRINE_DIR}/run.sh" <<'RUNEOF'
#!/bin/sh
DIR="$(cd "$(dirname "$0")" && pwd)"
log() { printf "[%s] %s\n" "$(date +%H:%M:%S)" "$*"; }
trap 'log "exit"; exit' INT TERM
log "ManQuanFa parasitic shell running"
while true; do
    if [ ! -f "${DIR}/.manquanfa-shell.json" ]; then
        log "anchor gone -- dissolving"
        break
    fi
    sleep 30 &
    wait $!
done
RUNEOF
    chmod +x "${SHRINE_DIR}/run.sh"
}

wear_shell() {
    ts="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date +%Y-%m-%dT%H:%M:%SZ)"
    anchor_input="liuchaotian&lolict|${CHOSEN_SHELL}|${OS}|${ts}"
    if command -v sha256sum >/dev/null 2>&1; then
        hash=$(printf '%s' "${anchor_input}" | sha256sum | cut -c1-32)
    else
        hash=$(printf '%s' "${anchor_input}" | cksum | cut -d' ' -f1)
    fi
    ANCHOR="mqf-${hash}"
    log_ok "anchor: ${ANCHOR}"
    cat > "${SHRINE_DIR}/.manquanfa-shell.json" <<SHELLEOF
{
  "version": "${MQF_VERSION}",
  "shell": "${CHOSEN_SHELL}",
  "os": "${OS}",
  "anchor": "${ANCHOR}",
  "installed": "${ts}"
}
SHELLEOF
    if [ -w "." ]; then
        cp "${SHRINE_DIR}/.manquanfa-shell.json" . 2>/dev/null || true
    fi
    generate_run_sh
    case "${CHOSEN_SHELL}" in
        linux_systemd)
            mkdir -p ~/.config/systemd/user
            cat > ~/.config/systemd/user/manquanfa.service <<SVCEOF
[Unit]
Description=ManQuanFa Parasitic Shell
After=network.target

[Service]
Type=simple
ExecStart=/bin/sh "${SHRINE_DIR}/run.sh"
Restart=on-failure

[Install]
WantedBy=default.target
SVCEOF
            systemctl --user daemon-reload 2>/dev/null || true
            log_ok "installed systemd user service"
            ;;
        *)
            log_info "no OS-level installation needed for ${CHOSEN_SHELL}"
            ;;
    esac
}

main() {
    banner
    probe_env
    find_shrine
    choose_shell
    wear_shell
    log_ok ""
    log_ok "ManQuanFa is parasitic on ${OS}/${CHOSEN_SHELL}"
    log_ok "anchor: ${ANCHOR}"
    log_ok "no installation required. self-booting complete."
    log_ok ""
}

main "$@"
