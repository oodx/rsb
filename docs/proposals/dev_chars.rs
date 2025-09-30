//STAKEHOLDER NEW FEATURES FOR DEVELOPMENT

/*

count_nonprint(string) => count
count_emoji(string) => count

grapheme_len(char) => len
str_len_unicode(str) => len //gets proper character len including unicode sizes

list_nonprint(string) => 1: <p1>, 2: <p2>
list_emoji(string) =>  1: <e1>, 2: <e2>

//rsb asc100-style sentinel markers
list_markers(string) => 1: <m1>, 2: <m2>   #MARKER#



//these fnuctions are patterns used in rsb

rsb_esc_unicode(char, type [ default | ns | jynx | marker | token ] ) 
  default  => /u1111
  ns       => {u:1111}
  jynx     => %{u:1111}  // jynx template format
  token    => char:"u1111"; //token stream format
  marker   => #U1111# // asc100 marker format
  invalid  => #INV# // mark

rsb_esc_string( string, type [ default | ns | jynx | marker | token ] ) 
  default  => text /u1111 text  
  ns       => text {u:1111} text
  jynx     => text %{u:1111} text  // jynx template format
  token    => text:"text1";char:"u1111";text:"text2"; //token stream format
  marker   => text #U1111# text // asc100 marker format
  invalid  => text #INV# text  // mark



  strip_ansii(str)    // remove all ansi escape characters
  strip_unicode(str)  // remove all extended unicode characters
  strip_nonprint(str) // remove all nonprintable characters

  trim_line(str) // if we dont have trim already this should trim leading and trailing whitespace
*/
