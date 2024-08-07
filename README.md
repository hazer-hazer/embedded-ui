# embedded-ui

This library is created for personal use, and needs some rework to make it universally-usable.
But I want to share it with everyone who is interested in creating complex UI (maybe not so complex) for embedded systems.

**One important thing to note, is that mostly all core functionality is a copy of [iced-rs](https://github.com/iced-rs/iced) as developing UI framework in Rust was a hard thing for me.**
But don't compare **embedded-ui** to iced in any way, it's just better to study this library independently

## Pros & Cons

The main disadvantages of this library come from that in fact it isn't embedded-ready because of:

- `core::any` usage for widget states eats performance
- `Box` is used, so you must have global heap allocator
- `Vec` - again, global heap allocator is required.

I am planning to make an attempt on rewriting the code to use `heapless` or even fixed-sized arrays, still keeping `Any` but using references instead of the `Box`es. I didn't start with it because it would be kinda painful, and I bet I'd never go to any with such requirements.

Pros:

- From the start it was developed to be used with 128x32 monochrome OLED display and yes, you can make UI even on such a small displays with it.
- Almost renderer-agnostic, but aimed towards `embedded_graphics::DrawTarget`
- Controls-agnostic. It means, you're able to control the whole UI just with a single encoder
- Color-independent
- Events are customizable
- I made pretty large collections of icons starting from size of 5x5 pixels!
- Auto-sizing and flex layouts are supported

Cons:

- Of course its **very**-WIP state
- It is somewhat untested 😢
- You may find a lot of overhead in computations and memory usage which is unacceptable in embedded systems
- Only monospaced fonts are supported for now

## Text

> In many cases, when we're creating buttons, knobs and other interactive widgets, we want text to be centered.
> And in case of embedded systems where users mostly not creating interfaces for text reading but with some small names for components, I think centered-by-default is a good choice, so keep this in mind.

## Icons

To design icons I used free online app called Piskel for pixel-art. It supports exporting as C header file which you can put into repo as `icons-input.c` and run `node make-icons.js` Node.JS script to convert this header file into `src/icons` directory. All you need is to give a name to each icon and its corresponding method.
For convenience, I've added a painting of each icon on top of it in generated file, so you can see what it looks like.

My icon packs are not consistent as catalogs because with such small sizes it is impossible to depict some figures.
So bigger the size of the icons wider the catalog. It also means that when picking an icon and don't know the exact limits, it is possible to use a generic icon name and the best size will be chosen automatically.

There's a special trait `InternalIconSet` which is implemented for all icon packs (and you can implement for yours) which contains icons used in system widgets, such as arrows for [`Select`](#select) or check sign in [`Checkbox`](#checkbox).

## Length

Length is a universal single-dimension size type, used both for width and height.
There are four options in `Length` enum:

- `Fixed` - fixed length in pixels
- `Shrink` - occupies as least free space as possible giving more space for other elements
- `Fill` - occupies as much free space as possible
- `Div(N)` - takes `free / N` space. For example, in 100px container in width, element with width of `Div(4)` will take 25px.

## Flex layout

There's a single container using flex layout -- [`Linear`](#linear). It is a generic element for both row and column.

Flex

## Component structure

As I said before, the core logic is taken from [iced-rs](https://github.com/iced-rs/iced), but may differ in some ways.
Each component consist of these parts:

- Layout: gives sizing of component and layout of its children
- Event handling: how component reacts to events such as inputs
- Drawing: the actual displaying of component
- State:

### Designing your own component

I see it's better to just walk through this checklist for `YourComponent` (required steps are marked with `x`):

- [ ] Does your component have state?:
  - [ ] Declare `YourComponentState` structure
  - It is common to define state `is_pressed: bool` if `YourComponent` needs to react to clicks.
- [ ] Needs your component different drawing depending on state?
  - [ ] Declare `YourComponentStatus`
  - Add `Normal` for "raw" `YourComponent` style, `Pressed`, `Focused` or anything else you need.
- [ ] Likely your component needs styles:
  - [ ] Declare style with `component_style` (read more in docs)
- [x] Now create `YourComponent` structure
  - [ ] Add `id: ElId` if your component is interactive
  - [ ] Add `size: Size<Length>` for responsive sizing of your component
  - [ ] Add event handler for user to define the interactions
    - Event handlers commonly have type `Box<dyn Fn(YourComponentValue)> -> Message + 'a`
  - [ ] Add `class: S::Class<'a>` to store styling
  - [ ] Add any other fields you need
- [x] Implement `Widget` trait for your component:
  - [x] `Widget::id` must return `id` if `YourComponent` has it
  - [x] `Widget::tree_ids` must return list of `YourComponent::id` and `tree_ids` of `YourComponent` children (if any)
  - [x] `Widget::size` usually returns `self.size`, rarely size is calculated, for example if `YourComponent` size is a square-sized and just `Length` is stored, so `Size::new_equal(self.size)` is returned.
  - [x] `Widget::state_tag` needs to be implemented, but it is unused for now.
  - [x] `Widget::state` returns `YourComponentState` wrapped in `State` or stateless state
  - [x] `Widget::state_children` returns a flattened tree of children states
  - [x] `Widget::on_event` needs implementation for interactive components:
    - [ ] If `YourComponent` works with common events (such as focusing)

## Built-in components

### `Checkbox`

### `FocusInput`

### `Graph`

### `Icon`

### `Image`

### `Knob`

> TODO:
>
> - Add child element to the center of the knob. For value as text and anything

### `Linear`

### `ProgressBar`

**TODO**

### `Radio`

> **Postponed**. Use [`Select`](#select)
> As I think `Select` covers radio button logic so `Radio` postponed.

### `Scroll`

### `Select`

### `Slider`

> TODO:
>
> - Slider text, to display real value. It should follow the knob not go out of viewport.
> - Add min/max if it makes sense

### `Table`

> **Postponed**. Use [`Linear`](#linear) with `row` and `col` macros.

### `Toggle`

> **Postponed**. Use [`Checkbox`](#checkbox).

## Roadmap

### Near plans

- [`Knob`](#knob)
- [`Graph`](#graph)
- [`ProgressBar`](#progressbar)
- [`Slider`](#slider)
- [`FocusInput`](#focusinput)
- [`Scroll`](#scroll)
- Basic pages and links

### Future

- Better navigation and links
- Transformations
- Add animations. This opens the door for not only animations, but new time-based logic too:
  - Animated transformations
  - Spinners, loaders and other such stuff
- Windows as popups, overlays
- More high-level widgets (most aren't usable for small displays):
  - Text inputs
  - Toolbar
  - Header/Footer (not sure if we need it)
  - Accordion
  - Badge and chip
  - Date/time picker
  - Dropdown
  - Tree (tree structure representation)
  - Roller (see [LVGL widget](https://docs.lvgl.io/master/widgets/roller.html))
- Complex special effects:
  - Image in background, border, etc.
  - Shadow (yeah, what about shadows on 128x32 monochrome display lol?)
  - Parallax
  - Transparency filters
