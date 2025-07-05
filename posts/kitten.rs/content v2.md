Kitten.rs, my first deployed website, was a naive attempt to create a website without a 
web-framework like Flask, Ruby On Rails, or Django. Created with the intention of 
only hosting random posts about projects I've done, the code has basically zero thought 
put into it and the consequences haunted me whenever I wanted to add a new post. 

Every project I pursue has one major requirement, the project must teach me something. 
The approach in which Kitten.rs reached this requirement was the unique task of using as 
little direct dependencies as possible, which somehow meant no web frameworks for niceties 
like routing or header parsing. Adding said niceties was of "little" challenge but - as 
mentioned previously - zero thought went into my implementation and lead to problems such as:
a half-working assets directory, terrible user interface with posts, and being stuck entirely 
in Web 1.0.

# Simplicity - The Double Edged Sword

Simplicity is great until it isn't enough, then it becomes your shackles. 

The plan was simple, create a handler that holds project information and retrieve data from it 
when a page is requested. This approach is technically fine I guess, I can't really think of 
any major dealbreakers for this specific instance, but why aren't I just using an sqlite file 
to track everything?

I didn't need many routes so surely just one match block for eveything would be fine? I only 
need four static and four dynamic routes, should be pretty simple to add without needing to 
invest in a full system.

I haven't made any web-compatible projects so I didn't need support for embedding content 
into a page.

Three design considerations. Three choices were enough to keep the project "simple" whilst 
directing me to a path of unmaintainability. Project handler was fine I guess, I'm not hosting 
multiple instances on the same machine so I don't need to share data, but routing and a lack 
of proper support for embedding projects just killed the website for me. I still don't have 
any web-compatible projects to showcase but I have a few that are WIP, all of which could be 
shown off in their unfinished state; I also don't require that many routes but adding new 
content (outside of posts) was a major pain point that needs to be rectified. 

# Projects Vs Blogs

I made the mistake of calling blogs 'projects' and never thought to change the naming until 
the start of the second website. In my defense, the projects section was actually meant to 
contain working projects and not just blog posts, but I'm dumb and never actually implemented 
functionality for that :) 

# What Should Have Been 

For this section we are going to assume that the initial conditions are still in place, no 
web frameworks and allow for dynamic content that I have uploaded. The good news is that 
the second condition can work the same as it always has, read from a file and have some way 
of converting that file's content to HTML. The bad news is that routing would need to be 
entirely reworked. 

## Routing 

I'm actually kind of disappointed that I never thought of this to begin with, it seems quite 
simple now that I actually thought about fixing this problem for one second. If I were to create 
a HashMap then I could store functions that return a result containing Ok(page content) or 
Err(http status), this map could be indexed with &str and now we have a hashmap that allows for 
route matching without a 200 line conditional statement. Hashmap.get() returns an Option so 
404's could be handled by just checking for None, and routes could be added with inserts. The 
code would probably be verbose but I'm certain some wizardry with macros (or even just processing 
a Vector of some struct) could simplify the appearance to that of Rocket.rs' routing implementation. 
Error pages could be implemented with a simple Err() check and constructing a generic page from the 
error information. 

It actually pains me quite a bit that I went with a giant match block instead of thinking of this 
solution originally, but I guess our code is created by learning from the mistakes of the past.

## Searching

As a consequence of routing, searching was not simple to introduce. It would have required 
being able to separate url parameters, which was not implemented in any form, and likely 
would have just required a full rewrite of routing. The lack of searching might seem fine now, 
but trying to find one specific post in a sea of posts could easily become a pain for the user 
and I want to make a website that isn't awful to use.

Unfinished posts were an annoyance on the developer side of things as a consequence of lacking 
search capabilities. With the lack of any sorting or filtering, unfinished posts just had to be 
completely culled from production and left to rot in their unfinished state in the Git repo. I 
like having a peak behind the curtain - and the unfinished posts were visible on the GitHub repo - 
so I thought that the website should have a way of showing unfinished posts to those who wish to 
read them. Sure they're crude but thats the fun of them, I just enjoy seeing the thought process 
being so visible in an unfinished product and maybe someone else shares that thought with me.

# Separating Blogs and Projects

The section 'Projects Vs Blogs' touched on this so I'm just going keep it simple. Starting from now, 
'projects' will be used to refer to finished pieces of work that do not have discussion attached 
to them. All project pages will just be the working project, plus any relevant information like 
credits or build instructions, whilst blogs will contain the discussions that were previously found 
on the projects tab. Any project with a relevant blog will have a direct link to the post, or 
collection of posts if applicable, and any blog will have relevant project links featured prominently 
on the page. 

The great part of adding a system like this is that I'm no longer constrained to singular posts about 
a project. Before I kept project blogs to a single post as that was just easier, but now I can 
sort posts into collections about one project. I can split a blog post without having to worrying 
about going back and fixing links everytime I make an update to a project, since now external project 
links will be featured on a singular dedicated project page.

# So What Now?

[Kitten.rs](https://Kitten.rs) is no more, for now. As of the time of writing you probably 
found this website by either: finding my GitHub repo, or navigated to Kitten.rs, and thats 
because Kitten.rs is going to host random projects. I have two VPS' now, and I'm going to be 
using Kitten.rs for development projects that require a server connection. The website is going 
to be replaced with a fork of [Lilith.rs](https://github.com/Portablefire22/lilith-rs) that 
redirects to this site when navigating to any non-index pages. 
