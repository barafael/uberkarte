-- HyperTalk language reference: one node per keyword / family of keywords.
-- Grounded in the Apple HyperCard Script Language Guide (1996, HyperCard 2.x),
-- the grammars-v4 HyperTalk grammar, Wikipedia, and Tricks of the HyperTalk Masters.
-- `since` marks the HyperCard version that introduced the item where known.

-- == COMMANDS ===============================================================
UPSERT keyword:cmd_add SET word='add', category='command', aliases=['subtract','multiply','divide'], summary='Arithmetic on containers: add X to Y, subtract X from Y, multiply/divide Y by X; coerces operands to numbers and writes the result back.', confidence = 'high';
UPSERT keyword:cmd_answer SET word='answer', category='command', summary="Modal dialog with up to three buttons: answer \"msg\" with \"A\" or \"B\" or \"C\"; the clicked button name lands in it.", confidence = 'high';
UPSERT keyword:cmd_ask SET word='ask', category='command', aliases=['ask file'], summary='Dialog with an editable text field; the typed reply goes into it. ask file prompts for a file path instead.', confidence = 'high';
UPSERT keyword:cmd_choose SET word='choose', category='command', summary="Selects a HyperCard tool programmatically: choose browse tool, choose field tool; the tool function reads the current tool.", confidence = 'high';
UPSERT keyword:cmd_clickdrag SET word='click', category='command', aliases=['drag','type'], summary='Input synthesis: click at <point> [with <modifiers>], drag from <p1> to <p2>, type "text" [with commandkey]; the scripting-the-user idiom behind most automated demos.', confidence = 'high';
UPSERT keyword:cmd_convert SET word='convert', category='command', summary='Date/number arithmetic in place: convert <container> to seconds, convert <container> from dateItems to date; mutates the container it is given.', confidence = 'high';
UPSERT keyword:cmd_create SET word='create', category='command', aliases = ['delete'], summary="create menu <name> builds a custom menu (items are added with put ... into menu); create stack <name> makes a new stack. delete removes chunks of containers, buttons, fields, menus and menu items - cards are removed with doMenu \"Delete Card\".", confidence = 'high';
UPSERT keyword:cmd_do SET word='do', category='command', summary="Sends the value of an expression as a message to the current card, so a string can be run as HyperTalk: do line 3 of cd fld 1. The runtime escape hatch for generated code.", confidence = 'high';
UPSERT keyword:cmd_domenu SET word='doMenu', category='command', summary='Invokes a menu item by name: doMenu "New Card". Sends the doMenu message, so stacks can trap or extend every menu command.', confidence = 'high';
UPSERT keyword:cmd_edit SET word='edit the script', category='command', summary='Opens the script editor on a container: edit the script of this stack. The language can open its own source - authoring and runtime share one surface.', confidence = 'high';
UPSERT keyword:cmd_enable SET word='enable', category='command', aliases=['disable'], summary="enable / disable a button, menu or menu item (shorthand for setting its enabled property).", confidence = 'high';
UPSERT keyword:cmd_exit SET word='exit', category='command', summary='exit <handlerName> leaves a handler early; exit to hypercard abandons the current script entirely (the emergency brake for runaway handlers).', confidence = 'high';
UPSERT keyword:cmd_find SET word='find', category='command', aliases=['find chars','find whole','find string'], summary="Stack-wide text search over fields: find \"word\", with find whole / word / chars / string forms and an optional \"in field\" and \"of marked cards\" scope. Sets the foundChunk/foundField family on success and the result on failure. find does not mark cards; marking is the separate mark command (mark cards by finding ...).", confidence = 'high';
UPSERT keyword:cmd_get SET word='get', category='command', summary='Evaluates an expression and stores it in the implicit variable it: get the loc of the target. The one-assignment-per-line workhorse before named variables.', confidence = 'high';
UPSERT keyword:cmd_go SET word='go', category='command', aliases=['go to'], summary='Navigation: go to card "X", go next, go last, go stack "Y" [in a new window]. Sends the go message, which stacks can trap.', confidence = 'high';
UPSERT keyword:cmd_hide SET word='hide', category='command', aliases=['show'], summary='hide/show fields, buttons, pictures, titlebar, menubar; syntactic sugar over the visible property (hide fld i == set the visible of fld i to false).', confidence = 'high';
UPSERT keyword:cmd_lockscreen SET word='lock screen', category='command', aliases=['unlock screen'], summary='Suspends redraw so multi-step updates appear atomically: lock screen ... unlock screen [with visual effect]. Sugar for the lockScreen property.', confidence = 'high';
UPSERT keyword:cmd_fileio SET word='open file', category='command', aliases=['close file','read','write'], summary='Flat-file I/O: open file, read from file until/for N, write to file at eof; data arrives in it and leaves as text.', confidence = 'high';
UPSERT keyword:cmd_play SET word='play', category='command', aliases = ['beep'], summary="Sound: play \"res name\", play harpsichord c e g (notes as arguments), beep N; the sound is done tests completion. Speech (speak) needed the Speech Manager / PlainTalk and is not part of the base language.", confidence = 'high';
UPSERT keyword:cmd_pushpop SET word='push', category='command', aliases=['pop'], summary="push card / pop card keep a private push-list of card references (a scripted stack of places to return to). It is separate from the Recent list that go back / go forth use.", confidence = 'high';
UPSERT keyword:cmd_put SET word='put', category='command', summary='The assignment command, predicate-last: put 5 * 4 into x; also put ... before/after <chunk>. Creates locals on first use and coerces freely.', confidence = 'high';
UPSERT keyword:cmd_select SET word='select', category='command', summary='Moves the text selection: select the clickLine, select line 2 of fld "x", select before/after text; the selection is then readable as a container.', confidence = 'high';
UPSERT keyword:cmd_send SET word='send', category='command', summary='Sends a message string down a chosen path: send "mouseUp" to card button "OK" of card "Veracity", send "visual effect dissolve" to HyperCard (bypassing local traps).', confidence = 'high';
UPSERT keyword:cmd_set SET word='set', category='command', summary='set the <property> of <container> to <value>; the generic mutator, and the long form that show/hide, enable/disable and lock screen abbreviate.', confidence = 'high';
UPSERT keyword:cmd_show SET word='show', category='command', summary='See hide; also show card field "x" as the popup idiom paired with wait until the mouseClick.', confidence = 'high';
UPSERT keyword:cmd_sort SET word='sort', category='command', summary='sort cards [of stack] by <field> [ascending/descending] [text/numeric/datetime/international]; sorts the stack itself, reordering cards - the database leaking into the language.', confidence = 'high';
UPSERT keyword:cmd_startusing SET word='start using', category='command', aliases=['stop using'], summary='start using stack "Lib" splices another stack into the message path after this one - the library mechanism of HyperCard 2.x.', confidence = 'high';
UPSERT keyword:cmd_visual SET word='visual effect', category='command', summary="Screen transitions for navigation: visual effect iris open, wipe left, zoom in; issued before the card change it decorates.", confidence = 'high';
UPSERT keyword:cmd_wait SET word='wait', category='command', summary='wait 10 ticks / wait until <expr> / wait while <expr>; the cooperative pause that keeps idle and mouse messages alive.', confidence = 'high';

-- == FUNCTIONS =============================================================
UPSERT keyword:fn_value SET word='value()', category='function', aliases=['the value of'], summary='Evaluates a string as a HyperTalk expression and returns the result: value("2+2") is 4. The string-to-code door; do is its statement-shaped twin.', confidence = 'high';
UPSERT keyword:fn_length SET word='length', category='function', summary='Character count of a container or chunk: the length of fld "x", length(t).', confidence = 'high';
UPSERT keyword:fn_offset SET word='offset', category='function', summary='offset(pattern, string) returns the 1-based character position of the first occurrence, 0 if absent; the search primitive paired with is in.', confidence = 'high';
UPSERT keyword:fn_numberof SET word='number of', category='function', summary='the number of cards / buttons / lines of t / marked cards; the count function that drives every repeat-with loop over collections.', confidence = 'high';
UPSERT keyword:fn_random SET word='random', category='function', summary='random(n) returns an integer from 1 to n; the dice of HyperCard games.', confidence = 'high';
UPSERT keyword:fn_math SET word='sqrt', category='function', aliases = ['trunc','round','abs','min','max','sum','average','sin','cos','tan','atan','exp','exp1','exp2','ln','ln1','log2'], summary="The math library: trunc/round/abs, min/max/sum/average over item-delimited lists, trig, exponentials and logs. Number representation is not stated in the sources checked.", confidence = 'high';
UPSERT keyword:fn_charnum SET word='chartonum', category='function', aliases=['numtochar'], summary="charToNum(c) / numToChar(n) convert between a one-character string and its character code (Apple: ASCII value) - the byte-level door.", confidence = 'high';
UPSERT keyword:fn_finance SET word='annuity', category='function', aliases=['compound'], summary="annuity(rate, periods) and compound(rate, periods): the two financial functions (annuity factor and compound-interest factor); both take two arguments.", confidence = 'high';
UPSERT keyword:fn_params SET word='param', category='function', aliases=['paramcount','params'], summary='param(n), paramcount, params: positional access to a handler or function arguments; how generic handlers read their call site.', confidence = 'high';
UPSERT keyword:fn_datetime SET word='the date', category='function', aliases=['the time','the ticks','the seconds'], summary="Date/time reads: the date, the time, the ticks (60ths of a second since system startup), the seconds (since 1904); convert does the arithmetic.", confidence = 'high';
UPSERT keyword:fn_mouse SET word='the mouse', category='function', aliases=['the mouseLoc','the mouseClick'], summary='Polling reads: the mouse is down/up, the mouseLoc is a point, the mouseClick flips true once per click - the busy-wait inputs for games.', confidence = 'high';
UPSERT keyword:fn_keys SET word='the commandKey', category='function', aliases = ['the shiftKey','the optionKey'], summary="Modifier state: the commandKey, shiftKey and optionKey are up or down.", confidence = 'high';
UPSERT keyword:fn_result SET word='the result', category='function', summary='Error channel: empty after a successful go/find/open, an error string otherwise; the idiomatic existence test for cards and files.', confidence = 'high';
UPSERT keyword:fn_tool SET word='the tool', category='function', summary='Reads the current tool: the tool is browse tool; pairs with choose.', confidence = 'high';
UPSERT keyword:fn_version SET word='the version', category='function', summary='HyperCard version string; openStack guards read: if the version < 2.0 then answer "needs 2.0".', confidence = 'high';
UPSERT keyword:fn_env SET word='the stacks', category='function', aliases = ['the windows','the menus','the diskspace'], summary="Environment listings as line-delimited text: the stacks, the windows, the menus, the diskspace; iterate them with the number of lines of the windows.", confidence = 'high';
UPSERT keyword:fn_click SET word='the clickLoc', category='function', aliases=['the clickLine','the clickChunk','the clickText'], summary='What the last click hit: clickLoc (point), clickLine ("line 10 of card field 4"), clickChunk, clickText; clickLine + do built countless in-card interpreters.', confidence = 'high';
UPSERT keyword:fn_selected SET word='the selectedText', category='function', aliases=['the selectedLine','the selectedChunk','the selectedField'], summary='Selection reads: selectedText, selectedLine, selectedChunk (a container description you can re-select), selectedField.', confidence = 'high';

-- == CONTROL STRUCTURES ====================================================
UPSERT keyword:ctl_if SET word='if/then/else', category='control', summary='if <expr> then <stmt> | if ... then <block> else <block> end if; single-line and block forms, no case/switch in the base language.', confidence = 'high';
UPSERT keyword:ctl_repeat SET word='repeat', category='control', summary='One keyword, five shapes: repeat, repeat while/until, repeat for N [times], repeat with i = a to b [down to], each closed by end repeat.', confidence = 'high';
UPSERT keyword:ctl_exit SET word='exit <handler>', category='control', summary='Early return from a message handler: exit mouseUp. Named after the handler it leaves.', confidence = 'high';
UPSERT keyword:ctl_exitrepeat SET word='exit repeat', category='control', summary='Breaks out of the innermost repeat loop.', confidence = 'high';
UPSERT keyword:ctl_nextrepeat SET word='next repeat', category='control', summary='Skips to the next iteration - the continue of HyperTalk.', confidence = 'high';
UPSERT keyword:ctl_pass SET word='pass', category='control', summary='pass <message> re-sends the message up the path untouched - the safety valve of every doMenu/mouseUp trap; omitting it swallows the system.', confidence = 'high';
UPSERT keyword:ctl_return SET word='return', category='control', summary='Returns a value from a function handler (or bare return to return empty).', confidence = 'high';
UPSERT keyword:ctl_global SET word='global', category='control', summary="global a, b declares handler-wide globals that outlive the handler and are shared across stacks until HyperCard quits; variables are otherwise local to the handler.", confidence = 'high';
UPSERT keyword:ctl_on SET word='on/end', category='control', summary='on <message> [params] ... end <message> defines a message handler; the handler name must be repeated at the end - a bookend grammar that made scripts self-describing.', confidence = 'high';
UPSERT keyword:ctl_function SET word='function/end', category='control', summary="function <name> [params] ... end <name> defines a value-returning handler, called with parentheses like a built-in function; a handler can also redefine a built-in function by reusing its name.", confidence = 'high';
UPSERT keyword:ctl_comment SET word='--', category='control', summary='Comment to end of line; two minus signs, borrowed from the natural-language look of the language.', confidence = 'high';
UPSERT keyword:ctl_continue SET word='|', category='control', aliases=['¬'], summary="Line continuation with the option-return character (the 'not' sign); a logical line may span several physical lines.", confidence = 'high';

-- == CHUNK KEYWORDS ========================================================
UPSERT keyword:chk_character SET word='character', category='chunk', aliases=['char','chars'], summary='Character chunk: character 3 of t, the first char of t; the finest-grained text container.', confidence = 'high';
UPSERT keyword:chk_word SET word='word', category='chunk', summary='Whitespace-delimited token: the third word of theFilesText; the tokenizer that never needed writing.', confidence = 'high';
UPSERT keyword:chk_item SET word='item', category='chunk', summary='Delimiter-based token: item 2 of pos; delimiter is the itemDelimiter property (default comma), making CSV parsing a keyword.', confidence = 'high';
UPSERT keyword:chk_line SET word='line', category='chunk', summary='Return-delimited record: line 5 of card field "sometext"; the row abstraction of the field-as-database idiom.', confidence = 'high';
UPSERT keyword:chk_ordinal SET word='first', category='chunk', aliases=['second','third','fourth','fifth','sixth','seventh','eighth','ninth','tenth','middle','last','any'], summary='English ordinals as indexes: the last line of t, the middle word of t, the any card; zero-based counting nowhere to be found.', confidence = 'high';
UPSERT keyword:chk_range SET word='X to Y', category='chunk', summary="Chunk ranges: character 1 to 10 of fld \"x\", line 2 to 4 of t; ranges are assignable containers, so text can be replaced in place by putting into the chunk.", confidence = 'high';

-- == CONTAINER KEYWORDS ====================================================
UPSERT keyword:obj_card SET word='card', category='container', aliases=['cd','cards'], summary='A single screen of records; addressable as card "name", card id 12, the second card, this card, cd fld 1.', confidence = 'high';
UPSERT keyword:obj_background SET word='background', category='container', aliases=['bkgnd','bg','bkgnds'], summary="The shared chrome behind cards: a background's parts and picture appear on every card that uses it.", confidence = 'high';
UPSERT keyword:obj_button SET word='button', category='container', aliases=['btn','buttons','btns'], summary='A clickable region with a script; style rectangle/default/transparent/opaque/checkbox/radio.', confidence = 'high';
UPSERT keyword:obj_field SET word='field', category='container', aliases=['fld','fields','flds'], summary='Text region: scrolling or not, locked or not; the primary data container of the object model and of chunk expressions.', confidence = 'high';
UPSERT keyword:obj_stack SET word='stack', category='container', aliases=['this stack'], summary='The file-level container of cards, backgrounds, parts and global script; also a message-path level.', confidence = 'high';
UPSERT keyword:obj_this SET word='this', category='container', summary='Context anchor: this card, this stack, this background - resolved against where the running handler lives.', confidence = 'high';
UPSERT keyword:obj_part SET word='part', category='container', summary="Any button or field: the parts of this card, card part \"OK\".", confidence = 'high';
UPSERT keyword:obj_window SET word='window', category='container', summary='OS windows as containers: the window "Paint", the card window, the message watcher; close window / the number of windows.', confidence = 'high';
UPSERT keyword:obj_menu SET word='menu', category='container', aliases=['menuitem'], summary='Menus and their items as containers: create menu, set the enabled of menuitem 3 of menu "Edit" to false.', confidence = 'high';
UPSERT keyword:obj_id SET word='id', category='container', summary="Stable numeric identity: card id 12, button id 7; the addressing mode that survives renames and reordering.", confidence = 'high';
UPSERT keyword:sp_me SET word='me', category='special', summary='The container whose script is running - self for the message path; put line 1 of me reads the field the handler lives in.', confidence = 'high';
UPSERT keyword:sp_it SET word='it', category='special', summary='The implicit variable: filled by answer, ask, get, read; the reason HyperTalk needs no temporary declarations in simple scripts.', confidence = 'high';
UPSERT keyword:sp_target SET word='the target', category='special', summary='The object that first received the current message; word 2 of the name of the target tells a stack-level handler what was clicked.', confidence = 'high';
UPSERT keyword:sp_selection SET word='the selection', category='special', summary='The current text selection as a container: put the selection into myWord; select replaces it. Clipboard-free interaction with the user.', confidence = 'high';

-- == OPERATORS =============================================================
UPSERT keyword:op_arith SET word='+ -', category='operator', aliases=['* / div mod'], summary='Numeric operators; operands are coerced from text, and div/mod are the integer pair. Precedence: * / div mod above + -.', confidence = 'high';
UPSERT keyword:op_power SET word='^', category='operator', summary='Exponentiation; binds tighter than * and /.', confidence = 'high';
UPSERT keyword:op_concat SET word='&', category='operator', aliases=['&&'], summary='& concatenates exactly; && inserts a space between operands - the sentence-builder of natural-language output.', confidence = 'high';
UPSERT keyword:op_compare SET word='=', category='operator', aliases=['is','is not','<>','<','>','<=','>='], summary="Comparison for numbers or text (Apple), also Boolean values; is / is not are the English spellings of = / <>. How mixed number-and-text operands compare is not specified in the guide text checked.", confidence = 'high';
UPSERT keyword:op_verbal SET word='contains', category='operator', aliases=['is in','is not in','is within','is not within'], summary="Verbal containment: t contains \"x\", \"a\" is in t, point is within rect; predicates that read as sentences and drive repeat-while find loops. (within binds with not and there is, at level 2; contains and in are level 7.)", confidence = 'high';
UPSERT keyword:op_typetest SET word='is a', category='operator', aliases=['is an','is not a','is not an'], summary="Type predicates: x is a number / integer / point / rect / date / empty / logical - the introspection half of the coercion model. (The type is 'logical'; there is no 'boolean'.)", confidence = 'high';
UPSERT keyword:op_thereis SET word='there is a', category='operator', aliases=['there is an','there is no','there is not a'], summary='Existence predicates: if there is a file filePath, if there is not a window "frame" - the guard clause of defensive HyperTalk.', confidence = 'high';
UPSERT keyword:op_logic SET word='and', category='operator', aliases=['or','not'], summary="Boolean operators and, or, not; and and bind tighter than or, with or loosest. The guide does not say whether and/or short-circuit.", confidence = 'medium';
UPSERT keyword:op_precedence SET word='operator precedence', category='operator', summary="Ten levels, tightest first (Apple's table 7-2): (1) grouping; (2) unary minus, not, there is [not] a/an, within; (3) ^; (4) * / div mod; (5) + -; (6) & &&; (7) > < <= >= together with is in, contains, is not in and is [not] a/an; (8) = is, is not, <>; (9) and; (10) or. Operators of equal precedence evaluate left to right, except ^, which goes right to left. Concatenation binds tighter than comparison; and/or bind loosest.", confidence = 'high';

-- == CONSTANTS =============================================================
UPSERT keyword:con_empty SET word='empty', category='constant', summary='The empty value: put empty into x clears a container; if MY_Pillars is empty tests first-run state - Myst used it as its uninitialized sentinel.', confidence = 'high';
UPSERT keyword:con_pi SET word='pi', category='constant', summary='The circle constant, spelled out.', confidence = 'high';
UPSERT keyword:con_whitespace SET word='space', category='constant', aliases = ['return','tab','quote','formFeed','lineFeed'], summary="Named characters: space, tab, return, quote, formFeed, lineFeed - string building without escape sequences. (Apple's constants are down, empty, false, formFeed, lineFeed, pi, quote, space, tab, true, up and zero through ten; comma and colon are not HyperCard constants.)", confidence = 'high';
UPSERT keyword:con_numbers SET word='zero', category='constant', aliases=['one','two','three','four','five','six','seven','eight','nine','ten'], summary='Number words zero through ten usable as literals.', confidence = 'high';
UPSERT keyword:con_booleans SET word='true', category='constant', aliases = ['false'], summary="Logical literals true and false (the constants list also has up, down, empty, pi, space, tab, return, quote and the numbers zero..ten); yes/no/on/off are not language constants.", confidence = 'high';
UPSERT keyword:con_mousestate SET word='down', category='constant', aliases=['up'], summary='Mouse/modifier state words: the mouse is down, the commandKey is down.', confidence = 'high';

-- == SYSTEM MESSAGES =======================================================
UPSERT keyword:msg_startup SET word='startup', category='message', summary='Sent once when HyperCard launches, first caught by the Home stack - the system-wide autoexec.', confidence = 'high';
UPSERT keyword:msg_quit SET word='quit', category='message', summary="Sent to the current card when you choose Quit HyperCard (Command-Q), just before HyperCard goes away.", confidence = 'high';
UPSERT keyword:msg_suspend SET word='suspend', category='message', aliases = ['resume'], summary="suspend: sent to the current card when HyperCard is suspended by launching another application with the open command; resume when it resumes. Games paused sound and timers here.", confidence = 'high';

UPSERT keyword:msg_idle SET word='idle', category='message', summary="Sent to the current card repeatedly whenever nothing else is happening - the heartbeat for animation and background work.", confidence = 'high';
UPSERT keyword:msg_mousedown SET word='mouseDown', category='message', summary='Button pressed inside a part/card; sent to locked fields too, which is why locked fields can be buttons.', confidence = 'high';
UPSERT keyword:msg_mouseup SET word='mouseUp', category='message', summary='Released inside the same rectangle - the canonical HyperTalk entry point, on mouseUp ... end mouseUp.', confidence = 'high';
UPSERT keyword:msg_mousestillsdown SET word='mouseStillDown', category='message', summary='Repeated while the button is held - drag-style interaction without the drag command.', confidence = 'high';
UPSERT keyword:msg_mouseenter SET word='mouseEnter', category='message', aliases = ['mouseLeave'], summary="mouseEnter / mouseLeave: sent to a button or field when the pointer first moves inside / outside its rectangle; hover behavior, 1987 edition.", confidence = 'high';
UPSERT keyword:msg_mousewithin SET word='mouseWithin', category='message', summary='Repeated while the pointer sits inside a part - per-frame hover.', confidence = 'high';
UPSERT keyword:msg_mousedoubleclick SET word='mouseDoubleClick', category='message', summary="Sent after the second click of a double-click is released, within the double-click interval and 4 pixels of the first click and inside the same object; it is then the only system message sent for the second click.", confidence = 'high';
UPSERT keyword:msg_keydown SET word='keyDown', category='message', summary="Sent to the current card when a key is pressed; the key is passed as the message parameter.", confidence = 'high';

UPSERT keyword:msg_functionkey SET word='functionKey', category='message', summary="Sent to the current card when a function key on the Apple Extended Keyboard is pressed; the functionKey command also performs Undo, Cut, Copy and Paste for parameters 1 to 4.", confidence = 'high';
UPSERT keyword:msg_commandkeydown SET word='commandKeyDown', category='message', aliases = [], summary="commandKeyDown \"H\": Command-key equivalents arrive as messages, so shortcuts are interceptable and extendable.", confidence = 'high';
UPSERT keyword:msg_returnkey SET word='returnKey', category='message', aliases=['enterKey','tabKey'], summary='Editing keys sent to fields/cards: returnKey, enterKey, tabKey; autoTab rides on tabKey.', confidence = 'high';
UPSERT keyword:msg_enterinfield SET word='enterInField', category='message', aliases=['returnInField'], summary="Sent to a field when the Enter key is pressed with an insertion point or selection in it; if no handler intercepts it and the text changed, HyperCard sends closeField. (returnInField is a separate command that enters a return.)", confidence = 'high';
UPSERT keyword:msg_domenu SET word='doMenu', category='message', summary='Wraps every menu pick and doMenu call; the classic trap: on doMenu theItem ... pass doMenu.', confidence = 'high';
UPSERT keyword:msg_go SET word='go', category='message', summary="HyperTalk commands are also messages, and a handler with the same name as a command, placed anywhere in the hierarchy, runs instead of the built-in one (Apple's guide); functions can be redefined the same way. So a stack can trap go to rewrite or veto its own navigation, but should pass the message on if it wants the built-in behavior.", confidence = 'high';
UPSERT keyword:msg_openstack SET word='openStack', category='message', aliases=['closeStack'], summary='Stack lifecycle; openStack is the constructor of every stack (init globals, choose browse tool, set userLevel).', confidence = 'high';
UPSERT keyword:msg_suspendstack SET word='suspendStack', category='message', aliases = ['resumeStack'], summary="suspendStack: sent to the current card when you leave an open stack to go to another; resumeStack when HyperCard returns to a stack.", confidence = 'high';
UPSERT keyword:msg_opencard SET word='openCard', category='message', aliases=['closeCard'], summary="Card lifecycle: openCard is sent to a card when you go to it, closeCard just before you leave; openCard handlers can paint a scene (Myst's scripts use them).", confidence = 'high';
UPSERT keyword:msg_openbg SET word='openBackground', category='message', aliases=['closeBackground'], summary='Background lifecycle, fired when the shared chrome changes.', confidence = 'high';
UPSERT keyword:msg_newcard SET word='newCard', category='message', aliases=['deleteCard','newBackground','deleteBackground','newStack','deleteStack'], summary='Creation/deletion notices for the structural containers; deleteCard is the last chance to object or clean up.', confidence = 'high';
UPSERT keyword:msg_openfield SET word='openField', category='message', aliases=['closeField','exitField'], summary='Field editing lifecycle: openField on entry, closeField when text changed and focus left, exitField when it did not.', confidence = 'high';
UPSERT keyword:msg_newpart SET word='newField', category='message', aliases=['newButton','deleteField','deleteButton'], summary='Part creation/deletion notices; on newButton set autoHilite of the target to true is the canonical example.', confidence = 'high';
UPSERT keyword:msg_resize SET word="sizeWindow", category='message', aliases = ['moveWindow'], summary="sizeWindow and moveWindow: sent when the card window is resized or moved by the user or a handler. (A 'resize' message was not found in the sources checked.)", confidence = 'high';
UPSERT keyword:msg_appleevent SET word='appleEvent', category='message', summary='Apple events arrive as appleEvent class, id, sender on the current card - HyperCard on the MultiFinder network.', confidence = 'high';

UPSERT keyword:msg_doerror SET word="errorDialog", category='message', summary="errorDialog: sent instead of the error dialog when lockErrorDialogs is true and a script error occurs; a handler can intercept the error text.", confidence = 'high';

-- == PROPERTIES ============================================================
UPSERT keyword:prop_name SET word='name', category='property', aliases=['long name','short name'], summary='Identity properties: name, long name (with type prefix), short name (bare); the name of the target is a card title for link buttons.', confidence = 'high';
UPSERT keyword:prop_id SET word='id', category='property', summary="Numeric identity of any part/card/stack, unique and permanent within its stack.", confidence = 'high';


UPSERT keyword:prop_style SET word='style', category='property', summary="Look of a button or field. Fields: transparent, opaque, rectangle, shadow, scrolling. Buttons: transparent, opaque, rectangle, roundRect, shadow, checkBox, radioButton, standard, default, oval.", confidence = 'high';
UPSERT keyword:prop_family SET word="family", category='property', aliases = [], summary="family: a button's radio-button group number (0 = none); buttons sharing a family number act as one exclusive set. (Text appearance is a separate set: textFont, textSize, textStyle, textAlign, textHeight.)", confidence = 'high';
UPSERT keyword:prop_enabled SET word='enabled', category='property', summary='Whether a part responds; enable/disable are its verbs.', confidence = 'high';
UPSERT keyword:prop_visible SET word='visible', category='property', summary='Whether a part draws; show/hide are its verbs; hidden-but-present is the popup-field trick.', confidence = 'high';
UPSERT keyword:prop_showname SET word='showName', category='property', summary='Whether a button draws its name - off for transparent hotspot buttons over pictures.', confidence = 'high';
UPSERT keyword:prop_hilite SET word='hilite', category='property', summary='The pressed/checked look of a button; radio/checkbox state lives here.', confidence = 'high';
UPSERT keyword:prop_locktext SET word='lockText', category='property', summary='Editable or not; a locked field receives mouseDown/mouseUp like a button - the read-but-clickable state.', confidence = 'high';
UPSERT keyword:prop_dontwrap SET word='dontWrap', category='property', summary='Disables word wrap in a field.', confidence = 'high';
UPSERT keyword:prop_sharedtext SET word='sharedText', category='property', summary='Background field whose contents are shared across all its cards (vs per-card text) - the form-field memory.', confidence = 'high';
UPSERT keyword:prop_autoselect SET word='autoSelect', category='property', summary='Field property: clicking selects the whole line - the listbox illusion that drives click-a-line-to-go navigation.', confidence = 'high';
UPSERT keyword:prop_autohilite SET word='autoHilite', category='property', summary='Button property: flash while pressed; turn off for popup-holding buttons.', confidence = 'high';
UPSERT keyword:prop_rect SET word='rect', category='property', aliases=['topleft','botright','top','left','right','bottom','width','height','location','loc'], summary='Geometry as chunkable points: the loc of the target, the rect of fld 1; topleft/botright are items you can rewrite piecewise.', confidence = 'high';

UPSERT keyword:prop_marked SET word='marked', category='property', summary='Boolean per card: set the marked of card X to true; sort the marked cards / go to next marked card build the ad-hoc report.', confidence = 'high';
UPSERT keyword:prop_script SET word='script', category='property', summary='The source of an object as a text container - behavior is data you can put, get, and edit like any other field.', confidence = 'high';
UPSERT keyword:prop_owner SET word='owner', category='property', summary="The owner of a card is its background (equal to the name of its background); for a window it is HyperCard or the name of an XCMD. Not a general walk up the containment tree.", confidence = 'high';
UPSERT keyword:prop_number SET word='number', category='property', summary='Position in the z/order collection: the number of card fields, the number of this card; the loop bound of every parts iteration.', confidence = 'high';
UPSERT keyword:prop_icon SET word='icon', category='property', aliases=['icons'], summary='Icon numbers drawn by a button (normal, pressed, disabled, highlighted).', confidence = 'high';
UPSERT keyword:prop_pict SET word="showPict", category='property', aliases = [], summary="showPict: a boolean on a card/background saying whether its background graphics are visible. Pictures as windows come from the picture command, not from a pict property.", confidence = 'high';
UPSERT keyword:prop_lockscreen SET word='lockScreen', category='property', summary='Global redraw switch behind lock screen; batch updates, unlock with a visual effect.', confidence = 'high';
UPSERT keyword:prop_lockmessages SET word='lockMessages', category='property', summary='Suppresses system messages during a handler (auto-cleared on idle); the go-to-another-stack-without-firing-openCard trick.', confidence = 'high';
UPSERT keyword:prop_lockrecent SET word='lockRecent', category='property', summary='Keeps the Go menu history clean during scripted navigation.', confidence = 'high';
UPSERT keyword:prop_blindtyping SET word='blindTyping', category='property', summary="Allows typing into the Message box while it is hidden.", confidence = 'high';
UPSERT keyword:prop_itemdelimiter SET word='itemDelimiter', category='property', summary="The character that splits items (default comma; added in HyperCard 2.1); set itemDelimiter to \"|\" and any delimited format becomes a table.", confidence = 'high';
UPSERT keyword:prop_userlevel SET word='userLevel', category='property', summary="Five levels: 1 browsing, 2 typing, 3 painting, 4 authoring, 5 scripting - the shipped-programmability dial, settable by script; the 'magic' trick or a script could raise it, so it was a convenience, not security.", confidence = 'high';


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
UPSERT keyword:msg_arrowkey SET word='arrowKey', category='message', aliases=['controlKey'], summary='Keyboard messages beyond keyDown: arrowKey "left"|"right"|"up"|"down" and controlKey <number>; functionKey covers the F-keys.', confidence = 'high';
UPSERT keyword:cmd_mark SET word='mark', category='command', aliases=['unmark'], summary='Marks cards: mark card 3, mark all cards, mark cards where <expression>, mark cards by finding "x". Marking is how the found-set idiom works; the marked property is a card boolean.', confidence = 'high';
RELATE keyword:msg_arrowkey->defined_in->concept:hypertalk;
RELATE keyword:cmd_mark->defined_in->concept:hypertalk;
