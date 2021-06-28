# area_computer

Using this for updates.

#### 6/23/21:

Currently it works by looping through several times and outputing what the estimated value of pi is along with
how many iterations it took. The iterations continue until a certain threshold of accuracy is hit and then stops.

Next up: have this vector <steps, pi value> be graphed. That's the ultimate goal.

#### 6/27/21:

Next set of goals:

have this output to a graph.
1) Steps vs accuracy
2) Steps vs time (how long each iteration takes)
3) Accuracy vs time

Also trying to figure out how to get this to complie on Linux. Probably going to require a linker/docker thing as the rustup commands do not seem to work. For that task however I'm going to learn that via a new Hello World program.

Other things to note:

1) how do floats work? Should we be using straight binary instead of a float? How about instead of a float we start over but use an "integrer" value of pi (so multiply the whole thing by 100 or 10000 or something, do the work that way, then shrink it down to a float.)?

How I'm going to do this:

hold pi as a vector, compute it with the vector, then compare to the original value.

This should be on a new branch we'll call VECTOR

#### 6/28/21:

Created vector branch and currently working on that.

How to handle pi? I want to compare this digit to digit, so what do I do?