#!/usr/bin/env bash
#===============================================================================
#-----------------------------><-----------------------------#
#$ name:templating
#$ author:qodeninja
#$ desc: A library for extracting and rendering text blocks from files.
#-----------------------------><-----------------------------#
#=====================================code!=====================================

  echo "loaded stderr.sh";

  strip_leading_comment() {
    sed 's/^[[:space:]]*#[[:space:]]//'; #dont strip all whitesapce just maybe one
  }

  escape_sed_replacement(){
    printf '%s\n' "$1" | sed 's/[\/&\\]/\\&/g'
  }

  deref_var() {
    local __varname="$1"
    [[ "$__varname" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]] || return 1
    eval "printf '%s' \"\$${__varname}\""
  }

  expand_vars() {
    local raw="$1" output="" varname value
    local prefix rest matched=1  # default to no match
    while [[ "$raw" == *'$'* ]]; do
      prefix="${raw%%\$*}"
      rest="${raw#*\$}"
      varname=$(expr "$rest" : '\([a-zA-Z_][a-zA-Z0-9_]*\)')
      # If no valid varname, break
      [ -z "$varname" ] && break
      value=$(deref_var "$varname") #bash 3.2
      rest="${rest#$varname}"
      output+="$prefix$value"
      raw="$rest"
      matched=0
    done
    output+="$raw"
    printf '%s\n' "$output"
    return $matched
  }

  expand_line_vars(){
    local ret out lineX line="$1";

    [ "${opt_dev:-1}" -eq 0 ] && [ "${opt_silly:-1}" -eq 0 ] && silly "$line";

    if [[ "$line" == *'$'* ]]; then
      lineX=$(expand_vars "$line"); ret=$?;
      if [ "${opt_dev:-1}" -eq 0 ] && [ "${opt_silly:-1}" -eq 0 ]; then
        [ $ret -eq 0 ] && info "$lineX <--expanded";
        [ $ret -eq 1 ] && note "$lineX --> skipped";
      fi
      out="$lineX";
      ret=0;
    else
      out="$line";
      ret=1;
    fi
    echo -e "$out";
    return $ret;
  }

  replace_escape_codes(){
    local input ret res shebang esc_shebang;

    if [ -p /dev/stdin ]; then
      input="" # Initialize empty
      while IFS= read -r line || [[ -n $line ]]; do
        line=$(expand_line_vars "$line");ret=$?;
        input+=$line$'\n'
      done
    elif [ -n "$1" ]; then
      input="$1"
    else
      error "Error: No input provided to replace_escape_codes"
      return 1
    fi

    #shebang needs special babysitting for sed
    shebang="#!/usr/bin/env bash";
    esc_shebang=$(escape_sed_replacement "$shebang");

    #replace data
    input="${input//%date%/$(date +'%Y-%m-%d %H:%M:%S')}";

    # Replace color codes and glyphs.
    echo "$input" |
      sed "s|\${x}|$x|g" | sed "s|\${rev}|$revc|g" |
      sed "s|\${r}|$red|g" | sed "s|\${o}|$orange|g" | sed "s|\${c}|$cyan|g" |
      sed "s|\${g}|$green|g" | sed "s|\${isnek}|$snek|g" | sed "s|\${it}|$itime|g" |
      sed "s|\${id}|$delta|g" | sed "s|\${il}|$lambda|g" | sed "s|\${isp}|$spark|g" |
      sed "s|\${spark}|$spark|g" | sed "s|\${star}|$star|g" | sed "s|\${bolt}|$bolt|g" |
      sed "s|\${b2}|$blue2|g" | sed "s|\${w2}|$white2|g" | sed "s|\${p}|$purple|g" |
      sed "s|\${u}|$grey|g" | sed "s|\${y}|$yellow|g" | sed "s|\${b}|$blue|g" |
      sed "s|\${w}|$white|g" | sed "s|\${u2}|$grey2|g" | sed "s|\${r2}|$red2|g" |
      sed "s|\${bld}|$bld|g" | sed "s|\${line}|$line|g" | sed "s|\${LINE}|$LINE|g" |
      sed "s|\${ff}|$flag_on|g" | sed "s|\${fo}|$flag_off|g" | sed "s|\${shebang}|$esc_shebang|g"

    return 0
  }

  sed_block(){
    local id="$1" target="$2" pre="^[#]+[=]+" post=".*" str end;
    if [[ -f $target ]]; then
      str="${pre}${id}[:]?[^\!=\-]*\!${post}";
      end="${pre}\!${id}[:]?[^\!=\-]*${post}";
      sed -rn "1,/${str}/d;/${end}/q;p" "$target" | strip_leading_comment | replace_escape_codes;
      return 0;
    fi
    # Let the caller handle the error
    return 1;
  }

  block_print(){
    local lbl="$1" target="$2" IFS res ret;
    res=$(sed_block "$lbl" "$target"); ret=$?;
    if [ $ret -ne 0 ] || [ -z "$res" ]; then
      err="Block '$lbl' not found or empty in '$target'";
      return 1;
    fi

    printf '%s\n' "$res" | while IFS= read -r line; do
      if [[ $lbl =~ ^(doc|inf|rc|link|conf).* ]]; then
        printf '%b\n' "$line"
      else
        printf '%s\n' "$line"
      fi
    done
    return 0;
  }




#-------------------------------------------------------------------------------
# Universal Block Functions
#-------------------------------------------------------------------------------

  get_block(){
    local res ret src=$1 lbl=$2;
    res=$(sed -n "/#### ${lbl} ####/,/########/p" "$src");
    [ -z "$res" ] && ret=1 || ret=0;
    echo "$res";
    return $ret;
  }


	get_embedded_doc(){
    local str ret src=$1 lbl=$2;
    trace "Getting embedded link. (label=$lbl)";
    [ -z "$lbl" ] || [ -z "$src" ]||[ ! -f "$src" ]  && { 
      fatal "Cant read embedded doc invalid args ($1) ($2)";
      return 1;
    }
    str=$(block_print "$lbl" "$src");
    
    if [ ${#str} -gt 0 ]; then
      echo -e "$str"
    else 
      error "Problem reading embedded link";
      exit 1;
    fi
	}


#-------------------------------------------------------------------------------
# Dev Drivers 
#-------------------------------------------------------------------------------

  # debug embeded docs
  # supports <this> context so src does not have to be passed
  dev_print_embed(){
    if require_dev; then 
      opt_trace=0;
      local src=$2 ret res lbl=$1; #note that src and lbl are backwards here vs other funcs
      src="${src:-$THIS_SELF}"; 
      res="$(get_embedded_doc $src $lbl )";ret=$?;
      [ $ret -eq 0 ] && __docbox "$res";
      return $ret;
    else
      error "[DEV GUARD]. 'dev_rc_embed' aborted.";
    fi
    return 1;
  }
