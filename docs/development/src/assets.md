# Assets

## Loading

Assets can be loaded:

* At application startup:

    - <span class="item_good">+ Fast retrieval at runtime (no lag).</span>
    - <span class="item_warn">- Adds to application startup loading time.</span>
    - Typically only for persistent assets.

* When a `State` begins &mdash; `on_start()`:

    - <span class="item_good">+ Does not add to application startup loading time.</span>
    - <span class="item_warn">- State transition may not be smooth if there are many assets / assets are large.</span>

* At runtime:

    - <span class="item_good">+ Does not add to application startup loading time.</span>
    - <span class="item_warn">- User interface may not be responsive if there are many assets / assets are large.</span>

The persistence of each asset in memory can be:

* Persistent as long as the application continues to run.
* Free after use &mdash; typically when a `State` exits &mdash; `on_stop()`.

The following table shows examples of when it is suitable to use each mode of loading / persistence:


|                | Application Startup | State `on_start()`       | Runtime         |
| -------------- | ------------------- | ------------------------ | --------------- |
| Persistent     | Theme, fonts, menus | Character assets         | -               |
| Free after use | -                   | Stage assets             | Saved game data |
