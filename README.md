# embedded-ui

This library is created for personal use, and needs some rework to make it universally-usable.
But I want to share it with everyone who is interested in creating complex UI (maybe not so complex) for embedded systems.

__One important thing to note, is that mostly all core functionality is a copy of [iced-rs](https://github.com/iced-rs/iced) as developing UI framework in Rust was a hard thing for me.__
But don't compare __embedded-ui__ to iced in any way, it's just better to study this library independently

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
- Of course its __very__-WIP state
- It is somewhat untested 😢
- You may find a lot of overhead in computations and memory usage which is unacceptable in embedded systems
- Only monospaced fonts are supported for now

## Icons

To design icons I used free online app called Piskel for pixel-art. It supports exporting as C header file which you can put into repo as `icons-input.c` and run `node make-icons.js` Node.JS script to convert this header file into `src/icons` directory. All you need is to give a name to each icon and its corresponding method.
For convenience, I've added a painting of each icon on top of it in generated file, so you can see what it looks like.

My icon packs are not consistent as catalogs because with such small sizes it is impossible to depict some figures.
So bigger the size of the icons wider the catalog. It also means that when picking an icon and don't know the exact limits, it is possible to use a generic icon name and the best size will be chosen automatically.


There's a special trait `InternalIconSet` which is implemented for all icon packs (and you can implement for yours) which contains icons used in system widgets, such as arrows for `Select` or check sign in `Checkbox`.

## Length

Length is a universal single-dimension size type, used both for width and height. 
There are four options in `Length` enum:
- `Fixed` - fixed length in pixels
- `Shrink` - occupies as least free space as possible giving more space for other elements
- `Fill` - occupies as much free space as possible
- `Div(N)` - takes `free / N` space. For example, in 100px container in width, element with width of `Div(4)` will take 25px.

## Flex layout

There's a single container using flex layout -- 'Linear'. It is a generic element for both row and column.

Flex
