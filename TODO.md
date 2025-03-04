# TODO:

## High priority
- [ ] Implement Decisions. Record them at all dialogue inputs and choices.
  - [ ] Start with name, gender, and height inputs. Deliberation time, etc.
- [ ] Add created at / updated at to all new tables
  - [ ] Dialogue, responses, decisions
- [ ] Enhance routines
  - [ ] Abstract out the raw/un-raw logic. As a "Routine" trait?
- [ ] Add a loop (in game engine? In routines?) that stops the game and displays "window too small" if under
      a certain min height or min width
- [ ] Add seeds
  - [ ] Don't seed, and don't try to create tables with each db connection!
    - [ ] For now, just do a trivial check and assert that the last item from seeds exists
- [ ] Add "deliberation_time"
- [ ] Figure out how to abstract the main interfaces (world nav, dialogue/narration, book builder, battle)
  - [x] Bigger than the `game_engines/interactions/`. Routines?
  - [x] Starts and ends raw mode?
  - [ ] Can wrap whatever this concept is in amazing error handling + break down logic (so we don't get stuck in raw mode, etc.)
- [ ] Add battle models. Turns, BattleLog, ... everything...
- [ ] Dialogue system
  - [ ] Add dialogue models. Dialogue history, decisions, etc.
  - [ ] How to use dialogue routine?
- [ ] Add BookBuilder experience
- [ ] Add Battle experience
- [ ] print_menu should *not* render! It should return a &str
- [ ] Return the player id when creating the player instead of having to load it afterwards.
- [ ] Return the game state id when creating the game state instead of having to load it afterwards.
- [ ] Catch up on tests!
  - [ ] Test everything now that crossterm is implemented
  - [x] Music tests (it does't make sense to test some of the methods)

## Medium
- [ ] Add player background
- [ ] The existence of InterfaceMode on tbg::game_engine implies the need for a ui crate.
  - [ ] Some of terminal utils needs to be abstracted out
  - [ ] The menu struct needs to exist
- [ ] Add a settings model, let users update their typing speed (used in simulate_typing)
- [ ] Consider when to save, and what to save in the gamestate
  - [ ] Does player x, y really need to be *persisted* each render? That's a lot of DB calls....
  - [ ] Does UI state need to be persisted? I'm leaning towards yes, so you can exit in the middle of a battle, or dialouge, etc.
    - It would suck if you lost all progress unless you could manually save, and saving only happens while roaming.
- [ ] Model the epics, and stages within epics.
  - [ ] Validate epic and stage names, "intro", "character_creation", etc.
  - [ ] Add "advance" method to safely move a player through validated states (i.e., one stage to the next, one epic to the next)
- [ ] Test that all "update" methods update the updated_at value
- [ ] Let users choose to continue, or create a new save file at the title screen
- [ ] Generate user seed
- [ ] Remove the dialogue_id from the responses in the YAML structure... unnecessary field, but having it makes deserialization way more simple.

## Low or nice-to-haves
- [ ] Add docs and Doc-tests
- [ ] Think about how translations would work
  - [ ] Storing text in a way that it can easily be translated (like in Rails, the `config/locales/{language}.yml`)
- [ ] With a few adaptations, and if it's considered during development, this game has the potenital to be very accessible
- [ ] Right now the GameEngine::Routines::Battle routine is setup for Player vs NPC. I've decided to focus on PvNPC,
      without worrying too much about PvP. It's in the back of my head but I can't overengineer PvNPC right now.
- [ ] Consider removing the `IF NOT EXISTS` statements from DB setup
- [ ] Consider renaming Dialogue to DialogueNode
- [ ] Consider moving the `CharacterType` definition out of the dialogue module.

## Done!
- [x] Move player and related files into new folder nested in models.
- [x] just use FromSql for height and gender, clean up Gender's unnecessary complexity
- [x] Add a title screen / landing page
  - [x] Add an ASCII title for THE BOOK GAME
