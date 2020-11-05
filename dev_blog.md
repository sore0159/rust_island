_20/11/05_
Added a working confirm check state!  State transitions seem to work fine!  Had to tweak the "text" tool to imprint rather than add styles, but I think that's a better default.  We'll see if I ever really need to use additive writing.

Soon up is adding in a 3d context window and associated tools to the Canvas.  I'm tempted to use glium because I've used it in the past, but I'm worried it won't work well once I get into it (similar to termion).  I'm tempted to try to use gfx-hal or whatever, but that will involve going through a new tutorial and toolkit.

I don't think I want anything fancy with graphics, just drawing 2d shapes from tesselated paths.  Maybe some fun with shaders, not sure.  If glium works, then I think it's likely to cover everything I want to do.

Also soon todo is actually creating some game data and an actual prototype UI to interact with that data.  I guess the next step there is getting Serde hooked up to the game data structures I have now.

_20/11/01_
Switch from termion library to crossterm library a little painful, but seems to be complete.  How distant to keep the terminal library from the terminal UI code is something I'm not sure about, but I decided to let it seep in a bit.  Not sure about the current Style methodology, but it seems to work for now.  I like the terminal resizing and the ability to not crash while reading key events.

No forward progress but at least caught up to where I was before the library change.  Adding in anyhow for Result types is nice; not really throwing interesting errors at present but might add in the thiserror crate if I doc


_20/10/30_ 
UI abstraction switched over to State abstraction pretty painlessly.  Suprising pain point in the use of termion's async_stdin to read raw mode key presses: after some finagling I think I have something that gives me key presses, though it doesn't register ESC and sometimes seems to think the arrow keys are the characters ABCD.  Hard to say how often and if this will be a problem.  Considering looking at a different terminal libray if so.

Next step is extending the mockup to a state that allows you to make a selection!

_20/10/28_
State abstraction built, first pass.  Several arbitrary judgement calls made during this construction.  I expect to have to adjust a lot of things when I try to actually use it.

I don't know how persistance will work with the StateStack: I read Serde handles Box Dyns but I don't know?  Probably have the game data be serializable and then just have my own procedure for rebuilding an approprate StateStack when loading the data.

_20/10/27_
Current task: Creating a "State" system.  Not sure how to integrate this abstraction with my current UI abstraction: taking the basic template wholecloth from Amethyst has overlap with event handling.

I guess change UI to just be a thing which provides/writes a stream of events, and also provides a canvas (or multiple) to draw on?  Leaving the Event and Canvas just as generic and letting them be set by the UI implementation itself might work.

Note: Canvas abstraction would cover terminal output, 3d context window, and even audio systems.


