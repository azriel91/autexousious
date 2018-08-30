# Changelog

## 0.5.0 (2018-08-31)

### Added

* Implemented maps with bounds and layers. ([#47][#47], [#48][#48])
* Updated sprites configuration to take in `u32` instead of `f32` for sprite dimensions. ([#47][#47])
* Prevent characters from moving outside map margins. ([#48][#48])
* Use `typename` to derive `System` names. ([#52][#52])
* Implemented rudimentary character selection menu. ([#51][#51])

### Changed

* Sprite offsets are optional when declaring sprite sheets. ([#47][#47])
* Sprites are rendered using a dedicated sprite pass. ([#55][#55])
* `stop_run` sequence has been renamed to `run_stop`. ([#49][#49])
* Use state specific dispatcher for `GamePlayState`. ([#50][#50])

### Fixed

* Map layers with different sprite dimensions are now rendered with the right layout data. ([#55][#55])
* Characters switch to the `jump_descend` sequence when airborne during `walk`, `run`, and `run_stop`. ([#49][#49])

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
