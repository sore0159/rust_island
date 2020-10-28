*20/10/28*
        State abstraction built, first pass.  Several arbitrary judgement calls made during this construction.  I expect to have to adjust a lot of things when I try to actually use it.
        I don't know how persistance will work with the StateStack: I read Serde handles Box Dyns but I don't know?  Probably have the game data be serializable and then just have my own procedure for rebuilding an approprate StateStack when loading the data.

*20/10/27*
        Current task: Creating a "State" system.  Not sure how to integrate this abstraction with my current UI abstraction: taking the basic template wholecloth from Amethyst has overlap with event handling.
        I guess change UI to just be a thing which provides/writes a stream of events, and also provides a canvas (or multiple) to draw on?  Leaving the Event and Canvas just as generic and letting them be set by the UI implementation itself might work.
        Note: Canvas abstraction would cover terminal output, 3d context window, and even audio systems.


