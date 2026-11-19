-- HyperTalk language reference: one node per keyword / family of keywords.
-- Grounded in the Apple HyperCard Script Language Guide (1996, HyperCard 2.x),
-- the grammars-v4 HyperTalk grammar, Wikipedia, and Tricks of the HyperTalk Masters.
-- `since` marks the HyperCard version that introduced the item where known.

-- == COMMANDS ===============================================================
UPSERT keyword:cmd_add SET word='add', category='command', aliases=['subtract','multiply','divide'], summary='Arithmetic on containers: add X to Y, subtract X from Y, multiply/divide Y by X; coerces operands to numbers and writes the result back.';
UPSERT keyword:cmd_answer SET word='answer', category='command', summary='Modal dialog with up to three buttons: answer "msg" with "A" or "B" or "C"; the clicked button name lands in it, and the dialog is also a message (on answer).';
UPSERT keyword:cmd_ask SET word='ask', category='command', aliases=['ask file'], summary='Dialog with an editable text field; the typed reply goes into it. ask file prompts for a file path instead.';
UPSERT keyword:cmd_choose SET word='choose', category='command', summary='Selects a HyperCard tool programmatically: choose browse tool, choose field tool; the tool is itself a container property.';
UPSERT keyword:cmd_clickdrag SET word='click', category='command', aliases=['drag','type'], summary='Input synthesis: click at <point> [with <modifiers>], drag from <p1> to <p2>, type "text" [with commandkey]; the scripting-the-user idiom behind most automated demos.';
UPSERT keyword:cmd_convert SET word='convert', category='command', summary='Date/number arithmetic in place: convert <container> to seconds, convert <container> from dateItems to date; mutates the container it is given.';
UPSERT keyword:cmd_create SET word='create', category='command', aliases=['delete'], summary='create menu / create menuitem builds custom menus; delete removes cards, fields, buttons, menus, or marked records.';
UPSERT keyword:cmd_do SET word='do', category='command', summary='Executes a string as HyperTalk: do line 3 of cd fld 1. The runtime escape hatch for generated code; the same evaluator the message box uses.';
UPSERT keyword:cmd_domenu SET word='doMenu', category='command', summary='Invokes a menu item by name: doMenu "New Card". Sends the doMenu message, so stacks can trap or extend every menu command.';
UPSERT keyword:cmd_edit SET word='edit the script', category='command', summary='Opens the script editor on a container: edit the script of this stack. The language can open its own source - authoring and runtime share one surface.';
UPSERT keyword:cmd_enable SET word='enable', category='command', aliases=['disable'], summary='enable/disable menu items or menu bars; shorthand for setting the enabled property.';
UPSERT keyword:cmd_exit SET word='exit', category='command', summary='exit <handlerName> leaves a handler early; exit to hypercard abandons the current script entirely (the emergency brake for runaway handlers).';
UPSERT keyword:cmd_find SET word='find', category='command', aliases=['find chars','find whole','find string'], summary='Stack-wide full-text search over card/background fields: find "word"; marks the found chunk, sets the result on failure, and pairs with mark/find-next idioms.';
UPSERT keyword:cmd_get SET word='get', category='command', summary='Evaluates an expression and stores it in the implicit variable it: get the loc of the target. The one-assignment-per-line workhorse before named variables.';
UPSERT keyword:cmd_go SET word='go', category='command', aliases=['go to'], summary='Navigation: go to card "X", go next, go last, go stack "Y" [in a new window]. Sends the go message, which stacks can trap.';
UPSERT keyword:cmd_hide SET word='hide', category='command', aliases=['show'], summary='hide/show fields, buttons, pictures, titlebar, menubar; syntactic sugar over the visible property (hide fld i == set the visible of fld i to false).';
UPSERT keyword:cmd_lockscreen SET word='lock screen', category='command', aliases=['unlock screen'], summary='Suspends redraw so multi-step updates appear atomically: lock screen ... unlock screen [with visual effect]. Sugar for the lockScreen property.';
UPSERT keyword:cmd_fileio SET word='open file', category='command', aliases=['close file','read','write'], summary='Flat-file I/O: open file, read from file until/for N, write to file at eof; data arrives in it and leaves as text.';
UPSERT keyword:cmd_play SET word='play', category='command', aliases=['beep','speak'], summary='Sound: play "res name", play harpsichord c e g (notes as natural-language args), beep N, speak "text" [with voice]; the sound is done tests completion.';
UPSERT keyword:cmd_pushpop SET word='push', category='command', aliases=['pop'], summary='push card / pop card maintain a navigation stack in the HyperCard history, the go-back primitive before browsers made it universal.';
UPSERT keyword:cmd_put SET word='put', category='command', summary='The assignment command, predicate-last: put 5 * 4 into x; also put ... before/after <chunk>. Creates locals on first use and coerces freely.';
UPSERT keyword:cmd_select SET word='select', category='command', summary='Moves the text selection: select the clickLine, select line 2 of fld "x", select before/after text; the selection is then readable as a container.';
UPSERT keyword:cmd_send SET word='send', category='command', summary='Sends a message string down a chosen path: send "mouseUp" to card button "OK" of card "Veracity", send "visual effect dissolve" to HyperCard (bypassing local traps).';
UPSERT keyword:cmd_set SET word='set', category='command', summary='set the <property> of <container> to <value>; the generic mutator, and the long form that show/hide, enable/disable and lock screen abbreviate.';
UPSERT keyword:cmd_show SET word='show', category='command', summary='See hide; also show card field "x" as the popup idiom paired with wait until the mouseClick.';
UPSERT keyword:cmd_sort SET word='sort', category='command', summary='sort cards [of stack] by <field> [ascending/descending] [text/numeric/datetime/international]; sorts the stack itself, reordering cards - the database leaking into the language.';
UPSERT keyword:cmd_startusing SET word='start using', category='command', aliases=['stop using'], summary='start using stack "Lib" splices another stack into the message path after this one - the library mechanism of HyperCard 2.x.';
UPSERT keyword:cmd_visual SET word='visual effect', category='command', summary='Screen transitions for navigation: visual effect iris open, wipe left, zoom in; every use sends a visual message that scripts can intercept.';
UPSERT keyword:cmd_wait SET word='wait', category='command', summary='wait 10 ticks / wait until <expr> / wait while <expr>; the cooperative pause that keeps idle and mouse messages alive.';

-- == FUNCTIONS =============================================================
UPSERT keyword:fn_value SET word='value()', category='function', aliases=['the value of'], summary='Evaluates a string as a HyperTalk expression and returns the result: value("2+2") is 4. The string-to-code door; do is its statement-shaped twin.';
UPSERT keyword:fn_length SET word='length', category='function', summary='Character count of a container or chunk: the length of fld "x", length(t).';
UPSERT keyword:fn_offset SET word='offset', category='function', summary='offset(pattern, string) returns the 1-based character position of the first occurrence, 0 if absent; the search primitive paired with is in.';
UPSERT keyword:fn_numberof SET word='number of', category='function', summary='the number of cards / buttons / lines of t / marked cards; the count function that drives every repeat-with loop over collections.';
UPSERT keyword:fn_random SET word='random', category='function', summary='random(n) returns an integer from 1 to n; the dice of HyperCard games.';
UPSERT keyword:fn_math SET word='sqrt', category='function', aliases=['trunc','abs','min','max','sum','average','sin','cos','tan','atan','exp','exp1','exp2','ln','ln1','log2'], summary='The math library: trunc/abs, min/max/sum/average over item-delimited lists, trig, exponentials and logs; numbers are 32-bit with SANE semantics.';
UPSERT keyword:fn_charnum SET word='chartonum', category='function', aliases=['numtochar'], summary='chartonum(c) / numtochar(n) cross between a one-character string and its Mac Roman code - the byte-level door.';
UPSERT keyword:fn_finance SET word='annuity', category='function', aliases=['compound'], summary='annuity(payment,rate,periods) and compound(amount,rate,periods): the financial functions shipped in the base language.';
UPSERT keyword:fn_params SET word='param', category='function', aliases=['paramcount','params'], summary='param(n), paramcount, params: positional access to a handler or function arguments; how generic handlers read their call site.';
UPSERT keyword:fn_datetime SET word='the date', category='function', aliases=['the time','the ticks','the seconds'], summary='Date/time reads: the date, the time, the ticks (60ths since launch), the seconds (since 1904); convert does the arithmetic.';
UPSERT keyword:fn_mouse SET word='the mouse', category='function', aliases=['the mouseLoc','the mouseClick'], summary='Polling reads: the mouse is down/up, the mouseLoc is a point, the mouseClick flips true once per click - the busy-wait inputs for games.';
UPSERT keyword:fn_keys SET word='the commandKey', category='function', aliases=['the shiftKey','the optionKey','the commandkeys'], summary='Modifier state: the commandKey is down; 2.x added the commandkeys array-like string of all Command-key equivalents.';
UPSERT keyword:fn_result SET word='the result', category='function', summary='Error channel: empty after a successful go/find/open, an error string otherwise; the idiomatic existence test for cards and files.';
UPSERT keyword:fn_tool SET word='the tool', category='function', summary='Reads the current tool: the tool is browse tool; pairs with choose.';
UPSERT keyword:fn_version SET word='the version', category='function', summary='HyperCard version string; openStack guards read: if the version < 2.0 then answer "needs 2.0".';
UPSERT keyword:fn_env SET word='the stacks', category='function', aliases=['the windows','the menus','the diskspace','the voices','the speech'], summary='Environment listings as line-delimited text: the stacks, the windows, the menus, the diskspace, the voices; iterate them with the number of lines of the windows.';
UPSERT keyword:fn_click SET word='the clickLoc', category='function', aliases=['the clickLine','the clickChunk','the clickText'], summary='What the last click hit: clickLoc (point), clickLine ("line 10 of card field 4"), clickChunk, clickText; clickLine + do built countless in-card interpreters.';
UPSERT keyword:fn_selected SET word='the selectedText', category='function', aliases=['the selectedLine','the selectedChunk','the selectedField'], summary='Selection reads: selectedText, selectedLine, selectedChunk (a container description you can re-select), selectedField.';

-- == CONTROL STRUCTURES ====================================================
UPSERT keyword:ctl_if SET word='if/then/else', category='control', summary='if <expr> then <stmt> | if ... then <block> else <block> end if; single-line and block forms, no case/switch in the base language.';
UPSERT keyword:ctl_repeat SET word='repeat', category='control', summary='One keyword, five shapes: repeat, repeat while/until, repeat for N [times], repeat with i = a to b [down to], each closed by end repeat.';
UPSERT keyword:ctl_exit SET word='exit <handler>', category='control', summary='Early return from a message handler: exit mouseUp. Named after the handler it leaves.';
UPSERT keyword:ctl_exitrepeat SET word='exit repeat', category='control', summary='Breaks out of the innermost repeat loop.';
UPSERT keyword:ctl_nextrepeat SET word='next repeat', category='control', summary='Skips to the next iteration - the continue of HyperTalk.';
UPSERT keyword:ctl_pass SET word='pass', category='control', summary='pass <message> re-sends the message up the path untouched - the safety valve of every doMenu/mouseUp trap; omitting it swallows the system.';
UPSERT keyword:ctl_return SET word='return', category='control', summary='Returns a value from a function handler (or bare return to return empty).';
UPSERT keyword:ctl_global SET word='global', category='control', summary='global a, b declares handler-wide globals that outlive the handler and cross stacks until quit; the message box treats every name as global.';
UPSERT keyword:ctl_on SET word='on/end', category='control', summary='on <message> [params] ... end <message> defines a message handler; the handler name must be repeated at the end - a bookend grammar that made scripts self-describing.';
UPSERT keyword:ctl_function SET word='function/end', category='control', summary='function <name> [params] ... end <name> defines a value-returning handler; user functions are called with parentheses, never with the.';
UPSERT keyword:ctl_comment SET word='--', category='control', summary='Comment to end of line; two minus signs, borrowed from the natural-language look of the language.';
UPSERT keyword:ctl_continue SET word='|', category='control', aliases=['¬'], summary='Line continuation: | or the option-return character (logical line vs physical line); long chunk expressions wrap with it.';

-- == CHUNK KEYWORDS ========================================================
UPSERT keyword:chk_character SET word='character', category='chunk', aliases=['char','chars'], summary='Character chunk: character 3 of t, the first char of t; the finest-grained text container.';
UPSERT keyword:chk_word SET word='word', category='chunk', summary='Whitespace-delimited token: the third word of theFilesText; the tokenizer that never needed writing.';
UPSERT keyword:chk_item SET word='item', category='chunk', summary='Delimiter-based token: item 2 of pos; delimiter is the itemDelimiter property (default comma), making CSV parsing a keyword.';
UPSERT keyword:chk_line SET word='line', category='chunk', summary='Return-delimited record: line 5 of card field "sometext"; the row abstraction of the field-as-database idiom.';
UPSERT keyword:chk_ordinal SET word='first', category='chunk', aliases=['second','third','fourth','fifth','sixth','seventh','eighth','ninth','tenth','middle','last','any'], summary='English ordinals as indexes: the last line of t, the middle word of t, the any card; zero-based counting nowhere to be found.';
UPSERT keyword:chk_range SET word='X to Y', category='chunk', summary='Chunk ranges: character 1 to 10 of fld "x", line 2 to 4 of t; ranges are assignable containers, which is how replaceStr rewrites text in place.';

-- == CONTAINER KEYWORDS ====================================================
UPSERT keyword:obj_card SET word='card', category='container', aliases=['cd','cards'], summary='A single screen of records; addressable as card "name", card id 12, the second card, this card, cd fld 1.';
UPSERT keyword:obj_background SET word='background', category='container', aliases=['bkgnd','bg','bkgnds'], summary='The shared chrome behind cards; bg fld "pict name" is how Myst addressed its whole screen buffer.';
UPSERT keyword:obj_button SET word='button', category='container', aliases=['btn','buttons','btns'], summary='A clickable region with a script; style rectangle/default/transparent/opaque/checkbox/radio.';
UPSERT keyword:obj_field SET word='field', category='container', aliases=['fld','fields','flds'], summary='Text region: scrolling or not, locked or not; the primary data container of the object model and of chunk expressions.';
UPSERT keyword:obj_stack SET word='stack', category='container', aliases=['this stack'], summary='The file-level container of cards, backgrounds, parts and global script; also a message-path level.';
UPSERT keyword:obj_this SET word='this', category='container', summary='Context anchor: this card, this stack, this background - resolved against where the running handler lives.';
UPSERT keyword:obj_part SET word='part', category='container', summary='Any button or field: the parts of this card, card part "OK"; 2.2 unified card and background parts under one collection.';
UPSERT keyword:obj_window SET word='window', category='container', summary='OS windows as containers: the window "Paint", the card window, the message watcher; close window / the number of windows.';
UPSERT keyword:obj_menu SET word='menu', category='container', aliases=['menuitem'], summary='Menus and their items as containers: create menu, set the enabled of menuitem 3 of menu "Edit" to false.';
UPSERT keyword:obj_id SET word='id', category='container', summary='Stable numeric identity: card id 12, button id -3; the addressing mode that survives renames, preferred by generated code.';
UPSERT keyword:sp_me SET word='me', category='special', summary='The container whose script is running - self for the message path; put line 1 of me reads the field the handler lives in.';
UPSERT keyword:sp_it SET word='it', category='special', summary='The implicit variable: filled by answer, ask, get, read; the reason HyperTalk needs no temporary declarations in simple scripts.';
UPSERT keyword:sp_target SET word='the target', category='special', summary='The object that first received the current message; word 2 of the name of the target tells a stack-level handler what was clicked.';
UPSERT keyword:sp_selection SET word='the selection', category='special', summary='The current text selection as a container: put the selection into myWord; select replaces it. Clipboard-free interaction with the user.';

-- == OPERATORS =============================================================
UPSERT keyword:op_arith SET word='+ -', category='operator', aliases=['* / div mod'], summary='Numeric operators; operands are coerced from text, and div/mod are the integer pair. Precedence: * / div mod above + -.';
UPSERT keyword:op_power SET word='^', category='operator', summary='Exponentiation; binds tighter than * and /.';
UPSERT keyword:op_concat SET word='&', category='operator', aliases=['&&'], summary='& concatenates exactly; && inserts a space between operands - the sentence-builder of natural-language output.';
UPSERT keyword:op_compare SET word='=', category='operator', aliases=['is','is not','<>','<','>','<=','>='], summary='Comparison; numeric when both sides are numbers, text otherwise. is/is not are the English spellings of = / <>.';
UPSERT keyword:op_verbal SET word='contains', category='operator', aliases=['is in','is not in','is within','is not within'], summary='Verbal containment: t contains "x", "a" is in t, point is within rect; predicates that read as sentences and drive repeat-while find loops.';
UPSERT keyword:op_typetest SET word='is a', category='operator', aliases=['is an','is not a','is not an'], summary='Type predicates: x is a number / integer / point / rect / date / logical / boolean - the introspection half of the coercion model.';
UPSERT keyword:op_thereis SET word='there is a', category='operator', aliases=['there is an','there is no','there is not a'], summary='Existence predicates: if there is a file filePath, if there is not a window "frame" - the guard clause of defensive HyperTalk.';
UPSERT keyword:op_logic SET word='and', category='operator', aliases=['or','not'], summary='Boolean operators, lowest precedence of all; note and/or do not short-circuit - both sides always evaluate.';
UPSERT keyword:op_precedence SET word='operator precedence', category='operator', summary='Distinctive ladder (guide appendix E): grouping; unary minus/not/there-is; ^; * / div mod; + -; & &&; comparisons; verbal operators; = is is not; and; or. Concatenation binds tighter than comparison, and/or bind loosest.';

-- == CONSTANTS =============================================================
UPSERT keyword:con_empty SET word='empty', category='constant', summary='The empty value: put empty into x clears a container; if MY_Pillars is empty tests first-run state - Myst used it as its uninitialized sentinel.';
UPSERT keyword:con_pi SET word='pi', category='constant', summary='The circle constant, spelled out.';
UPSERT keyword:con_whitespace SET word='space', category='constant', aliases=['return','tab','quote','comma','colon','formfeed','linefeed'], summary='Named characters: return joins lines, space and tab pad, quote quotes - string building without escape sequences.';
UPSERT keyword:con_numbers SET word='zero', category='constant', aliases=['one','two','three','four','five','six','seven','eight','nine','ten'], summary='Number words zero through ten usable as literals.';
UPSERT keyword:con_booleans SET word='true', category='constant', aliases=['false','yes','no','on','off'], summary='Logical literals; yes/no/on/off are accepted synonyms, another concession to the natural-language surface.';
UPSERT keyword:con_mousestate SET word='down', category='constant', aliases=['up'], summary='Mouse/modifier state words: the mouse is down, the commandKey is down.';

-- == SYSTEM MESSAGES =======================================================
UPSERT keyword:msg_startup SET word='startup', category='message', summary='Sent once when HyperCard launches, first caught by the Home stack - the system-wide autoexec.';
UPSERT keyword:msg_quit SET word='quit', category='message', summary='Asks permission to leave; a handler can beep and exit quit to veto.';
UPSERT keyword:msg_suspend SET word='suspend', category='message', aliases=['resume'], summary='HyperCard backgrounded / reactivated; games pause their sound and timers here.';
UPSERT keyword:msg_appswitch SET word='appSwitch', category='message', summary='Another application came forward (2.x); the multi-tasking edge of the message path.';
UPSERT keyword:msg_idle SET word='idle', category='message', summary='Sent when the event queue empties - the heartbeat for animation and background work; the only message that arrives without user action.';
UPSERT keyword:msg_mousedown SET word='mouseDown', category='message', summary='Button pressed inside a part/card; sent to locked fields too, which is why locked fields can be buttons.';
UPSERT keyword:msg_mouseup SET word='mouseUp', category='message', summary='Released inside the same rectangle - the canonical HyperTalk entry point, on mouseUp ... end mouseUp.';
UPSERT keyword:msg_mousestillsdown SET word='mouseStillDown', category='message', summary='Repeated while the button is held - drag-style interaction without the drag command.';
UPSERT keyword:msg_mouseenter SET word='mouseEnter', category='message', aliases=['mouseLeave'], summary='Pointer crosses a part boundary in either direction; hover behavior, 1987 edition.';
UPSERT keyword:msg_mousewithin SET word='mouseWithin', category='message', summary='Repeated while the pointer sits inside a part - per-frame hover.';
UPSERT keyword:msg_mousedoubleclick SET word='mouseDoubleClick', category='message', summary='Second click within the double-click interval and 4 pixels; when sent it suppresses the ordinary down/up pair.';
UPSERT keyword:msg_keydown SET word='keyDown', category='message', summary='A key with no Command modifier; the char is in the message box/keystroke property.';
UPSERT keyword:msg_keypressed SET word='keyPressed', category='message', summary='Non-character keys (arrows, function keys as raw presses, 2.x).';
UPSERT keyword:msg_functionkey SET word='functionKey', category='message', summary='F1-F10 pressed; the Home stack maps them to tools.';
UPSERT keyword:msg_commandkeydown SET word='commandKeyDown', category='message', aliases=['commandKeyUp'], summary='Command-key equivalents as messages: commandKeyDown "H" - interceptable, extendable keyboard shortcuts.';
UPSERT keyword:msg_returnkey SET word='returnKey', category='message', aliases=['enterKey','tabKey'], summary='Editing keys sent to fields/cards: returnKey, enterKey, tabKey; autoTab rides on tabKey.';
UPSERT keyword:msg_enterinfield SET word='enterInField', category='message', aliases=['returnInField'], summary='Enter/Return pressed with the insertion point in a field; unhandled enterInField becomes closeField when text changed.';
UPSERT keyword:msg_domenu SET word='doMenu', category='message', summary='Wraps every menu pick and doMenu call; the classic trap: on doMenu theItem ... pass doMenu.';
UPSERT keyword:msg_go SET word='go', category='message', summary='Wraps navigation; trapping go lets a stack rewrite or veto its own links.';
UPSERT keyword:msg_openstack SET word='openStack', category='message', aliases=['closeStack'], summary='Stack lifecycle; openStack is the constructor of every stack (init globals, choose browse tool, set userLevel).';
UPSERT keyword:msg_suspendstack SET word='suspendStack', category='message', aliases=['resumeStack'], summary='The stack stopped/started being the active one (2.x) - per-stack pause hooks.';
UPSERT keyword:msg_opencard SET word='openCard', category='message', aliases=['closeCard'], summary='Card lifecycle; openCard paints the scene - Myst drove its entire renderer from openCard handlers.';
UPSERT keyword:msg_openbg SET word='openBackground', category='message', aliases=['closeBackground'], summary='Background lifecycle, fired when the shared chrome changes.';
UPSERT keyword:msg_newcard SET word='newCard', category='message', aliases=['deleteCard','newBackground','deleteBackground','newStack','deleteStack'], summary='Creation/deletion notices for the structural containers; deleteCard is the last chance to object or clean up.';
UPSERT keyword:msg_openfield SET word='openField', category='message', aliases=['closeField','exitField'], summary='Field editing lifecycle: openField on entry, closeField when text changed and focus left, exitField when it did not.';
UPSERT keyword:msg_newpart SET word='newField', category='message', aliases=['newButton','deleteField','deleteButton'], summary='Part creation/deletion notices; on newButton set autoHilite of the target to true is the canonical example.';
UPSERT keyword:msg_resize SET word='resize', category='message', aliases=['scrollWindow','moveWindow','sizeWindow'], summary='Window geometry messages; resize fires as the user drags a window edge (2.x).';
UPSERT keyword:msg_appleevent SET word='appleEvent', category='message', summary='Apple events arrive as appleEvent class, id, sender on the current card - HyperCard on the MultiFinder network.';
UPSERT keyword:msg_abandonrequest SET word='abandonRequest', category='message', summary='The user hit Cmd-period; a handler can clean up before the script is killed (HyperCard Reference).';
UPSERT keyword:msg_doerror SET word='doError', category='message', summary='Sent when a script error occurs; trapping it lets a stack intercept its own crash dialogs (HyperCard Reference).';

-- == PROPERTIES ============================================================
UPSERT keyword:prop_name SET word='name', category='property', aliases=['long name','short name'], summary='Identity properties: name, long name (with type prefix), short name (bare); the name of the target is a card title for link buttons.';
UPSERT keyword:prop_id SET word='id', category='property', summary='Numeric identity of any part/card/stack; negative ids for background parts.';
UPSERT keyword:prop_text SET word='text', category='property', summary='The plain-text contents of a container; the text of a field equals its value, but the text of a script-less thing is just its characters.';
UPSERT keyword:prop_formattedtext SET word='formattedText', category='property', summary='Text plus style commands (2.x): the formattedText of a field round-trips fonts, sizes and styles as set-the-textStyle lines - rich text as source code.';
UPSERT keyword:prop_style SET word='style', category='property', summary='Object kind: the style of the target is "rectangle"; buttons: rectangle/default/transparent/checkbox/radio; fields: scrolling/plain/keyed.';
UPSERT keyword:prop_family SET word='family', category='property', aliases=['size','style','textStyle'], summary='Text appearance properties: family (font), size, style (bold/italic/underline/group), textStyle (plain/bold/...); set the textStyle of char 1 to 10 of fld "x" to bold.';
UPSERT keyword:prop_enabled SET word='enabled', category='property', summary='Whether a part responds; enable/disable are its verbs.';
UPSERT keyword:prop_visible SET word='visible', category='property', summary='Whether a part draws; show/hide are its verbs; hidden-but-present is the popup-field trick.';
UPSERT keyword:prop_showname SET word='showName', category='property', summary='Whether a button draws its name - off for transparent hotspot buttons over pictures.';
UPSERT keyword:prop_hilite SET word='hilite', category='property', summary='The pressed/checked look of a button; radio/checkbox state lives here.';
UPSERT keyword:prop_locktext SET word='lockText', category='property', summary='Editable or not; a locked field receives mouseDown/mouseUp like a button - the read-but-clickable state.';
UPSERT keyword:prop_dontwrap SET word='dontWrap', category='property', summary='Disables word wrap in a field.';
UPSERT keyword:prop_sharedtext SET word='sharedText', category='property', summary='Background field whose contents are shared across all its cards (vs per-card text) - the form-field memory.';
UPSERT keyword:prop_autoselect SET word='autoSelect', category='property', summary='Field property: clicking selects the whole line - the listbox illusion that drives click-a-line-to-go navigation.';
UPSERT keyword:prop_autohilite SET word='autoHilite', category='property', summary='Button property: flash while pressed; turn off for popup-holding buttons.';
UPSERT keyword:prop_rect SET word='rect', category='property', aliases=['topleft','botright','top','left','right','bottom','width','height','location','loc'], summary='Geometry as chunkable points: the loc of the target, the rect of fld 1; topleft/botright are items you can rewrite piecewise.';
UPSERT keyword:prop_marks SET word='marks', category='property', summary='The marked-chunk of a field - what find marked; the found-set cursor of the find/mark/find-next loop.';
UPSERT keyword:prop_marked SET word='marked', category='property', summary='Boolean per card: set the marked of card X to true; sort the marked cards / go to next marked card build the ad-hoc report.';
UPSERT keyword:prop_script SET word='script', category='property', summary='The source of an object as a text container - behavior is data you can put, get, and edit like any other field.';
UPSERT keyword:prop_owner SET word='owner', category='property', summary='The container that holds this one: the owner of a card is the stack; walks the containment tree upward.';
UPSERT keyword:prop_number SET word='number', category='property', summary='Position in the z/order collection: the number of card fields, the number of this card; the loop bound of every parts iteration.';
UPSERT keyword:prop_icon SET word='icon', category='property', aliases=['icons'], summary='Icon numbers drawn by a button (normal, pressed, disabled, highlighted).';
UPSERT keyword:prop_pict SET word='pict', category='property', aliases=['background','showPict'], summary='The PICT resource a part or card displays; background is the picture behind a card - Myst swapped scenes through bg picture machinery.';
UPSERT keyword:prop_lockscreen SET word='lockScreen', category='property', summary='Global redraw switch behind lock screen; batch updates, unlock with a visual effect.';
UPSERT keyword:prop_lockmessages SET word='lockMessages', category='property', summary='Suppresses system messages during a handler (auto-cleared on idle); the go-to-another-stack-without-firing-openCard trick.';
UPSERT keyword:prop_lockrecent SET word='lockRecent', category='property', summary='Keeps the Go menu history clean during scripted navigation.';
UPSERT keyword:prop_blindtyping SET word='blindTyping', category='property', summary='Accepts message-box typing with the box hidden - the back door protection schemes had to close, and rescue handlers relied on.';
UPSERT keyword:prop_itemdelimiter SET word='itemDelimiter', category='property', summary='The character that splits items (default comma); set itemDelimiter to "|" and any delimited format becomes a table - Myst did exactly this.';
UPSERT keyword:prop_userlevel SET word='userLevel', category='property', summary='Browsing(1)/typing(2)/authoring(3)/programming(4): the shipped-programmability dial, scriptable and stack-enforced.';
UPSERT keyword:prop_recent SET word='recent', category='property', summary='The Go menu history as a container: go recent, set the recent to empty.';

-- Every keyword is defined by the HyperTalk language concept -----------------
FOR $kid IN (SELECT VALUE id FROM keyword) {
  RELATE $kid->defined_in->concept:hypertalk;
};

-- Keywords that instantiate corpus concepts ---------------------------------
RELATE keyword:chk_character->relates_to->concept:chunk_expression SET note='chunk family member';
RELATE keyword:chk_word->relates_to->concept:chunk_expression SET note='chunk family member';
RELATE keyword:chk_item->relates_to->concept:chunk_expression SET note='chunk family member';
RELATE keyword:chk_line->relates_to->concept:chunk_expression SET note='chunk family member';
RELATE keyword:chk_ordinal->relates_to->concept:chunk_expression SET note='ordinal indexing of chunks';
RELATE keyword:chk_range->relates_to->concept:chunk_expression SET note='assignable chunk ranges';
RELATE keyword:cmd_put->relates_to->concept:container_model SET note='the container write verb';
RELATE keyword:cmd_get->relates_to->concept:container_model SET note='the container read verb';
RELATE keyword:cmd_set->relates_to->concept:property_model SET note='the generic property mutator';
RELATE keyword:cmd_hide->relates_to->concept:property_model SET note='sugar over the visible property';
RELATE keyword:cmd_lockscreen->relates_to->concept:property_model SET note='sugar over the lockScreen property';
RELATE keyword:prop_script->relates_to->concept:script_property SET note='behavior-as-data, literally';
RELATE keyword:prop_formattedtext->relates_to->concept:container_model SET note='the rich-text face of a container';
RELATE keyword:prop_text->relates_to->concept:container_model SET note='the plain-text face of a container';
RELATE keyword:fn_value->relates_to->concept:evaluation_modes SET note='explicit re-evaluation of text as code';
RELATE keyword:cmd_do->relates_to->concept:evaluation_modes SET note='statement-level re-evaluation';
RELATE keyword:op_typetest->relates_to->concept:coercion_ladder SET note='the introspection side of coercion';
RELATE keyword:op_precedence->relates_to->concept:operator_precedence SET note='the precedence table itself';
RELATE keyword:sp_it->relates_to->concept:target_me_it SET note='the implicit accumulator';
RELATE keyword:sp_me->relates_to->concept:target_me_it SET note='self on the message path';
RELATE keyword:sp_target->relates_to->concept:target_me_it SET note='origin of the current message';
RELATE keyword:msg_domenu->relates_to->concept:message_path SET note='commands are messages too';
RELATE keyword:msg_mousedown->relates_to->concept:message_path SET note='the canonical dispatch';
RELATE keyword:cmd_play->relates_to->concept:forgiving_semantics SET note='play harpsichord c e g: bare words as data';
RELATE keyword:cmd_find->relates_to->concept:find_mark_sort SET note='the search verb';
RELATE keyword:prop_marked->relates_to->concept:find_mark_sort SET note='the found-set flag';
RELATE keyword:cmd_sort->relates_to->concept:find_mark_sort SET note='sort the cards by field';
RELATE keyword:msg_idle->relates_to->concept:idle_time SET note='the idle heartbeat';
RELATE keyword:cmd_startusing->relates_to->concept:message_path SET note='libraries splice into the path';
RELATE keyword:fn_click->relates_to->concept:message_box SET note='clickLine made the message box a click target';
RELATE keyword:prop_userlevel->relates_to->concept:learnability_first SET note='the programmability dial shipped to users';
RELATE keyword:cmd_edit->relates_to->concept:q_authoring_runtime_unity SET note='the language edits its own host';
