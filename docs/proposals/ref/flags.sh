#!/usr/bin/env bash
#===============================================================================
#-------------------------------------------------------------------------------
#$ name: 
#$ author: 
#$ semver: 
#-------------------------------------------------------------------------------
#=====================================code!=====================================


  readonly LIB_FLAGS="${BASH_SOURCE[0]}";
  _index=

#-------------------------------------------------------------------------------
# Load Guard
#-------------------------------------------------------------------------------

if ! _index=$(is_lib_registered "LIB_FLAGS"); then 

  register_lib LIB_FLAGS;

# note: %bitmap templates
#
# ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
# │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
# ├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
# │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │    │
# └────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘
# ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
# │ 15 │ 14 │ 13 │ 12 │ 11 │ 10 │ 09 │ 08 │ 07 │ 06 │ 05 │ 04 │ 03 │ 02 │ 01 │ 00 │ Bit #
# ├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤
# │ AA │ BB │ CC │ DD │ EE │ FF │ GG │ HH │ II │ JJ │ KK │ LL │ MM │ NN │ OO │ PP │ Label
# └────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘

# === Core Bitmask Operations ===
FLAG_32=4294967295;
FLAG_16=65535;
FLAG_8=255;


fx_flag_on_val() {
  local value="$1" mask="$2";
  [ $((value & mask)) -ne 0 ]
}


flag_on() {
  local name="$1" mask="$2"
  __valid_flag "$name" || return 1;
  [ $(( ${!name} & mask )) -ne 0 ]
}

flag_off() {
  local name="$1" mask="$2"
  __valid_flag "$name" || return 1;
  [ $(( ${!name} & mask )) -eq 0 ]
}

flag_enable() {
  local name="$1" mask="$2" val
  __valid_flag "$name" || return 1;
  val=$(( ${!name} | mask ))
  eval "$name=$val"
}

flag_disable() {
  local name="$1" mask="$2" val
  __valid_flag "$name" || return 1;
  val=$(( ${!name} & ~mask ))
  eval "$name=$val"
}

flag_xor() {
  local name="$1" mask="$2" val
  __valid_flag "$name" || return 1;
  val=$(( ${!name} ^ mask ))
  eval "$name=$val"
}

flag_switch_all() {
  local name="$1" state="$2" val
  __valid_flag "$name" || return 1;
  case "$state" in
    (1) val=$FLAG_16;;  # You can later make this dynamic
    (0) val=0 ;;
    (*) echo "Invalid switch state: $state (must be 0 or 1)" >&2; return 1 ;;
  esac
  eval "$name=$val"
}


flag_enable_all()  { flag_switch_all "$1" 1; }
flag_disable_all() { flag_switch_all "$1" 0; }

__max_bit_len(){
  local bits="$1"
  case "$bits" in
    (''|*[!0-9]*) echo "Invalid bit width: ${bits}" >&2; return 1 ;;
    (*) echo $(( 1 << $bits - 1 )) ;;
  esac
}


__valid_flag(){
  case "$1" in
    ([a-zA-Z_][a-zA-Z0-9_]*) return 0 ;;
    (*) echo "Invalid flag variable name: '$1'" >&2; return 1 ;;
  esac
}
#
# SUB-FUNCTION: Prepares the label array and legend (Optimized).
#
__flagmap_prepare_labels() {
  local bitmask=$1 bitcount=$2 map_string=$3
  local -n labels_out_ref=$4 # Nameref for the output label array
  local -n legend_out_ref=$5 # Nameref for the output legend string

  local -a temp_labels
  local -a legend_lines # OPTIMIZATION: Use an array for legend lines

  if [[ -n "$map_string" ]]; then
    # --- Custom Map Mode ---
    local -A custom_names custom_codes
    local default_name=""
    local legend_code='A'

    local -a map_items; IFS=',' read -r -a map_items <<< "$map_string"
    for item in "${map_items[@]}"; do
      if [[ $item == XX:* ]]; then
        default_name="${item#*:}"
      else
        local index="${item%%:*}" name="${item#*:}"
        custom_names[$index]="$name"
        custom_codes[$index]="$legend_code"
        legend_code=$(printf "\\$(printf '%03o' "$(( $(printf '%d' "'$legend_code") + 1 ))")")
      fi
    done
    
    local is_default_in_legend=0
    for ((i = 0; i < bitcount; i++)); do
      if ! (( (bitmask >> i) & 1 )); then
        temp_labels[i]="  "; continue
      fi

      if [[ -v "custom_names[$i]" ]]; then
        temp_labels[i]="${custom_codes[$i]}"
        legend_lines+=("$(printf "#    %2s : %s" "${custom_codes[$i]}" "${custom_names[$i]}")")
      elif [[ -n "$default_name" ]]; then
        temp_labels[i]="XX"
        if (( ! is_default_in_legend )); then
          legend_lines+=("$(printf "#    XX : %s (default)" "$default_name")")
          is_default_in_legend=1
        fi
      else
        temp_labels[i]="  "
      fi
    done

  else
    # --- Automatic Label Mode ---
    for ((i = 0; i < bitcount; i++)); do
      if ! (( (bitmask >> i) & 1 )); then temp_labels[i]="  "; continue; fi
      if (( bitcount <= 16 )); then
        temp_labels[i]=$(printf "%c%c" "$((i+65))" "$((i+65))")
      else
        temp_labels[i]="$(printf "%c" "$(( (i % 16) + 65 ))")$(( i / 16 + 1 ))"
      fi
    done
  fi
  
  labels_out_ref=("${temp_labels[@]}")
  # OPTIMIZATION: Join legend lines once at the end.
  (IFS=$'\n'; legend_out_ref="${legend_lines[*]}")
}

#
# SUB-FUNCTION: Renders the visual table grid (Optimized).
#
__flagmap_generate_table() {
  local bitmask=$1 bitcount=$2
  local -n labels=$3 # Nameref to the prepared label array

  local i j start end val
  local rows=$(( (bitcount + 15) / 16 ))

  for ((j = rows - 1; j >= 0; j--)); do
    start=$(( j * 16 )); end=$(( start + 15 < bitcount ? start + 15 : bitcount - 1 ))
    
    # OPTIMIZATION: Build borders with printf, not sed. Much faster.
    local top_border="# ┌"; local mid_border="# ├"; local bot_border="# └"
    for ((i=end; i>=start; i--)); do
      top_border+="────┬"
      mid_border+="────┼"
      bot_border+="────┴"
    done
    printf "%s┐\n" "${top_border%?}"
    printf "# │"; for ((i=end; i>=start; i--)); do printf " %02d │" "$i"; done; printf "\n"
    printf "%s┤\n" "${mid_border%?}"
    printf "# │"; for ((i=end; i>=start; i--)); do val=$(((bitmask>>i)&1)); printf "  %d │" "$val"; done; printf "\n"
    printf "# │"; for ((i=end; i>=start; i--)); do printf " %2.2s │" "${labels[i]}"; done; printf "\n"
    printf "%s┘\n" "${bot_border%?}"
  done
}

#
# MAIN FUNCTION: Orchestrates the process. (No changes needed here)
#
flagmap_visual() {
  local bitmask=$1 bitcount=$2 outvar=$3 custom_map_string=$4
  
  local -a all_labels
  local legend=""
  __flagmap_prepare_labels "$bitmask" "$bitcount" "$custom_map_string" all_labels legend

  local table
  table=$(__flagmap_generate_table "$bitmask" "$bitcount" all_labels)

  # Prepend newline to legend only if it exists
  [[ -n "$legend" ]] && legend=$'\n'"$legend"
  
  printf -v "$outvar" '%s' "${table%?}${legend}"
}

#-------------------------------------------------------------------------------
# Load Guard Error
#-------------------------------------------------------------------------------

else

  error "Library LIB_FLAGS found at index [$_index]";
  exit 1;

fi
