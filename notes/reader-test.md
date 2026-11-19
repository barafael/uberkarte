# Reader test (H6): materials

*2026-11-19. Prepared, not yet run: it needs five people who don't program.*

## Question

Can someone who doesn't program read a handler in this language and say what
it does? And do braces and semicolons make that harder than HyperTalk's
`end` blocks? The decision to keep braces (2026-11-18) is the thing at stake.

## Design

- Five participants who don't program. One session each, about 20 minutes, on
  paper.
- Four handlers. Each participant sees each handler once, two in braces and two
  with `end` blocks, so nobody reads the same handler twice:

  | Group | Braces | `end` blocks |
  |---|---|---|
  | A (participants 1, 3, 5) | 1 Hello, 3 Pillars | 2 Status, 4 Bounce |
  | B (participants 2, 4) | 2 Status, 4 Bounce | 1 Hello, 3 Pillars |

- Order on the page: 1, 2, 3, 4 for everyone; the syntax alternates.
- Before the handlers, read out the context sentence for each (below). Do not
  explain any word of the language.
- Ask the one question per handler, write down the answer verbatim, then score
  it with the key. No time limit; note the time taken.

## Running it on screen

`reader-test/reader-test.html` runs the same test on a laptop, one participant
after another. Open the file in a browser; it needs no network and sends
nothing anywhere. The page:
- assigns groups A, B, A, B, A;
- shows each handler in the right syntax, without syntax colouring;
- records each answer and the seconds spent.

Afterwards its Results view shows each answer next to the key, takes the 0/1/2
scores and computes the pass rule and the means. Results are kept in that
browser's storage. Use **Copy results** after each session in case the storage
is cleared, and paste the text back here for analysis.

The page is generated from this file (`python3 notes/reader-test/build.py`), so
edit the handlers here, then rebuild.

## Scoring

Per handler: 2 = all key points, 1 = the main effect but a key point missing or
wrong, 0 = main effect wrong or "don't know".

**Pass (H6):** at least 3 of 5 participants score 2 on both of their braces
handlers. **Compare:** mean score for braces versus `end` blocks, per handler.
If braces score clearly lower (by 0.5 or more on average), revisit keeping them.

## Known confounds

- Handler 3: the braces version says `!=` where the `end` version says
  `is not`. `is not` cannot be parsed by stock Rhai (a two-word operator), so
  the two versions differ in more than block syntax on that line.
- Handler 4 uses the message name `frame` in both versions so that only the
  block syntax differs; real HyperTalk would use `idle`.
- Handler 2: the braces version says `the number of card`, the `end` version
  `the number of this card` (both are valid in this language now; HyperTalk
  needs `this`).

The braces versions are the ones in `prototype/stack-core/tests/path.rs`
(`reader_test_handlers_run` checks that handlers 3 and 4 behave as the key
says) and in the demo stacks (handlers 1 and 2).

## The handlers

### 1. Hello

Context: *"This is the script of a button labelled 'Say hello' on an address
card. The card has a field called name, which holds a person's full name."*
Question: **"What happens when someone clicks the button?"**

Braces:

```
on mouseUp {
    answer "Hello, " & word 1 of field "name";
}
```

`end` blocks:

```
on mouseUp
  answer "Hello, " & word 1 of field "name"
end mouseUp
```

Key: (a) a message or dialog appears; (b) it says "Hello, " followed by
(c) the first word of the name, so the first name only.

### 2. Status

Context: *"This is in the script of an address book. It has several cards, one
per person, and a field called status."*
Question: **"When does this run, and what does it do?"**

Braces:

```
on openCard {
    put "Card " & the number of card into field "status";
}
```

`end` blocks:

```
on openCard
  put "Card " & the number of this card into field "status"
end openCard
```

Key: (a) runs whenever a card is shown or opened; (b) writes "Card" and the
number of the card being shown (for example "Card 2") into the status field.

### 3. Pillars

Context: *"This belongs to a scene in a game with eight stone pillars named
pillar 1 to pillar 8, each with a hidden glowing picture named lit 1 to lit 8.
It is the script of the whole scene."*
Question: **"What happens when the player clicks pillar 3? And what has to
happen for the ship to come up?"**

Braces:

```
on mouseUp {
    global pillars, boat;
    put the name of target into name;
    if word 1 of name != "pillar" { pass mouseUp; }
    put word 2 of name into n;
    if pillars is empty { put "off,off,off,off,off,off,off,off" into pillars; }
    play "click";
    if item n of pillars is "on" {
        put "off" into item n of pillars;
        hide picture ("lit " & n);
    } else {
        put "on" into item n of pillars;
        show picture ("lit " & n);
    }
    if pillars is "off,on,off,off,on,on,off,off" {
        put "up" into boat;
        play "ship";
        put "Something heavy moves in the water by the dock." into field "status";
    }
}
```

`end` blocks:

```
on mouseUp
  global pillars, boat
  put the name of the target into name
  if word 1 of name is not "pillar" then pass mouseUp
  put word 2 of name into n
  if pillars is empty then put "off,off,off,off,off,off,off,off" into pillars
  play "click"
  if item n of pillars is "on" then
    put "off" into item n of pillars
    hide picture ("lit " & n)
  else
    put "on" into item n of pillars
    show picture ("lit " & n)
  end if
  if pillars is "off,on,off,off,on,on,off,off" then
    put "up" into boat
    play "ship"
    put "Something heavy moves in the water by the dock." into field "status"
  end if
end mouseUp
```

Key: (a) clicking a pillar plays a click and switches that pillar on or off,
showing or hiding its glow; pillar 3 lights up the first time it is clicked;
(b) the ship rises when exactly pillars 2, 5 and 6 are on and the others off;
(c) clicks on things other than pillars are passed on (partial credit is fine
without (c)).

### 4. Bounce

Context: *"This runs many times a second in a game. The ball is at ballX, ballY
and moves by dx, dy each time. The play area is about 480 wide."*
Question: **"What does this do to the ball?"**

Braces:

```
on frame {
    global ballX, ballY, dx, dy;
    add dx to ballX;
    add dy to ballY;
    if ballX < 6 or ballX > 474 { put -dx into dx; }
    if ballY < 6 { put -dy into dy; }
}
```

`end` blocks:

```
on frame
  global ballX, ballY, dx, dy
  add dx to ballX
  add dy to ballY
  if ballX < 6 or ballX > 474 then put -dx into dx
  if ballY < 6 then put -dy into dy
end frame
```

Key: (a) moves the ball each time by dx and dy; (b) when it goes past the
left or right edge it reverses its horizontal direction; (c) when it reaches
the top it reverses its vertical direction (it bounces).

## Record sheet

| Participant | Group | H1 | H2 | H3 | H4 | Time | Notes |
|---|---|---|---|---|---|---|---|
| 1 | A | braces: | end: | braces: | end: | | |
| 2 | B | end: | braces: | end: | braces: | | |
| 3 | A | braces: | end: | braces: | end: | | |
| 4 | B | end: | braces: | end: | braces: | | |
| 5 | A | braces: | end: | braces: | end: | | |
