# Changelog

## master (unreleased)

### Added

* Added `dodge` sequence. ([#102])
* Added `dash_forward` and `dash_back` sequences. ([#102])

### Changed

* Use logic clock based system to update object components each tick. ([#92])
* Use logic clock based system to update map components each tick. ([#99])
* ***Breaking:*** In objects and maps, sprites are now specified using `sprite = { sheet = 0, index = 0 }`. ([#92])

### Fixed

* Map layers are now positioned correctly. ([#99])
* Game does not go past loading when asset loading has error. ([#101])

### Removed

* No longer use `amethyst_animation` to update object and map components. ([#92], [#99])

[#92]: https://gitlab.com/azriel91/autexousious/issues/92
[#99]: https://gitlab.com/azriel91/autexousious/issues/99
[#101]: https://gitlab.com/azriel91/autexousious/issues/101
[#102]: https://gitlab.com/azriel91/autexousious/issues/102

## 0.9.0 (2019-02-15)

### Added

* Control input via stdin. ([#93])
* First cut of hot reloading. ([#94])

### Changed

* Characters and objects are now instantiated using prefabs. ([#56])
* `KeyboardEscapeIntercept` is now tested properly without simulating input. ([#15])
* Load game objects and maps asynchronously. ([#94])

[#15]: https://gitlab.com/azriel91/autexousious/issues/15
[#93]: https://gitlab.com/azriel91/autexousious/issues/93
[#94]: https://gitlab.com/azriel91/autexousious/issues/94

## 0.8.0 (2019-01-04)

### Changed

* Object animations are updated in a dedicated system. ([#81])
* Components are now more granular. ([#83])
* Components are augmented onto existing entities instead of added when built. ([#56])

[#56]: https://gitlab.com/azriel91/autexousious/issues/56
[#81]: https://gitlab.com/azriel91/autexousious/issues/81
[#83]: https://gitlab.com/azriel91/autexousious/issues/83

## 0.7.0 (2018-11-23)

### Added

* Naive collision detection and effects. ([#69])
* Flinch and foward falling sequences. ([#70])
* Game win condition when there is one player remaining. ([#71])
* Mirrored collision detection. ([#82])

[#69]: https://gitlab.com/azriel91/autexousious/issues/69
[#70]: https://gitlab.com/azriel91/autexousious/issues/70
[#71]: https://gitlab.com/azriel91/autexousious/issues/71
[#82]: https://gitlab.com/azriel91/autexousious/issues/82

## 0.6.0 (2018-10-12)

### Added

* Map selection UI that allows users to select maps. ([#66])
* Character selection controllable via stdin. ([#67])
* Map selection controllable via stdin. ([#67])
* Game mode selection controllable via stdin. ([#67])
* Configuration types for hit (`interactions`) and hurt (`body`) boxes. ([#68])

### Changed

* Use `AssetSlug` (*namespace/name*) to reference assets. ([#57])
* Updated Amethyst to make use of sprite batching and custom events. ([#63])

### Fixed

* Allow maps without layers. ([#72])

[#57]: https://gitlab.com/azriel91/autexousious/issues/57
[#63]: https://gitlab.com/azriel91/autexousious/issues/63
[#66]: https://gitlab.com/azriel91/autexousious/issues/66
[#67]: https://gitlab.com/azriel91/autexousious/issues/67
[#68]: https://gitlab.com/azriel91/autexousious/issues/68
[#72]: https://gitlab.com/azriel91/autexousious/issues/72

## 0.5.0 (2018-08-31)

### Added

* Implemented maps with bounds and layers. ([#47], [#48])
* Updated sprites configuration to take in `u32` instead of `f32` for sprite dimensions. ([#47])
* Prevent characters from moving outside map margins. ([#48])
* Use `typename` to derive `System` names. ([#52])
* Implemented rudimentary character selection menu. ([#51])

### Changed

* Sprite offsets are optional when declaring sprite sheets. ([#47])
* Sprites are rendered using a dedicated sprite pass. ([#55])
* `stop_run` sequence has been renamed to `run_stop`. ([#49])
* Use state specific dispatcher for `GamePlayState`. ([#50])

### Fixed

* Map layers with different sprite dimensions are now rendered with the right layout data. ([#55])
* Characters switch to the `jump_descend` sequence when airborne during `walk`, `run`, and `run_stop`. ([#49])

[#47]: https://gitlab.com/azriel91/autexousious/issues/47
[#48]: https://gitlab.com/azriel91/autexousious/issues/48
[#49]: https://gitlab.com/azriel91/autexousious/issues/49
[#50]: https://gitlab.com/azriel91/autexousious/issues/50
[#51]: https://gitlab.com/azriel91/autexousious/issues/51
[#52]: https://gitlab.com/azriel91/autexousious/issues/52
[#55]: https://gitlab.com/azriel91/autexousious/issues/55

## 0.4.0 (2018-07-20)

### Added

* Implemented jump.
* Implemented double tap to run.
* Mirror sprites when entity is facing left.
* Created test harness to make it easier to test logic.

## 0.3.0 (2018-06-08)

### Added

* Read controller configuration.
* Update character sequence based on input.

## 0.2.0 (2018-04-26)

### Added

* Loads object sprites and animations.
* Creates an entity per object.

## 0.1.0 (2018-03-24)

### Added

* Loads fonts and displays a simple menu.
* Published on [itch.io](https://azriel91.itch.io/will)
