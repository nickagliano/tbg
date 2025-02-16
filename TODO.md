# TODO:

## High priority
- [ ] just use FromSql for height and gender, clean up Gender's unnecessary complexity
- [ ] Move player and related files into new folder nested in models.
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

## Low or nice-to-haves
- [ ] Add docs and Doc-tests
- [ ] Think about how translations would work
  - [ ] Storing text in a way that it can easily be translated (like in Rails, the `config/locales/{language}.yml`)
- [ ] With a few adaptations, and if it's considered during development, this game has the potenital to be very accessible



## Done!
- [x] Add a title screen / landing page
  - [x] Add an ASCII title for THE BOOK GAME
