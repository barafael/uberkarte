-- HyperTalk code corpus: real programs and idioms recovered from the internet.
-- Each node carries the verbatim script, attribution, and edges to the
-- keywords it uses and the concepts it demonstrates.

-- == From Wikipedia "HyperTalk" (CC BY-SA 4.0) ==============================
UPSERT code:wp_put_into SET
  title = "put ... into (natural-order assignment)", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "The signature assignment: predicate-last, creates the variable on the fly, no declaration anywhere.",
  script = 'put 5 * 4 into theResult',
  notes = 'The BASIC equivalent is theResult = 5 * 4. HyperTalk puts the destination last because the sentence order is put X into Y.';
RELATE code:wp_put_into->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_put_into->uses->keyword:cmd_put;
RELATE code:wp_put_into->demonstrates->concept:container_model SET note='the write face of the container model';
RELATE code:wp_put_into->demonstrates->concept:natural_language_syntax SET confidence='high';

UPSERT code:wp_container_ref SET
  title = "Three ways to address the same field", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "Explicit, elided, and ordinal addressing of the same container - all equivalent, chosen for readability.",
  script = 'put card field "typehere" into theValue
put fld "typehere" into theValue
put the first card field into theValue',
  notes = 'Context fills the gaps: card is inferred from where the script runs, the value property is the default target, and ordinals replace numbers.';
RELATE code:wp_container_ref->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_container_ref->uses->keyword:obj_field;
RELATE code:wp_container_ref->uses->keyword:obj_card;
RELATE code:wp_container_ref->uses->keyword:chk_ordinal;
RELATE code:wp_container_ref->uses->keyword:cmd_put;
RELATE code:wp_container_ref->demonstrates->concept:container_model SET note='navigational addressing by visual hierarchy';
RELATE code:wp_container_ref->demonstrates->concept:natural_language_syntax;

UPSERT code:wp_ask_it SET
  title = "ask / it round-trip", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "Dialog in, answer out - the it pseudo-variable as the implicit receiver of user input.",
  script = 'ask "What is the value?"
put it into card field "display"';
RELATE code:wp_ask_it->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_ask_it->uses->keyword:cmd_ask;
RELATE code:wp_ask_it->uses->keyword:sp_it;
RELATE code:wp_ask_it->uses->keyword:cmd_put;
RELATE code:wp_ask_it->demonstrates->concept:target_me_it SET note='it as the implicit accumulator';

UPSERT code:wp_hide_fields SET
  title = "Iterate a collection with the number of", kind = 'pattern', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "The canonical parts-loop: collections are containers too, and their number property is the loop bound.",
  script = 'repeat with i = 1 to the number of card fields
  hide field i
end repeat',
  notes = 'hide is sugar for set the visible of field i to false - command and property are two spellings of one operation.';
RELATE code:wp_hide_fields->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_hide_fields->uses->keyword:ctl_repeat;
RELATE code:wp_hide_fields->uses->keyword:fn_numberof;
RELATE code:wp_hide_fields->uses->keyword:cmd_hide;
RELATE code:wp_hide_fields->demonstrates->concept:property_model SET note='commands as property sugar';

UPSERT code:wp_chunk_walk SET
  title = "Chunk expressions nested to depth", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "Text is a container of containers: words inside lines inside fields, addressable in one sentence.",
  script = 'put the third word of theFilesText into colThree
put the first character of the third word of line 5 of card field "sometext" into theChar';
RELATE code:wp_chunk_walk->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_chunk_walk->uses->keyword:chk_word;
RELATE code:wp_chunk_walk->uses->keyword:chk_character;
RELATE code:wp_chunk_walk->uses->keyword:chk_line;
RELATE code:wp_chunk_walk->uses->keyword:obj_field;
RELATE code:wp_chunk_walk->demonstrates->concept:chunk_expression SET confidence='high';
RELATE code:wp_chunk_walk->demonstrates->concept:container_model;

UPSERT code:wp_file_open SET
  title = "Open, read, style: a file-picker handler", kind = 'handler', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "A complete mouseUp handler: answer file, guard with there is a file, read into it, style the result.",
  script = 'on mouseDown
  answer file "Please select a text file to open."
  if it is empty then exit mouseDown
  put it into filePath
  if there is a file filePath then
    open file filePath
    read from file filePath until return
    put it into cd fld "some field"
    close file filePath
    set the textStyle of character 1 to 10 of card field "some field" to bold
  end if
end mouseDown';
RELATE code:wp_file_open->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_file_open->uses->keyword:cmd_answer;
RELATE code:wp_file_open->uses->keyword:op_thereis;
RELATE code:wp_file_open->uses->keyword:cmd_fileio;
RELATE code:wp_file_open->uses->keyword:chk_range;
RELATE code:wp_file_open->uses->keyword:prop_family;
RELATE code:wp_file_open->uses->keyword:msg_mousedown;
RELATE code:wp_file_open->demonstrates->concept:container_model SET note='files join the same container algebra';

UPSERT code:wp_replacestr SET
  title = "replaceStr: search-and-replace as a function", kind = 'function', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0); one line's spacing differs from the article",
  summary = "The whole standard library of string search in four lines: repeat while pattern is in, offset, assignable chunk range.",
  script = 'function replaceStr pattern,newStr,inStr
  repeat while pattern is in inStr
    put offset(pattern,inStr) into pos
    put newStr into character pos to (pos + the length of pattern) - 1 of inStr
  end repeat
  return inStr
end replaceStr',
  notes = 'The chunk range character pos to ... is an lvalue: text rewriting happens by assignment, not by building a new string.';
RELATE code:wp_replacestr->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_replacestr->uses->keyword:ctl_function;
RELATE code:wp_replacestr->uses->keyword:op_verbal;
RELATE code:wp_replacestr->uses->keyword:fn_offset;
RELATE code:wp_replacestr->uses->keyword:fn_length;
RELATE code:wp_replacestr->uses->keyword:chk_range;
RELATE code:wp_replacestr->demonstrates->concept:chunk_expression SET note='chunks as assignable lvalues';

UPSERT code:wp_diag_buttons SET
  title = "Diagonal layout via item chunking", kind = 'handler', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "A coordinate pair is just a comma-delimited string; item 1 of pos is the x field of a hand-rolled struct.",
  script = 'on mouseUp
  put "100,100" into pos
  repeat with x = 1 to the number of card buttons
    set the location of card button x to pos
    add 15 to item 1 of pos
  end repeat
end mouseUp';
RELATE code:wp_diag_buttons->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_diag_buttons->uses->keyword:chk_item;
RELATE code:wp_diag_buttons->uses->keyword:cmd_set;
RELATE code:wp_diag_buttons->uses->keyword:prop_rect;
RELATE code:wp_diag_buttons->uses->keyword:cmd_add;
RELATE code:wp_diag_buttons->demonstrates->concept:pseudo_arrays SET confidence='high';

UPSERT code:wp_send_target SET
  title = "send a message to a named button", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "Messages are strings, targets are container expressions: invoke any object's handler from anywhere.",
  script = 'send "mouseUp" to card button "OK" of card "Veracity"';
RELATE code:wp_send_target->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_send_target->uses->keyword:cmd_send;
RELATE code:wp_send_target->uses->keyword:msg_mouseup;
RELATE code:wp_send_target->demonstrates->concept:message_path SET note='explicit injection into the path';

UPSERT code:wp_do_clickline SET
  title = "Click-to-run interpreter (clickLine + do)", kind = 'handler', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "A field of HyperTalk source that executes the line you click: clickLine names a container, do runs it.",
  script = 'on mouseUp
  select the clickLine
  put word 2 of the clickLine into linenum
  do line linenum of cd fld 1
end mouseUp',
  notes = 'clickLine returns text like "line 10 of card field 4" - a container description parsed with chunk expressions and re-enters the evaluator through do.';
RELATE code:wp_do_clickline->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_do_clickline->uses->keyword:fn_click;
RELATE code:wp_do_clickline->uses->keyword:cmd_do;
RELATE code:wp_do_clickline->uses->keyword:cmd_select;
RELATE code:wp_do_clickline->uses->keyword:chk_word;
RELATE code:wp_do_clickline->demonstrates->concept:evaluation_modes SET note='text re-entering the evaluator';
RELATE code:wp_do_clickline->demonstrates->concept:message_box SET note='the same evaluator the message box uses';

UPSERT code:wp_forgiving_select SET
  title = "Container descriptions as data (selectedLine)", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0); the explanatory comment line was added by the corpus",
  summary = "The selectedLine property returns a container description; HyperTalk re-parses it where a container is expected.",
  script = 'put the selectedLine of card field "Listbox" into theSelection
  -- theSelection now holds: line 2 to 3 of card field "Listbox"
select line (word 2 of theSelection) of card field "Listbox"',
  notes = 'Data that looks like a container reference is accepted in container position - the forgiving-semantics special case, without eval-like pitfalls.';
RELATE code:wp_forgiving_select->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_forgiving_select->uses->keyword:fn_selected;
RELATE code:wp_forgiving_select->uses->keyword:cmd_select;
RELATE code:wp_forgiving_select->demonstrates->concept:forgiving_semantics SET confidence='high';
RELATE code:wp_forgiving_select->demonstrates->concept:container_model;

UPSERT code:wp_forgiving_play SET
  title = "play harpsichord c e g (words as data)", kind = 'idiom', year = 1987,
  attribution = "Wikipedia: HyperTalk (CC BY-SA 4.0)",
  summary = "Bare words after a command are data, not function calls - three spellings of one melody, none of them evaluated.",
  script = 'play harpsichord c e g
play harpsichord "c e g"
put "c e g" into theMelody
play harpsichord theMelody',
  notes = 'And the pitfall that proves it is special-cased: play harpsichord "c e g()" is a syntax error in the melody, not a call.';
RELATE code:wp_forgiving_play->extracted_from->source:s_wiki_hypertalk;
RELATE code:wp_forgiving_play->uses->keyword:cmd_play;
RELATE code:wp_forgiving_play->demonstrates->concept:forgiving_semantics SET confidence='high';
RELATE code:wp_forgiving_play->demonstrates->concept:natural_language_syntax;

-- == From Tricks of the HyperTalk Masters (Jeanne A. E. DeVoto, 1992) =======
UPSERT code:jaed_anim SET
  title = "Custom message to a central handler", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "The DRY move of HyperCard: buttons send a custom message; the stack script owns the one real handler.",
  script = 'on animateButtons
  -- your commands to animate the buttons go here
end animateButtons

on mouseUp
  animateButtons
end mouseUp',
  notes = 'A bare handler name is a message send; it walks the path to the stack. Procedures were messages all along.';
RELATE code:jaed_anim->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_anim->uses->keyword:ctl_on;
RELATE code:jaed_anim->uses->keyword:msg_mouseup;
RELATE code:jaed_anim->demonstrates->concept:message_path SET note='custom messages ride the same path';

UPSERT code:jaed_lockall SET
  title = "lockEverything / unlockEverything", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "Bundle three global properties into two custom messages - the stack grows its own vocabulary.",
  script = 'on lockEverything
  set the lockMessages to true
  set the lockScreen to true
  set the lockRecent to true
end lockEverything

on unlockEverything
  set the lockMessages to false
  set the lockScreen to false
  set the lockRecent to false
end unlockEverything';
RELATE code:jaed_lockall->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_lockall->uses->keyword:prop_lockmessages;
RELATE code:jaed_lockall->uses->keyword:prop_lockscreen;
RELATE code:jaed_lockall->uses->keyword:prop_lockrecent;
RELATE code:jaed_lockall->uses->keyword:cmd_set;
RELATE code:jaed_lockall->demonstrates->concept:property_model;

UPSERT code:jaed_globals_timer SET
  title = "Global-variable play timer", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "Globals + openStack + idle: state that survives handlers, checked every time the queue empties.",
  script = 'on openStack
  global timeToPlay, startTime
  ask "How many minutes do you want to play?"
  put it * 60 into timeToPlay  -- convert it into seconds
  put the time into startTime  -- keep a record of the current time
end openStack

on idle
  global timeToPlay, startTime
  if startTime + timeToPlay > the time then
    beep
    answer "Time\'s up!"
    quit
  end if
end idle',
  notes = 'Every idiom of background behavior in one pair of handlers: declare globals per-handler, seed them in openStack, poll in idle.';
RELATE code:jaed_globals_timer->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_globals_timer->uses->keyword:ctl_global;
RELATE code:jaed_globals_timer->uses->keyword:msg_openstack;
RELATE code:jaed_globals_timer->uses->keyword:msg_idle;
RELATE code:jaed_globals_timer->uses->keyword:fn_datetime;
RELATE code:jaed_globals_timer->demonstrates->concept:idle_time SET confidence='high';

UPSERT code:jaed_target_dispatch SET
  title = "Stack-level dispatcher via the target", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "One mouseUp handler serves every link button: ask the target what was clicked, pass if it was not a button.",
  script = 'on mouseUp
  if word 2 of the name of the target is "button" then
    visual effect zoom open
    go to card the short name of the target
  else
    pass mouseUp
  end if
end mouseUp',
  notes = 'The name of a button is its target card; the whole link system of many stacks is this handler plus naming discipline.';
RELATE code:jaed_target_dispatch->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_target_dispatch->uses->keyword:sp_target;
RELATE code:jaed_target_dispatch->uses->keyword:prop_name;
RELATE code:jaed_target_dispatch->uses->keyword:chk_word;
RELATE code:jaed_target_dispatch->uses->keyword:ctl_pass;
RELATE code:jaed_target_dispatch->uses->keyword:cmd_go;
RELATE code:jaed_target_dispatch->demonstrates->concept:target_me_it SET confidence='high';
RELATE code:jaed_target_dispatch->demonstrates->concept:message_path;

UPSERT code:jaed_selection SET
  title = "Double-click a word into a variable", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "Synthesize a double-click with click at the clickLoc, then harvest the selection - glossary links from plain text.",
  script = 'on mouseDown
  set the lockScreen to true
  set the lockText of the target to false
  click at the clickLoc     -- double-click to select a word
  click at the clickLoc     --
  put the selection into myWord
  set the lockText of the target to true
  set the lockScreen to false
  -- [you can now do whatever you want with the selected word]
end mouseDown',
  notes = 'Scripting the mouse, scripting the selection, and restoring the property you borrowed - the etiquette of every interception handler.';
RELATE code:jaed_selection->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_selection->uses->keyword:cmd_clickdrag;
RELATE code:jaed_selection->uses->keyword:sp_selection;
RELATE code:jaed_selection->uses->keyword:prop_locktext;
RELATE code:jaed_selection->uses->keyword:fn_click;
RELATE code:jaed_selection->demonstrates->concept:container_model SET note='the selection is a container';

UPSERT code:jaed_findresult SET
  title = "find guarded by the result", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com; one comment added by the corpus",
  summary = "The result property as the error channel: empty means the find landed, non-empty means apologize.",
  script = 'on mouseUp
  ask "Which title do you want to find?"
  if it is not empty then findTitle it
end mouseUp

on findTitle theTitle
  global searchList
  find theTitle in background field "Book Title"
  if the result is not empty then  -- means the find failed
    beep
    answer "Sorry, that title is not in this list."
    exit findTitle
  end if
  put return & background field "Title" after searchList
end findTitle',
  notes = 'Errors are not exceptions: they wait in the result property for a handler polite enough to check.';
RELATE code:jaed_findresult->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_findresult->uses->keyword:cmd_find;
RELATE code:jaed_findresult->uses->keyword:fn_result;
RELATE code:jaed_findresult->uses->keyword:op_concat;
RELATE code:jaed_findresult->uses->keyword:con_whitespace;
RELATE code:jaed_findresult->demonstrates->concept:find_mark_sort SET confidence='high';

UPSERT code:jaed_goboard SET
  title = "Tic-tac-toe board as a line/item array", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "A 2-D array is a string with returns and commas: item horiz of line vertical of gameBoard.",
  script = 'on startGame
  global gameBoard
  repeat with vertical = 1 to 3
    repeat with horiz = 1 to 3
      put empty into item horiz of line vertical of gameBoard
    end repeat
  end repeat
end startGame',
  notes = 'Chunk expressions are lvalues all the way down - assignment writes into the middle of a string.';
RELATE code:jaed_goboard->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_goboard->uses->keyword:chk_item;
RELATE code:jaed_goboard->uses->keyword:chk_line;
RELATE code:jaed_goboard->uses->keyword:con_empty;
RELATE code:jaed_goboard->demonstrates->concept:pseudo_arrays SET confidence='high';

UPSERT code:jaed_domenu_trap SET
  title = "doMenu trap with pass", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "Intercept four navigation menu items, pass everything else - the pass line is what keeps the stack alive.",
  script = 'on doMenu theItem
  if theItem is "First" or theItem is "Last" or theItem is "Prev" or theItem is "Next" then
    beep
  else
    pass doMenu     ----------- VERY IMPORTANT! -------------
  end if
end doMenu',
  notes = 'Omit pass doMenu and every menu item dies, including the ones that would let you escape - the classic self-lockout story.';
RELATE code:jaed_domenu_trap->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_domenu_trap->uses->keyword:msg_domenu;
RELATE code:jaed_domenu_trap->uses->keyword:ctl_pass;
RELATE code:jaed_domenu_trap->demonstrates->concept:message_path SET note='commands are messages, so traps are handlers';

UPSERT code:jaed_visual_send SET
  title = "Rewriting visual effects without recursion", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com; two comments added by the corpus",
  summary = "Trap the visual message but send the replacement directly to HyperCard, so your own trap cannot catch it.",
  script = '-- wrong: recurses forever
on visual
  visual effect dissolve
end visual

-- right: skip past the stack level
on visual
  send "visual effect dissolve" to HyperCard
end visual',
  notes = 'The message path is a chain of responsibility - send-to-<level> is how you jump the queue.';
RELATE code:jaed_visual_send->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_visual_send->uses->keyword:cmd_visual;
RELATE code:jaed_visual_send->uses->keyword:cmd_send;
RELATE code:jaed_visual_send->demonstrates->concept:message_path SET confidence='high';

UPSERT code:jaed_checkcode SET
  title = "Recursive input validation", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "A handler that calls itself until the field is well-formed - recursion as a validation loop.",
  script = 'on checkCode
  if the number of chars in field "Customer Code" is not 7 then
    ask "Please type in the 7-letter customer code"
    put it into field "Customer Code"
    checkCode
  end if
end checkCode',
  notes = 'HyperTalk caps pending handlers at roughly ten to twenty levels - recursion was always a slightly dangerous idiom.';
RELATE code:jaed_checkcode->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_checkcode->uses->keyword:fn_numberof;
RELATE code:jaed_checkcode->uses->keyword:chk_character;
RELATE code:jaed_checkcode->uses->keyword:cmd_ask;
RELATE code:jaed_checkcode->demonstrates->concept:message_path SET note='a handler name is a message - including its own';

UPSERT code:jaed_makealist SET
  title = "makeAList: stack inventory", kind = 'handler', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com (trimmed)",
  summary = "The stack introspects itself: walk every background and card, append every part name to one string.",
  script = 'on makeAList
  lockEverything  --- see handler in section on message handlers
  put the name of this stack into theObjectList
  ----- first get all the background object names -----
  repeat with b = 1 to the number of backgrounds
    go background b
    put return & the name of this background after theObjectList
    repeat with x = 1 to the number of background fields
      put return & the name of background field x after theObjectList
    end repeat
    repeat with x = 1 to the number of background buttons
      put return & the name of background button x after theObjectList
    end repeat
  end repeat
  ------ now get all the card object names ------
  repeat with b = 1 to the number of cards
    go card b
    put return & the name of this card after theObjectList
    repeat with x = 1 to the number of card fields
      put return & the name of card field x after theObjectList
    end repeat
    repeat with x = 1 to the number of card buttons
      put return & the name of card button x after theObjectList
    end repeat
  end repeat
  put theObjectList into card field "list" of card 1
  unlockEverything
end makeAList',
  notes = 'Program-as-data made tooling like this a one-screen script: the stack can enumerate and print itself.';
RELATE code:jaed_makealist->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_makealist->uses->keyword:fn_numberof;
RELATE code:jaed_makealist->uses->keyword:prop_name;
RELATE code:jaed_makealist->uses->keyword:obj_part;
RELATE code:jaed_makealist->uses->keyword:op_concat;
RELATE code:jaed_makealist->demonstrates->concept:script_property SET note='introspection of the stack as data';
RELATE code:jaed_makealist->demonstrates->concept:q_authoring_runtime_unity SET note='authoring tools written in the language at runtime';

UPSERT code:jaed_version SET
  title = "Version guard in openStack", kind = 'pattern', year = 1992,
  attribution = "Jeanne A. E. DeVoto, Tricks of the HyperTalk Masters (1992), via jaedworks.com",
  summary = "Distribution-aware HyperCard: check the version before the stack does anything version-specific.",
  script = 'on openStack
  if the version < 1.2 then
    answer "This stack requires HyperCard 1.2 or later."
    doMenu "Home"
    exit openStack
  end if
  -- [the rest of the commands for openStack]
end openStack';
RELATE code:jaed_version->extracted_from->source:s_jaed_tricks;
RELATE code:jaed_version->uses->keyword:fn_version;
RELATE code:jaed_version->uses->keyword:cmd_domenu;
RELATE code:jaed_version->uses->keyword:msg_openstack;
RELATE code:jaed_version->demonstrates->concept:stackware SET note='stacks shipped to unknown machines';

-- == From Myst (1993), extracted by Andrew Plotkin (erkyrath/mystextract) ==
UPSERT code:myst_pillars SET
  title = "Myst: the pillar puzzle (state in an item list)", kind = 'handler', year = 1993,
  attribution = "Cyan Inc., Myst (1993); script dump via Andrew Plotkin, erkyrath/mystextract",
  summary = "A shipped game puzzle: eight toggles in a comma-delimited global, a winning pattern compared as a string.",
  script = 'on PillarClick theID
  global  MY_Pillars,MY_boat
  play stop
  play "SW Pushbutton Out"
  if MY_Pillars is empty then put "off,off,off,off,off,off,off,off" into MY_Pillars
  get item theID of MY_Pillars
  if it is "on" then put "off" into item theID of MY_Pillars
  else put "on" into item theID of MY_Pillars
  showPillar theID
  if MY_Pillars is "off,on,off,off,on,on,off,off" then
    put "up" into MY_boat
    play stop
    hc
    play "BR ShipUp"
    repeat until the sound is done
      soundIdle
    end repeat
  else
    if MY_boat is "up" then
      put "down" into MY_boat
      play stop
      hc
      play "BR ShipUp"
      repeat until the sound is done
        soundIdle
      end repeat
    end if
  end if
  ww
end PillarClick',
  notes = 'The game state of a 1993 bestseller is a string of on/off items; the win condition is one string equality. hc, ww, soundIdle and xCIcon3 are XCMDs.';
RELATE code:myst_pillars->extracted_from->source:s_mystextract;
RELATE code:myst_pillars->uses->keyword:chk_item;
RELATE code:myst_pillars->uses->keyword:con_empty;
RELATE code:myst_pillars->uses->keyword:ctl_global;
RELATE code:myst_pillars->uses->keyword:cmd_play;
RELATE code:myst_pillars->uses->keyword:ctl_repeat;
RELATE code:myst_pillars->demonstrates->concept:pseudo_arrays SET confidence='high';
RELATE code:myst_pillars->demonstrates->concept:xcmd SET note='soundIdle/hc/ww are XCMDs';

UPSERT code:myst_mech_openstack SET
  title = "Myst Mechanical Age: openStack as boot sequence", kind = 'handler', year = 1993,
  attribution = "Cyan Inc., Myst (1993); script dump via Andrew Plotkin, erkyrath/mystextract",
  summary = "A AAA title booting inside HyperCard: paths, palettes, color XCMDs, library stacks, cursor takeover.",
  script = 'on openStack
  global All_CurrStack,S_SCREENPOS,documents
  -- The documents paths are for MooVs - always on the CD
  put "Myst:Myst Graphics:Mechanical:" into documents
  SetScreen
  BlackFrame
  xMemory(1)
  HTUDefPal 6001
  Hypertint "later","delay","iRes5","NoTEOpt"
  htlock novbl
  htlock nobw
  htlock nocolormap
  start using stack "MERes1"
  start using stack "MERes2"
  DeCurse "install"
  deCurse "override","hand","color","nodelay"
  initGlobals
  put "Mechanical Age" into All_CurrStack
  end openStack',
  notes = 'Hypertint/htlock are the color XCMDs; DeCurse replaces the cursor; start using splices the resource stacks into the message path.';
RELATE code:myst_mech_openstack->extracted_from->source:s_mystextract;
RELATE code:myst_mech_openstack->uses->keyword:msg_openstack;
RELATE code:myst_mech_openstack->uses->keyword:cmd_startusing;
RELATE code:myst_mech_openstack->uses->keyword:ctl_global;
RELATE code:myst_mech_openstack->demonstrates->concept:xcmd SET confidence='high';
RELATE code:myst_mech_openstack->demonstrates->concept:bolted_on_color SET note='Hypertint is the color workaround in situ';

UPSERT code:myst_closemoovs SET
  title = "Myst: close every movie window", kind = 'handler', year = 1993,
  attribution = "Cyan Inc., Myst (1993); script dump via Andrew Plotkin, erkyrath/mystextract",
  summary = "Iterate the windows property backwards, substring-match names, close the QuickTime ones.",
  script = 'on closemoovs
  repeat with x = (the number of lines in the windows) down to 1
    if line x of the windows contains "moov" then close window (line x of the windows)
    else if line x of the windows contains "mov" then close window (line x of the windows)
  end repeat
end closemoovs',
  notes = 'Down-to iteration because closing shifts the line numbers; the windows property is just text to be chunked and matched.';
RELATE code:myst_closemoovs->extracted_from->source:s_mystextract;
RELATE code:myst_closemoovs->uses->keyword:fn_env;
RELATE code:myst_closemoovs->uses->keyword:op_verbal;
RELATE code:myst_closemoovs->uses->keyword:chk_line;
RELATE code:myst_closemoovs->uses->keyword:obj_window;
RELATE code:myst_closemoovs->demonstrates->concept:chunk_expression SET note='the OS environment as chunkable text';

UPSERT code:myst_blackframe SET
  title = "Myst: idempotent window setup (BlackFrame)", kind = 'handler', year = 1993,
  attribution = "Cyan Inc., Myst (1993); script dump via Andrew Plotkin, erkyrath/mystextract",
  summary = "there is not a window as a guard: create the frame only if it is missing, then hide the chrome.",
  script = 'on BlackFrame
  if there is not a window "frame" then
    unlock screen
    xWindowFrame
    if there is a window "frame" then
      doMenu "next window"
      hide menubar
      hide titlebar
    end if
  end if
end BlackFrame',
  notes = 'Existence predicates around an XCMD side effect - defensive HyperTalk in shipped software.';
RELATE code:myst_blackframe->extracted_from->source:s_mystextract;
RELATE code:myst_blackframe->uses->keyword:op_thereis;
RELATE code:myst_blackframe->uses->keyword:obj_window;
RELATE code:myst_blackframe->uses->keyword:cmd_domenu;
RELATE code:myst_blackframe->demonstrates->concept:xcmd;
