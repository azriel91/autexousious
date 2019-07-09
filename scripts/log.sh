#! /bin/bash

# Log levels:
#  0 - error
#  1 - warn
#  2 - notice
#  3 - info (default)
#  4 - debug
#
# Example usage (from repository root):
#
# ```
# LOG_LEVEL=4 # Optionally change the log level
# source ./scripts/log.sh
# ```
LOG_LEVEL_REAL=${LOG_LEVEL:-3}
ERRORS_EXIST=""

FMT_RESET="\033[0m"
FMT_ERR="\033[1;31m"        # red bold
FMT_ERR_MSG="\033[0;31m"    # red
FMT_WARN="\033[1;33m"       # yellow bold
FMT_WARN_MSG="\033[0;33m"   # yellow
FMT_NOTICE="\033[1;32m"     # green bold
FMT_NOTICE_MSG="\033[0;32m" # green
FMT_INFO="\033[1;36m"       # cyan bold
FMT_INFO_MSG="\033[0;36m"   # cyan
FMT_DEBUG="\033[1;34m"      # blue bold
FMT_DEBUG_MSG="\033[0;34m"  # blue

error() {
  message=$1
  fatal=${2:-true}
  printf "${FMT_ERR}Error:${FMT_RESET}${FMT_ERR_MSG} ${message}${FMT_RESET}\n" 1>&2

  ERRORS_EXIST=true
  if $fatal; then exit 1; fi
}

warn() {
  message=$1
  [ $LOG_LEVEL_REAL -lt 1 ] && return
  printf "${FMT_WARN}Warn:${FMT_RESET}${FMT_WARN_MSG} ${message}${FMT_RESET}\n" 1>&2
}

notice() {
  message=$1
  [ $LOG_LEVEL_REAL -lt 2 ] && return
  printf "${FMT_NOTICE}Notice:${FMT_RESET}${FMT_NOTICE_MSG} ${message}${FMT_RESET}\n" 1>&2
}

info() {
  message=$1
  [ $LOG_LEVEL_REAL -lt 3 ] && return
  printf "${FMT_INFO}Info:${FMT_RESET}${FMT_INFO_MSG} ${message}${FMT_RESET}\n" 1>&2
}

debug() {
  message=$1
  [ $LOG_LEVEL_REAL -lt 4 ] && return
  printf "${FMT_DEBUG}Debug:${FMT_RESET}${FMT_DEBUG_MSG} ${message}${FMT_RESET}\n" 1>&2
}

exit_with_help() {
  # Note: the following is tab indented because bash heredocs can unindent tabs but not spaces
  help_message=$(cat <<-EOF
				${FMT_ERR_MSG}The build contained some errors.${FMT_RESET}
				${FMT_ERR_MSG}Search for ${FMT_RESET}${FMT_ERR}Error:${FMT_RESET}${FMT_ERR_MSG} in the build output to find them.${FMT_RESET}
				For more information, you may enable debug logging by running:
				LOG_LEVEL=4 cargo make --no-workspace \$task
				EOF
        )
  printf "${help_message}\n" 1>&2
  exit 1
}

fail_if_error() {
  [ -n "${ERRORS_EXIST}" ] && exit_with_help
}