# APK UI Port Status

Reference: Anixart APK 10.5 (`decompile/jadx-out/resources/res/layout`, `decompile/jadx-out/sources/com/swiftsoft/anixartd`). Decompiled output is an analysis reference, not original source. The 547 layout XML files include Android library layouts, repeated list rows, dialogs, and configuration variants; they are not 547 independent app pages.

This is a screen-family inventory, not a claim of complete visual or functional parity. `HTML shell` means a hand-written web screen exists; `partial` means its APK content or interactions are incomplete; `pending` means no faithful port exists yet. The supplied screenshots appear modified/older than the APK and are treated as secondary references.

## Current web screen families

| APK family | APK references | Current port | Remaining work |
|---|---|---|---|
| App shell and main navigation | `activity_main.xml`, `fragment_tabs.xml`, `MainActivity` | Partial HTML shell with search header and five-item bottom navigation | Extract exact icon vectors, dynamic search hint, notification badge, responsive navigation rail, back stack |
| Home tabs/catalog | `HomeFragment`, `HomeTabFragment`, `fragment_tab.xml`, `fragment_home_tab.xml` | Partial; tab strip, hero, shortcuts, one random release card | Implement all nine home tabs, real paged grids, custom tab reorder/edit, loading/empty/error states |
| Discover | `fragment_discover.xml`, `discover_*.xml`, `DiscoverFragment` | Partial; recommendations, discussing, and watching pages now use APK-observed guest POST routes with basic paging | Interesting carousel payload, comments, collections, social menu, sponsorship, previous-page semantics and live response verification |
| Search | `fragment_search.xml`, `item_search.xml`, `SearchApi` | Partial; release query and result card grid | Search scopes, history, paging, suggestion rows, clear/empty/error, profile/channel/article search |
| Release details | `fragment_release.xml`, `release_*.xml`, `ReleaseFragment` | Partial; card details and episode selectors | Hero layout, vote/status actions, genres, screenshots, related/recommended, comments, banners, collections |
| Episodes and provider selection | `fragment_episodes.xml`, `EpisodesFragment` | Partial; type/source selection and episode list | APK chips, sort, workers, update history, player choice, quality dialog, provider-specific flows |
| Schedule | `fragment_schedule.xml`, `ScheduleFragment` | GET `schedule` service and weekday tabs render the seven response lists, with empty/error states | Live response verification, APK date/calendar behavior and exact visual comparison |
| Filter and filtered results | `fragment_filter.xml`, `fragment_filtered.xml`, `FilterFragment` | POST `filter/{page}` DTO supports the full APK request shape; UI currently sends year range and verified sort enum | Dynamic catalogs and the remaining filter controls (genre, category, status, type, country, studio, age, episodes/duration), result paging and live response verification |
| Bookmarks and history | `fragment_bookmark_tabs.xml`, `fragment_bookmarks.xml`, `BookmarksTabFragment` | HTML tab shell and guest/login state | Auth-backed lists, grid/list modes, sort/filter, status badges, context actions, paging |
| Feed | `fragment_feed_tabs.xml`, `fragment_feed_tab.xml`, feed item layouts | HTML shell with tabs, people row, composer, empty state | Feed data, posts, update prompts, post cards/actions, channels, subscriptions, empty/error states |
| Profile | `fragment_profile.xml`, profile header/content item layouts | HTML profile and statistics skeleton | Guest/auth profiles, friends, chart, history, tabs, badges, comments, roles, videos, health |
| Authentication | `fragment_sign_in*.xml`, `fragment_sign_up*.xml`, restore/verify layouts | HTML flows for sign-in, sign-up, restore, verify, provider buttons | Wire API, official OAuth, validation, session lifecycle and Windows Credential Manager |
| Settings and notifications | preference fragments, `fragment_notifications.xml` | HTML settings index/details and notification empty page | Extract all preference keys/options; connect persistence, notification API and account settings |
| Channels and social | channel fragments, `profile_social.xml`, `ChannelApi` | Pending | Channel list/profile/subscribers, editor, blocks/mutes/permissions and social navigation |
| Collections | collection fragments and editor | HTML tabs/skeleton | Collection API, detail/editor, release grid, comments/favorites and permission states |
| Articles and comments | article/editor/preview/reposts/suggestions/votes and comment fragments | Pending | Article cards/editor/preview, comments/replies/votes, reports, paging and auth gates |
| Reports and moderation | `fragment_report.xml`, enforcement and health fragments | Pending | Forms, reasons, evidence, appeal details and server-backed state |
| Player and provider dialogs | `activity_player.xml`, `activity_web_player.xml`, quality/player/downloader dialogs | Existing Kodik iframe handoff only | Reproduce allowed player controls/provider choices only after supported end-to-end playback is available |

## UI implementation rules

- Rebuild layout in local HTML/CSS/JS; do not copy Android runtime code or Android-only layouts into the Windows app.
- Keep screen modules split by feature and shared components in `web/components/`.
- Use APK dimensions/colors/strings and vector geometry as references. Do not package copyrighted APK artwork without distribution rights.
- Wire navigation and interactions only when their behavior is understood. A visual control without its API is labeled unavailable and never reports a successful server action.
- Compare each screen to APK 10.5 at a matching viewport after the WebView2 launch problem is resolved.

## Current slice summary

The public-data slice now includes guest schedule, three paged discover views, and a year/sort filter request in addition to random release, details, episodes, and search. API shapes and parameter names are APK-indicated; this environment cannot reach the service because its configured proxy rejects connections at `127.0.0.1:9`, so these new server paths are not marked live-verified. Interactive visual verification has also previously been blocked by WebView2 initialization error `0x8000FFFF` on this machine.
