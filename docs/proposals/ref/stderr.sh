#!/usr/bin/env bash
#===============================================================================
#-----------------------------><-----------------------------#
#$ name:stderr
#$ author:qodeninja
#$ date:
#$ semver:
#$ autobuild: 00001
#-----------------------------><-----------------------------#
#=====================================code!=====================================
  

  readonly LIB_STDERR="${BASH_SOURCE[0]}";
  _index=

#-------------------------------------------------------------------------------
# Load Guard
#-------------------------------------------------------------------------------

if ! _index=$(is_lib_registered "LIB_STDERR"); then 

  register_lib LIB_STDERR;

  readonly LOCAL_LIB_DIR="$(dirname ${LIB_STDERR})";
  source "${LOCAL_LIB_DIR}/escape.sh";


  # QUITE_MODE guards set in stdopts.options()
  # DEV_MODE guards set in stdopts.options()
  # TEST_MODE guards set directly in __log dispatcher
  
#-------------------------------------------------------------------------------
# Printers
#-------------------------------------------------------------------------------

  # @note : logo function moved to knife, sans color and quiet
  __logo(){
    local src=$1 r1=${2:-3} r2=${3:-9};
    if [ -z "$opt_quiet" ] || [ $opt_quiet -eq 1 ]; then
      local logo=$(sed -n "${r1},${r2} p" $src)
      printf "\n%b%s %s\n" "$blue" "${logo//#/ }" "$x" 1>&2;
    fi
  }


  __printx() {
    local text=$1 color=$2 prefix=$3 stream=${4:-2}
    local color_code=${!color:-$white2}
    [ -n "$text" ] && printf "%b" "${color_code}${prefix}${text}${x}" >&$stream
  }

  __log() {
    local type=$1 text=$2 force=$3 stream=2;
    case "$type" in
      warn)  [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "orange" "$delta "  $stream ;;
      okay)  [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "green"  "$pass "   $stream ;;
      info)  [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "blue"   "$lambda " $stream ;;
      note)  [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "grey"   "$colon2 " $stream ;;
      silly) [[ $force -eq 0 || $opt_silly -eq 0 ]] && __printx "$text\n" "purple" "$idots "  $stream ;;

      recov) [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "purple2" "$recv "  $stream ;;
      think) [[ $opt_trace -eq 0 ]]                 && __printx "$text\n" "white2"   "$idots "  $stream ;;
      trace) [[ $opt_trace -eq 0 ]]                 && __printx "$text\n" "grey2"   "$darr "  $stream ;;
      magic) [[ $force -eq 0 || $opt_trace -eq 0 ]] && __printx "$text\n" "purple" "$spark "  $stream ;;
      error)                                           __printx "$text\n" "red"   "$fail "   $stream ;;
      uclock) [[ $force -eq 0 || $opt_debug -eq 0 ]] && __printx "$text\n" "cyan"   "$uhour "   $stream ;;
    esac
  }

  stderr(){ [ -z "$QUIET_MODE" ] && [ -z "$QUIET_BOOT_MODE" ] &&  printf "%b" "${1}${xx}\n" 1>&2; }


  recover() { __log recov  "$1" "${2:-1}"; }
  warn()  { __log warn  "$1" "${2:-1}"; }
  okay()  { __log okay  "$1" "${2:-1}"; }
  info()  { __log info  "$1" "${2:-1}"; }
  note()  { __log note  "$1" "${2:-1}"; }
  silly() { __log silly "$1" "${2:-1}"; }
  magic() { __log magic "$1" "${2:-1}"; } # new
  trace() { __log trace "$1"; }
  think() { __log think "$1"; }
  error() { __log error "$1"; }
  uclock(){ __log uclock "$1"; } # new

  log_dev()   { __log dev "$1"; }
  log_test()  { __log qa "$1"; }
  


  __printf(){
    local text color prefix
    text=${1:-}; color=${2:-white2}; prefix=${!3:-};
    [ -n "$text" ] && printf "${prefix}${!color}%b${x}" "${text}" 1>&2 || :
  }


  __nl(){ printf "\n" 1>&2; }
  __x(){ printf "${x}" 1>&2; };


  __printbox(){
    if [ $opt_quiet -ne 0 ];  then
      local text="${1:-}";         # multiline string or single line
      local color="${2:-white2}";  # color for text + border
      local prefix="${!3:-}";      # optional glyph/symbol
      local stream="${4:-2}";      # default to stderr
      local width=70;              # max width, tweak if needed
      local border_char="-";
      local color_val=${!color:-$white2};
      local none="";
      # Build border line
      local border=""
      while [ ${#border} -lt $width ]; do
        border="${border}${border_char}"
      done

      # Top border
      printf "\n%b\n" "${color_val}${border}${prefix}${x}${nl}" >&$stream

      printf '%s\n' "$text" | while IFS= read -r line || [[ -n "$line" ]]; do
        printf "%b\n" "${sp}${sp}${color_val}${line}${x}" >&$stream
      done

      # Bottom border
      printf "\n%b\n\n" "${color_val}${border}${prefix}${x}" >&$stream
    else
      printf "Refuse to print box! ($opt_quiet)";
    fi
  }

  __box(){ __printbox "$1" "white" "none"; }
  __boltbox(){ __printbox "$1" "blue" "bolt"; }
  __docbox(){   __printbox "$1" "purple" "lambda"; }
  __errbox(){   __printbox "$1" "red" "none"; }
  __devbox(){   __printbox "$1" "red2" "none"; }
  

  __banner() {
      local text="$1"; local color="$2"; local fill_char="${3:--}"  width;

      # Get terminal width, defaulting to 80 if tput is not available
      width=$(tput cols 2>/dev/null || echo 80);

      # The visible text block includes the text plus two spaces on each side
      local text_block_len=$(( ${#text} + 4 ));

      # If the text is wider than the screen, just print it centered and colored
      if (( text_block_len >= width )); then
          printf "\n%b%s%b\n" "${color}" "  ${text}  " "${C_RESET}" >&2;
          return 0;
      fi

      # Calculate how many filler characters are needed
      local total_filler_len=$((width - text_block_len));
      local left_filler_len=$((total_filler_len / 2));
      local right_filler_len=$((total_filler_len - left_filler_len));

      # Build the left and right filler bars
      # Using a loop for maximum portability (avoids issues with seq or brace expansion)
      local left_bar=""
      for ((i=0; i<left_filler_len; i++)); do left_bar="${left_bar}${fill_char}"; done
      
      local right_bar=""
      for ((i=0; i<right_filler_len; i++)); do right_bar="${right_bar}${fill_char}"; done

      # Print the final banner to stderr
      # The structure is [left-bar][space][space][colored-text][space][space][right-bar]
      printf "\n%s  %b%s%b  %s\n" "${left_bar}" "${color}" "${text}" "${C_RESET}" "${right_bar}" >&2
  }


  xline(){ stderr "${blue}${LINE}↯\n${x}"; }

  #todo: refactor to use __log
  fatal(){ trap - EXIT; __printf "\n$red$fail $1 $2 \n"; exit 1; }


  toggle_quiet() {
    opt_quiet=$((1 - opt_quiet))
    return "$opt_quiet"
  }

  toggle_debug() {
    opt_debug=$((1 - opt_debug))
    return "$opt_debug"
  }

  debug_on(){ opt_debug=0; }
  
  quiet_off(){ opt_quiet=1; }

  require_dev(){
    [ "$opt_dev" -eq 0 ] && return 0;
    return 1;
  }

  __flag(){ 
    local flag=${1:-1} lbl=$2 color=;
    [ $flag -eq 0 ] && icon=$flag_on && color=$green;
    [ $flag -eq 1 ] && icon=$flag_off && color=$grey2;
    [ -n "$lbl"   ] && lbl=" ${lbl}"; 
    printf "%b%s%s%b" "$color" "$icon" "$lbl" $x;
  } 
    

  dump(){
      local len arr i this flag newl
      arr=("${@}"); len=${#arr[@]}
      [ $__buf_list -eq 0 ] && flag="\r" &&  newl="$eol" || newl="\n"
      if [ $len -gt 0 ]; then
        handle_input
        for i in ${!arr[@]}; do
          this="${arr[$i]}"
          [ -n "$this" ] && printf -v "out" "$flag$opt_dump_col$dots(%02d of %02d) $this $x" "$i" "$len"
          trace "$out"
          sleep 0.05
        done
        cleanup
        printf -v "out" "$flag$green$pass (%02d of %02d) Read. $x$eol" "$len" "$len"
        trace "$out"
      fi
    }

#-------------------------------------------------------------------------------
# Sig / Flow
#-------------------------------------------------------------------------------
    


  handle_sigint(){ s="$?"; kill 0; exit $s;  }
  handle_interupt(){ E="$?";  kill 0; exit $E; }
  handle_stop(){ kill -s SIGSTOP $$; }
  handle_input(){ [ -t 0 ] && stty -echo -icanon time 0 min 0; }
  cleanup(){ [ -t 0 ] && stty sane; }

  fin(){
      local E="$?"; cleanup
      if [ -z "$opt_quiet" ]; then
         [ $E -eq 0 ] && __printf "${green}${pass} ${1:-Done}." \
                      || __printf "$red$fail ${1:-${err:-Cancelled}}."
      fi
  }

  trap handle_interupt INT
  trap handle_stop SIGTSTP
  trap handle_input CONT
  trap fin EXIT

  # function   fatal(){ trap - EXIT; __print "\n$fail $1 [$2] \n"; exit 1; }
  # function   quiet(){ [ -t 1 ] && opt_quiet=${1:-1} || opt_quiet=1; }
  # function  status(){
  #   local ret res msg
  #   ret=$1; res=$2; msg=$3; __print "$res";
  #   [ $ret -eq 1 ] && fatal "Error: $msg, exiting" "1";
  #   return 0
  # }

#-------------------------------------------------------------------------------
# PROMPTS
#-------------------------------------------------------------------------------
    

  # Robust confirmation prompt. Reads from the TTY to avoid consuming piped stdin.
  # Respects the --yes flag.
  __confirm() {
    local prompt="${1:-Are you sure?}" answer

    # If --yes is passed, auto-confirm and don't prompt.
    if [ "${opt_yes:-1}" -eq 0 ]; then
      __printf "${prompt} ${bld}${green}auto-yes${x}\n"
      return 0
    fi

    # Ensure we read from the terminal, not from stdin if it's being piped.
    local tty_dev="/dev/tty"
    if ! [ -t 0 ] && ! [ -c "$tty_dev" ]; then
      error "Cannot ask for confirmation without a terminal."
      return 1 # Fail confirmation
    fi

    while true; do
      # Prompt on stderr, read a single character from the TTY.
      __printf "${prompt} [y/n/q] > " "white2"
      read -r -n 1 answer < "$tty_dev"
      case "$answer" in
        ([Yy]) __printf "${bld}${green}yes${x}\n"; return 0 ;;
        ([Nn]) __printf "${bld}${red}no${x}\n"; return 1 ;;
        ([Qq]) __printf "${bld}${purple}quit${x}\n"; exit 1 ;;
        (*)    __printf "\n${yellow}Invalid input. Please try again.${x}\n" ;;
      esac
    done
  }


  __prompt(){
    local msg="$1" default="$2"
     if [ "${opt_yes:-1}" -eq 1 ]; then # Only prompt if opt_yes is NOT 0 (i.e., not auto-yes)
      read -p "$msg --> " answer
      [ -n "$answer" ] && echo "$answer" || echo "$default"
    else # If opt_yes is 0 (auto-yes), just return the default
      echo "$default"
    fi
  }

  __ask(){
    local msg="$1" default="$2"
     if [ "${opt_yes:-1}" -eq 1 ]; then # Only prompt if opt_yes is NOT 0 (i.e., not auto-yes)
      read -p "$msg --> " answer
      [ -n "$answer" ] && echo "$answer" || echo "$default"
    else # If opt_yes is 0 (auto-yes), just return the default
      echo "$default"
    fi
  }

  prompt_path(){
      local res ret next
      prompt="$1"
      prompt_sure="$2"
      default="$3"
      prompt=$(eval echo "$prompt")
      while [[ -z "$next" ]]; do
        read -p "$prompt? > ${bld}${green}" __NEXT_DIR
        res=$(eval echo $__NEXT_DIR)
        [ -z "$res" ] && res="$default"
        if [ -n "$res" ]; then
          if [ "$res" = '?' ]; then
            echo "cancelled"
            return 1
          fi
          if confirm "${x}${prompt_sure} [ ${blue}$res${x} ] (y/n)"; then
            if [ ! -d "$res" ]; then
              error "Couldn't find the directory ($res). Try Again. Or '?' to cancel."
            else
              next=1
            fi
          fi
        else
          warn "Invalid Entry! Try Again."
        fi
      done
      echo "$res"
    }


  cprint() {
    typ=$1
    case $typ in
        a) alpha="abcedfghijklmnopqrstuvwxyz";;
        A) alpha="ABCDEFGHIJKLMNOPQRSTUVWXYZ";;
        h) alpha='abcdef0123456789';;
        H) alpha='ABCDEF0123456789';;
        v) alpha="aeiouy";;
        c) alpha="bcdfghjklmnpqrstvwxz" ;;
        V) alpha="AEIOUY" ;;
        C) alpha="BCDFGHJKLMNPQRSTVWXZ" ;;
        n) alpha="0123456789" ;;
        \#|-|_|.) alpha=$1;;
        *) echo "** Undefined **" ; exit 1 ;;
    esac
    len=${#alpha}
    r=$((RANDOM%len))
    echo -en ${alpha:r:1}
  }

  rprint() {
    code=$1
    for i in $(seq 1 ${#code})
    do
        c=${code:i-1:1}
        cprint $c
    done
    echo
  }

  dump_buffer(){
    local arr col
    arr=("${@}"); len=${#arr[@]}
    col=${1:-$opt_dump_col}
    if [ $len -gt 0 ]; then
      for i in ${!arr[@]}; do
        this="${arr[$i]}"
        [ -n "$this" ] && printf -v "out" "$opt_dump_col(%d) %s\n$x" "$i" "$this"
        printf "$out"
      done
    fi
  }

  prompt_buffer(){
    local res ret next
    prompt="$1"
    prompt_lbl="$2"
    default="$3"
    arr=("${__buffer[@]}"); len=${#arr[@]}

    while [[ -z "$next" ]]; do
      read -p "$prompt? > ${bld}${green}" __NEXT_VAL
      res=$(eval echo $__NEXT_DIR)
      ## TODO...
    done
  }


substring_filter(){
	local pattern=$1
	[[ $pattern == "all" ]] && pattern="*";
	if [[ -z "$pattern" ]]; then
			echo "Error: No pattern provided to substring_filter." >&2;
			return 1
	fi
	while IFS= read -r line; do
		if [[ "$line" == *"$pattern"* ]]; then
				echo "$line"
		fi
	done
}


# Filters a stream of text, supporting multiple inclusion and exclusion patterns.
#
# Usage:
#   ... | substring_filter "foo" "bar"     # OR: Prints lines containing "foo" OR "bar"
#   ... | substring_filter "!foo" "!bar"    # AND: Prints lines containing NEITHER "foo" NOR "bar"
#   ... | substring_filter "foo" "!bar"   # Prints lines containing "foo" AND NOT "bar"
#   ... | substring_filter              # With no args, prints all lines (like 'cat')
#
substring_super_filter() {
    # If no patterns are given, just print everything and exit.
    if [[ $# -eq 0 ]]; then
        cat
        return
    fi

    # --- 1. Argument Parsing: Sort patterns into include/exclude arrays ---
    local -a includes=()
    local -a excludes=()
    for arg in "$@"; do
        if [[ "$arg" == "!"* ]]; then
            # Add to excludes list, removing the leading '!'
            excludes+=("${arg#!}")
        else
            # Add to includes list
            includes+=("$arg")
        fi
    done

    # --- 2. Main Filtering Loop ---
    while IFS= read -r line; do
        local is_excluded=false
        local is_included=false

        # --- Rule A: Check for Exclusions (AND logic) ---
        # The line must NOT match ANY exclusion pattern.
        for pattern in "${excludes[@]}"; do
            if [[ "$line" == *"$pattern"* ]]; then
                is_excluded=true
                break # Found a reason to exclude, no need to check further.
            fi
        done

        # If the line is excluded, skip it immediately.
        if "$is_excluded"; then
            continue
        fi

        # --- Rule B: Check for Inclusions (OR logic) ---
        # If we get here, the line was not excluded.
        # Now, does it meet the inclusion criteria?

        # If there are no inclusion patterns, then any non-excluded line is a match.
        if (( ${#includes[@]} == 0 )); then
            is_included=true
        else
            # Otherwise, the line must match AT LEAST ONE inclusion pattern.
            for pattern in "${includes[@]}"; do
                if [[ "$line" == *"$pattern"* ]]; then
                    is_included=true
                    break # Found a reason to include, no need to check further.
                fi
            done
        fi

        # If the line passed both exclusion and inclusion checks, print it.
        if "$is_included"; then
            echo "$line"
        fi
    done
}


#-------------------------------------------------------------------------------
# Load Guard Error
#-------------------------------------------------------------------------------

else
  error "Library LIB_STDERR found at index [$_index]";
  return 1;
fi
