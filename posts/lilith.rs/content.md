So I purchased a new domain, which of course meant that I needed something to sit on the new domain.
I could have just slapped my old webserver from [kitten.rs](https://kitten.rs) onto this domain and 
called it an evening but that web server was rotten. The site was only intended to be used for simple 
static pages but my needs began to evolve and suddenly routing went from ~40 lines of code to a 
~200 line match statement that had ***every*** path hard coded except for the projects that were read 
from a file from disk. That wasn't acceptable and even trying to change that code made me weap, so 
I decided that I should make a new project that does things in a more correct manner than last time.
If you are interrested in reading through my discussion of [Kitten.rs](https://kitten.rs) then 
the [Kitten.rs Post-mortem](/projects/Kitten.rs) post will most likely interest you.

# The Design

Yeah, I didn't change much about the design from the original. I could say how I liked the design 
too much, or that the design was recognisable, but I just get skill issued when it comes to designing 
things. I can quickly determine how systems interact with eachother but I get my arse kicked whenever 
I need to make something that is slightly visually appealing. Not to say that there aren't any 
improvements though, I managed to fix the site's mobile access problems; now the site: fully scales 
text correctly, has a mobile-optimised navbar, and the footer actually stays at the bottom of the page!
Firefox reader view still registers the page, which was completely unintended in the original site but 
it's really nice that it still works.

# What's Different?

## MDRS

So what did I do differently this time? Well, a little 19 hour sidequest called "mdrs" was basically 
this site but instead of hosting projects and blog posts, it hosted download links. I don't think I 
can say what they linked to but the systems are effectively the same as what are implemented here 
but with different data being displayed. From this I had developed an interresting system that 
involved using .YAML files to hold the information about a project/blog post - effectively recreating 
the .TOML system of Kitten.rs - and storing the data from the .YAML into a sqlite3 database. 

## Sqlite

From this change in system, I can introduce: sorting, tags, and collections into the website. As of 
writing they aren't yet added but I'll no longer have to wait until a project is finished for me to 
publish a post, as my site can now indicate if projects are unfinished and what other posts are 
connected to that project. I also added a field called "hiatus_since" that allows me to indicate if 
a post relates to a project that is not currently being worked on and also how long that project has 
been on hiatus for. The hiatus system is mostly just to motivate me to pick up old projects since I 
occasionally come back and read old posts to see what I was doing a while back.

If you've ever looked at the projects folder on the 
[GitHub repo for Kitten.rs](https://github.com/Portablefire22/Kitten-rs/tree/main/projects) then 
you'll know that I have ***a lot*** of posts that just never get completed and sit there indefinitly.
I can't promise that won't happen now but the new site will allow me to mark posts as unfinished, 
hiding them from view unless the user selects to see them. This is mostly just so that my 
insane ramblings still get published somewhere and also so maybe someone can stalk my progress --
thats basically what I use GitHub following for anyway.

# So What About Kitten.rs?

The domain is going to stay - I enjoy the mental damage I deal whenever someone reads it for the first 
time - but the site is probably going to host projects that are in development. The domain will be 
running a fork of this site that re-directs to the new domain and any dev projects will be hosted 
at [dev.kitten.rs](https://dev.kitten.rs).

# What's Up With The Name?

If you're questioning that then you probably know me irl :) It's not that hard to figure out but 
let's setup a little scavenger hunt for those who haven't figured it out yet! There are 3 colours 
featured prominently on the site - not including hovers - and those three colours can be 
assembled into a five striped flag :D 
